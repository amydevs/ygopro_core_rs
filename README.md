# ygopro_core_rs

This crate provides safe Rust bindings to [EDOPro's ygopro-core fork](https://github.com/edo9300/ygopro-core).

# Usage

This crate is not yet published, so please specify it as a git repository in `cargo.toml`.

```toml
[dependencies]
ygopro_core_rs = { git = "https://github.com/amydevs/ygopro_core_rs.git" }
```

# License

This crate, and the underlying EDOPro ygopro-core fork are both licensed under AGPL-3. The Lua library that this library uses is licensed under MIT.

# Examples

TBD... See [EDOPro](https://github.com/ProjectIgnis/EDOPro) itself for an idea on how the library is used.

# What About the Query/Message Buffers?

The de/serialization of those buffers are outside of the scope of default features of this crate. This is to be done at a later date, either as an optional feature or a separate create, depending on how suitable the data structures are for the Bevy Rust EDOPro client I'll eventually make.