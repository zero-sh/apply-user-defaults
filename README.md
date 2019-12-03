[![Crates.io](https://img.shields.io/crates/v/apply-user-defaults.svg)](https://crates.io/crates/apply-user-defaults)

# apply-user-defaults

`apply-user-defaults` is a small utility to set macOS user defaults
declaratively from a YAML file.

## Usage

To use, simply structure a YAML file like the following:

```yaml
com.apple.dock:
  # System Preferences > Dock > Automatically hide and show the Dock.
  autohide: true

  # System Preferences > Dock > Minimize windows using: Scale effect.
  mineffect: "scale"

  # System Preferences > Dock > Show indicators for open applications.
  show-process-indicators: false

  # System Preferences > Dock > Size.
  tilesize: 72
```

Then apply it using:

```sh
$ apply-user-defaults path-to-file.yaml
```

You can also see what commands are being run by enabling verbose output:

```sh
$ apply-user-defaults path-to-file.yaml --verbose
==> defaults write com.apple.dock autohide -bool true
==> defaults write com.apple.dock mineffect -string scale
==> defaults write com.apple.dock show-process-indicators -bool false
==> defaults write com.apple.dock tilesize -int 72
Success! Applied defaults.
```

## Installation

`apply-user-defaults` can be installed with cargo:

```sh
$ cargo install apply-user-defaults
```

To build from source:

```sh
git clone https://github.com/zero-sh/apply-user-defaults.git
cd apply-user-defaults
cargo run -- path-to-file.yml --verbose
```

## License

This project is licensed under either the [Apache-2.0](LICENSE-APACHE) or
[MIT](LICENSE-MIT) license, at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
