[![Build Status](https://travis-ci.org/zero-sh/apply-user-defaults.svg?branch=master)](https://travis-ci.org/zero-sh/apply-user-defaults)
![License](https://img.shields.io/crates/l/apply-user-defaults.svg)
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

## Template Expansion

Environment variables can also be included using shell parameter expansion
syntax. For example:

```yaml
com.apple.finder:
  # Finder > Preferences > New Finder windows show > Home directory.
  NewWindowTargetPath: "file://${HOME}"
```

will evaluate to: 

```sh
defaults write com.apple.finder NewWindowTargetPath -string "file://$HOME"
```

where `$HOME` is the value contained in the `HOME` environment variable.

This only applies when the string in the YAML file begins with a dollar sign and
is wrapped in braces (just using `$HOME` won't work).

To disable, you may pass the flag `--no-env` or escape the dollar sign, e.g.
`'\\${VALUE}'`.

## Installation

Pre-compiled binaries are available on the [releases
page](https://github.com/zero-sh/apply-user-defaults/releases).

### Homebrew

If you're using Homebrew, you can install with a custom tap:

```sh
$ brew install zero-sh/tap/apply-user-defaults
```

### Cargo Install

To install via Cargo, run:

```sh
$ cargo install apply-user-defaults
```

### Building from Source

To build from source:

```sh
$ git clone https://github.com/zero-sh/apply-user-defaults.git
$ cd apply-user-defaults
$ cargo run -- path-to-file.yml --verbose
```

## License

This project is licensed under either the [Apache-2.0](LICENSE-APACHE) or
[MIT](LICENSE-MIT) license, at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
