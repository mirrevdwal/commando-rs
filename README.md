# Commando

A terminal-based menu based on Skim to launch custom commands.

## Installation

### Arch Linux

Installing can be done by cloning the project and running `makepkg -i` on the `PKGBUILD` provided in `contrib/`.

### Other

For other distributions, the package can be built from source by running `cargo build --release`.
After this, the binary can be found at `target/release/commando`.

## Configuration

Configuration for Commando can be found in `$XDG_CONFIG_HOME/commando/`.

Commands can be defined in Commando toml-files. These can be placed in `$XDG_CONFIG_HOME/commando/commandos/`, or any of your additional `$XDG_CONFIG_DIRS/commando/commandos/`.
Below you find an example of such a toml-file.

```
[[commands]]
name: "Power off"
command: "systemctl poweroff"

[[commands]]
name: "Suspend"
command: "systemctl suspend"
```

## Contributing

You can contribute to this project by filing an [issue](https://github.com/mirrevdwal/commando-rs/issues) or opening a [pull request](https://github.com/mirrevdwal/commando-rs/pulls).

## Licensing

Licensed under either of [Apache Licence](LICENSE-APACHE), Version 2.0 or [MIT License](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in Commando by you,
as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
