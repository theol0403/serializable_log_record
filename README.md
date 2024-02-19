# serializable_log_record

[![Crates.io](https://img.shields.io/crates/v/serializable_log_record.svg)](https://crates.io/crates/serializable_log_record)
[![Docs](https://docs.rs/serializable_log_record/badge.svg)](https://docs.rs/serializable_log_record)
[![MIT/APACHE-2.0](https://img.shields.io/crates/l/serializable_log_record.svg)](https://crates.io/crates/serializable_log_record)

The `log::Record` struct is not serializable by default. This crate provides a serializable version of the `log::Record` struct. But most importantly, it
provides a macro `into_log_record` which can convert the serialized record back into a regular record. This is not as straightforward as it sounds since
the `log::Record` struct uses `fmt::Arguments` in one of its fields which is not serializable due to its extremely strict lifetime.

This crate is a central helper crate for the [parallel_logger](https://crates.io/crates/parallel_logger) crate but can be used independently.

## Usage

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
serializable_log_record = "0.3"
```

How to use in your application:
```rust
let record: log::Record = ...get the record from a logger or build it manually...;
let serializable_record = SerializableLogRecord::from(&record);
```

If you enable the `serde` feature, the `SerializableLogRecord` struct implements the `Serialize` and `Deserialize` traits.<BR>
If you enable the `bincode2` feature, the `SerializableLogRecord` struct implements the `Encode` and `Decode` traits for bincode 2.

In order to convert the `SerializableLogRecord` back into a `log::Record` you can use the `into_log_record` macro:
```rust
let serializable_record = SerializableLogRecord::from(&record);
let mut builder = log::Record::builder();
logger.log(&SerializableLogRecord::into_log_record!(builder, serializable_record));
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

