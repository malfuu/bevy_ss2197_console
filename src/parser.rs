const COMMAND_SEPARATOR: char = ';';

// TODO: recheck these pubs
#[derive(Debug)]
pub struct RawCommand {
    pub command: String,
    pub tokens: Vec<String>,
}

impl RawCommand {
    fn parse(raw_command: &str) -> Self {
        let mut parts = raw_command.split_whitespace();

        let command = parts
            .next()
            .expect("parse called on empty command")
            .to_string();

        let tokens = parts.map(|part| part.to_string()).collect();

        RawCommand { command, tokens }
    }
}

pub(crate) fn parse_input(input: impl Into<String>) -> Vec<RawCommand> {
    let input = input.into();

    input
        .split(COMMAND_SEPARATOR)
        .filter(|cmd| !cmd.trim().is_empty())
        .map(RawCommand::parse)
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::parser::parse_input;

    #[test]
    fn empty_input() {
        let input = String::new();

        let parsed = parse_input(input);

        assert!(parsed.is_empty());
    }

    #[test]
    fn single_line() {
        let input = "hello 1 2 3".to_string();

        let parsed = parse_input(input);
        assert_eq!(parsed.len(), 1);

        let cmd = &parsed[0];
        assert_eq!(cmd.command, "hello");
        assert_eq!(cmd.tokens, vec!["1", "2", "3"]);
    }

    #[test]
    fn whitespace() {
        let input = "    foo    bar    ".to_string();

        let parsed = parse_input(input);
        assert_eq!(parsed.len(), 1);

        let cmd = &parsed[0];
        assert_eq!(cmd.command, "foo");
        assert_eq!(cmd.tokens, vec!["bar"]);
    }

    #[test]
    fn multiple_commands() {
        let input = "foo bar; abc xyz".to_string();

        let parsed = parse_input(input);
        assert_eq!(parsed.len(), 2);

        assert_eq!(parsed[0].command, "foo");
        assert_eq!(parsed[0].tokens, vec!["bar"]);

        assert_eq!(parsed[1].command, "abc");
        assert_eq!(parsed[1].tokens, vec!["xyz"]);
    }

    #[test]
    fn ignores_empty_commands_between_separators() {
        let input = "foo;;bar;   ;baz".to_string();

        let parsed = parse_input(input);
        assert_eq!(parsed.len(), 3);

        assert_eq!(parsed[0].command, "foo");
        assert_eq!(parsed[1].command, "bar");
        assert_eq!(parsed[2].command, "baz");
    }
}
