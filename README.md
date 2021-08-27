# ROSS

A minimal operating system written in rust by following [Philipp Oppermann's blog](https://os.phil-opp.com/)


### Build
To build this you need to be using the nightly toolchain and add the following to your config file for cargo
(usually found in `~/.cargo/config.toml`)
```toml
[unstable]
build-std-features = ["compiler-builtins-mem"]
build-std = ["core", "compiler_builtins"]
```
Optionally you can add,
```toml
[build]
target = "x86_64-ross.json"
```
to use the custom target by default however I prefer to add,
```toml
[alias]
bc = "build --target x86_64-ross.json"
```
instead. This allows us to use `cargo bc` to build this crate while `cargo build` functions as normal.
