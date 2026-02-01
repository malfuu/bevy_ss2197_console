use proc_macro::TokenStream;
use quote::quote;
use syn::{DataStruct, DeriveInput, LitStr, parse_macro_input, spanned::Spanned};

struct FieldArgument {
    ident: syn::Ident,
    ty: syn::Type,
}

struct CommandModel {
    /// The specific struct name (e.g., Foo)
    ident: syn::Ident,
    /// The string name for the console (e.g., "foo")
    name: String,
    /// The fields we need to parse
    fields: Vec<FieldArgument>,
}

fn analyze_struct(input: &DeriveInput, data: &DataStruct) -> syn::Result<CommandModel> {
    let name = get_command_name(input);

    let mut fields = Vec::new();

    for field in &data.fields {
        let ident = field.ident.clone().ok_or_else(|| {
            syn::Error::new(field.span(), "Only named fields work with ConsoleCommand.")
        })?;

        fields.push(FieldArgument {
            ident,
            ty: field.ty.clone(),
        });
    }

    Ok(CommandModel {
        ident: input.ident.clone(),
        name,
        fields,
    })
}

fn get_command_name(input: &DeriveInput) -> String {
    for attr in &input.attrs {
        if attr.path().is_ident("console") {
            let mut name_found = None;

            let _ = attr.parse_nested_meta(|meta| {
                if !meta.path.is_ident("name") {
                    return Ok(());
                }

                let content = meta.value()?;
                let lit = content.parse::<LitStr>()?;

                name_found = Some(lit.value());
                Ok(())
            });

            if let Some(name) = name_found {
                return name;
            }
        }
    }

    input.ident.to_string().to_lowercase()
}

fn generate_impl(model: CommandModel) -> proc_macro2::TokenStream {
    let struct_ident = &model.ident;
    let command_name = &model.name;

    let field_parsing = model.fields.iter().map(|f| {
        let fname = &f.ident;
        let ftype = &f.ty;
        quote! {
            let #fname = fields.next()
                .ok_or(BuildError::MissingArgument)?
                .parse::<#ftype>()
                .map_err(|_| BuildError::ParseError)?;
        }
    });

    let field_names = model.fields.iter().map(|f| &f.ident);

    quote! {
        impl ConsoleCommand for #struct_ident {
            fn name() -> &'static str {
                #command_name
            }

            fn build(command: RawCommand) -> Result<Box<Self>, BuildError> {
                let mut fields = command.tokens.into_iter();

                #(#field_parsing)*

                Ok(Box::new(Self {
                    #(#field_names),*
                }))
            }

            fn trigger(self, commands: &mut Commands) {
                commands.trigger(self);
            }
        }
    }
}

#[proc_macro_derive(ConsoleCommand, attributes(console))]
pub fn derive_console_command(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let result = match &input.data {
        syn::Data::Struct(data_struct) => analyze_struct(&input, data_struct).map(generate_impl),
        syn::Data::Enum(_) => Err(syn::Error::new(
            input.span(),
            "ConsoleCommand not implemented for Enums",
        )),
        syn::Data::Union(_) => Err(syn::Error::new(
            input.span(),
            "ConsoleCommand only supports structs and enums.",
        )),
    };

    match result {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
