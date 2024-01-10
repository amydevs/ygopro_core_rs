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

```rust
use ygopro_core_rs::DuelBuilder;

fn main() {
    let duel_builder = DuelBuilder::default();
    let duel = duel_builder.build();
    duel.start();
    duel.process();
    println!("{:?}", duel.get_message());
}
```

See [EDOPro](https://github.com/ProjectIgnis/EDOPro) itself for more of an idea on how the library is used.

Or the tests found in [`./src/duel.rs`](./src/duel.rs).

# What About the Query/Message Buffers?

The de/serialization of those buffers are outside of the scope of default features of this crate. This is to be done at a later date, either as an optional feature or a separate create, depending on how suitable the data structures are for the Bevy Rust EDOPro client I'll eventually make. For now, common message constants can be found under `ygopro_core_rs::common`.