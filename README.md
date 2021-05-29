# Drone Build Notifier

Utility for reporting Drone build status with desktop notification.

## Usage

Currently we don't provide prebuilt binaries, so you must install Rust and Cargo.
Then, compile this with `cargo build --release`.

Also, don't forget to prepare two icon files: one is `success.png` and the other one is `failure.png`.
Put the binary and icon files at the same directory.

## Configuration

We've provided an example configuration file.
You just need to copy it as `config.toml` and modify it as you want.

## Credits

The two icon files in this repository are from [Gradient UI icon set](https://www.iconfinder.com/iconsets/gradient-ui-1)
which is licensed under [CC BY-SA 3.0](https://creativecommons.org/licenses/by-sa/3.0/).

## License

MIT License

Copyright (c) 2021-present Pig Fang
