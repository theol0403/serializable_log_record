//! # Serializable Log Record
//!
//! This crate provides a `SerializableLogRecord` struct which is a custom representation of the `log::Record` struct from the `log` crate.
//! The `log::Record` struct is not directly serializable due to the use of `fmt::Arguments`.
//! The `SerializableLogRecord` struct solves this problem by providing a serializable version of `log::Record`.
//!
//! ## Usage
//!
//! Convert a `log::Record` to a `SerializableLogRecord` using the `::from` method:
//!
//! ```rust
//! # use log::{Record, Level};
//! use serializable_log_record::SerializableLogRecord;
//!
//! # let record = Record::builder()
//! #     .args(format_args!("Hello"))
//! #     .level(Level::Info)
//! #     .target("my_target")
//! #     .file(Some("lib.rs"))
//! #     .line(Some(10))
//! #     .build();
//! //let record: log::Record = ...;
//! let serializable_record = SerializableLogRecord::from(&record);
//! ```
//! `Serde`'s `Serialize` and `Deserialize` traits are implemented for `SerializableLogRecord` if the `serde` feature is enabled.
//! The feature `bincode2` is also available which implements `bincode::Encode` and `bincode::Decode` from bincode version 2 for `SerializableLogRecord`.
//!
//! To convert a `SerializableLogRecord` back into a `log::Record` use the `into_log_record` macro. The result of this macro has to be passed
//! directly into a call to the `log` method of any `log::Log` implementation. It cannot be stored in an intermediate variable or alike due to
//! the extremely restrictive lifetime of the `args` field of `log::Record`.
//!
//! ```rust
//! # use log::Level;
//! # use serializable_log_record::SerializableLogRecord;
//! #
//! # let record = log::Record::builder()
//! #     .args(format_args!("Hello"))
//! #     .level(Level::Info)
//! #     .target("my_target")
//! #     .file(Some("lib.rs"))
//! #     .line(Some(10))
//! #     .build();
//! #
//! # let serializable_record = SerializableLogRecord::from(&record);
//! #
//! # let any_logger = log::logger();
//! let mut builder = log::Record::builder();
//! any_logger.log(&serializable_log_record::into_log_record!(builder, serializable_record));
//! ```
//!

use std::str::FromStr;

use log::{Level, Record};

/// A custom representation of the `log::Record` struct which is unfortunately
/// not directly serializable (due to the use of `fmt::Arguments`).
///
/// Use `::from` to convert a `log::Record` to a `SerializedRecord`.
///
/// The use of `::into` is unfortunately not possible. This is why the
/// `log_into_record` macro is provided. Use it directly in a function call to
/// convert a `SerializedRecord` into a `log::Record`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bincode2", derive(bincode::Encode, bincode::Decode))]
#[non_exhaustive]
pub struct SerializableLogRecord {
    pub level: String,
    pub args: String,
    pub target: String,
    pub module_path: Option<String>,
    pub file: Option<String>,
    pub line: Option<u32>,
}

impl SerializableLogRecord {
    /// Create a new `SerializableLogRecord` from the given arguments.
    /// Use `::from` to directly convert a `log::Record` to a `SerializableLogRecord`.
    #[allow(clippy::must_use_candidate)]
    pub fn new(
        level: Level,
        args: String,
        target: String,
        module_path: Option<String>,
        file: Option<String>,
        line: Option<u32>,
    ) -> Self {
        Self {
            level: level.as_str().to_owned(),
            args,
            target,
            module_path,
            file,
            line,
        }
    }

    /// Internal macro use only.
    #[allow(clippy::must_use_candidate)]
    #[doc(hidden)]
    pub fn string_to_level(level: &str) -> Level {
        Level::from_str(level).unwrap_or(Level::Warn)
    }
}

impl<'a> From<&Record<'a>> for SerializableLogRecord {
    /// Convert a `log::Record` to a `SerializableLogRecord`.
    fn from(record: &Record<'a>) -> Self {
        Self::new(
            record.level(),
            record.args().to_string(),
            record.target().to_owned(),
            record.module_path().map(str::to_owned),
            record.file().map(str::to_owned),
            record.line(),
        )
    }
}

impl<'a> From<Record<'a>> for SerializableLogRecord {
    /// Convert a `log::Record` to a `SerializableLogRecord`.
    fn from(value: Record<'a>) -> Self {
        Self::from(&value)
    }
}

#[macro_export]
/// This macro converts a `SerializableLogRecord` into a `log::Record` which is to be passed
/// immediately into a call to the `log` method of any `log::Log` implementation.
macro_rules! into_log_record {
    ($builder:expr, $message:expr) => {
        $builder
            .level(SerializableLogRecord::string_to_level(&$message.level))
            .args(format_args!("{}", $message.args))
            .target($message.target.as_str())
            .module_path($message.module_path.as_deref())
            .file($message.file.as_deref())
            .line($message.line)
            .build()
    };
}
