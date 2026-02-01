# Bevy Space Console
## Half-Life like console for use in Bevy Projects

> It really is not in a good place for testing experimenting right now.

This is part of the ```Space Station 2197``` project, but is suitable for other application aswell!

## Quickstart

For more in-depth documentation, please check the files.


## Features

- [ ] Full command parsing
    - [x] Named field structs
    - [ ] Unnamed field structs
    - [ ] Enums
- [ ] Command history
- [ ] Command suggestion
    - [ ] Parameter suggestion
- [ ] Frontends
    - [x] bevy_egui console
        - [ ] Customization
        - [ ] ANSI colors
    - [ ] Bevy UI console
    - [ ] CLI console

## Crate Features

| Feature | Description |
|--------|-------------|
| **cli** (not implemented) | Enables `CliFrontendPlugin` |
| bevy_egui | Enables `EguiFrontendPlugin` |

## Tracing Capture
If you wish to capture info, warn and error logs from tracing, you will need to add the plugin's layer to Bevy's ```LogPlugin```.

```rust
// TODO: add it later
```

## Examples

Egui frontend:
```bash
cargo run --example egui --features bevy_egui
```

CLI frontend:
```bash
cargo run --example cli --features cli
```

## Alternatives

| Project | Description |
|--------|-------------|
| [bevy_console](https://github.com/makspll/bevy-console) | More mature solution with `clap` parsing. |
| [bevy_minibuffer](https://github.com/shanecelis/bevy_minibuffer) | WIP Text-Editor like developer console. |

## Bevy compatibility

| bevy   | bevy_space_console |
| ------ | ------------------ |
| 0.17.3 | 0.0.1              |

## License

## Contribution
# bevy_ss2197_console
