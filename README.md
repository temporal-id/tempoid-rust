# Tempo ID

[![crate](https://img.shields.io/crates/v/tempoid.svg)](https://crates.io/crates/tempoid)
![ci](https://github.com/temporal-id/tempoid-rust/actions/workflows/ci.yml/badge.svg)
[![License: MIT](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue.svg)](#license)

Short IDs with temporal ordering.

A library to generate URL-friendly, unique, and short IDs that are sortable by time. Inspired by nanoid and UUIDv7.

See [tempoid.dev](https://tempoid.dev) for more information.

## Motivation

- **URL-friendly**: The IDs are easy to select and copy.
- **Unique**: The IDs are practically unique and can be used in distributed systems.
- **Short**: The IDs are shorter than UUIDs because they are encoded with a larger alphabet.
- **Sortable**: The IDs are sortable by time because a timestamp is encoded in the beginning of the ID.
- **Customizable**: You can configure the ID length and the characters used.

Example ID:

```text
0uoVxkjTFsrRX30O5B9fX
<------><----------->
  Time     Random
```

## Collisions

- **Same millisecond**: There can be only a collision if two IDs are generated in the same millisecond.
- **Low probability**: Even if two IDs are generated in the same millisecond, the probability of a collision is very low.

The 13 random characters exceed the randomness of UUIDv7 (≈10^23 vs ≈10^22).

## Getting Started

```text
# Cargo.toml
[dependencies]
tempoid = <version>
```

## Usage

```rust
use tempoid::{TempoId, alphabet};

fn main() {
    // generate a new ID
    let id = TempoId::generate();

    // parse an ID
    let id = TempoId::parse("0uoVxkjTFsrRX30O5B9fX");

    // convert an ID to a string
    let id = TempoId::generate();
    let string = id.to_string();

    // use a different alphabet
    let id = TempoId::generate_with_alphabet(alphabet::base64);

    // use a custom alphabet
    let id = TempoId::generate_with_alphabet("ABCDEF");
}
```

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
