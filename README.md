# cargo-latest

A cargo subcommand for getting the latest versions of your packages.

# Demo

```sh
$ cargo latest rand serde anyhow
rand = "0.8.3"
serde = "1.0.125"
anyhow = "1.0.40"
```

You can paste the output directly into the `[dependencies]` section of your `Cargo.toml`.

# Install

```sh
$ cargo install cargo-latest
```
