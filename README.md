# cargo-strip

[![crates.io badge](https://img.shields.io/crates/v/cargo-strip.svg)](https://crates.io/crates/cargo-strip)
![CI](https://github.com/guedou/cargo-strip/workflows/CI/badge.svg)
[![Twitter Follow](https://img.shields.io/twitter/follow/guedou.svg?style=social)](https://twitter.com/intent/follow?screen_name=guedou)

As of Rust 1.59, the `cargo` command is now able to [strip a binary](https://doc.rust-lang.org/beta/cargo/reference/profiles.html#strip). This can be enabled in your `Cargo.toml` in the following way:
```
[package]
# ...

[profile.release]
strip="debuginfo"
```

---

A cargo subcommand that reduces the size of Rust binaries using the `strip` command.

## Installation & Usage

Run the following command:
```
cargo install --force cargo-strip
```

Simply strip your binary with:
```
cargo strip
```

When cross-compiling, the `--target` could be used to strip the binary, such as:
```
cargo strip --target armv7-unknown-linux-gnueabihf
```
