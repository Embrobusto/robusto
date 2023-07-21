/// Core intermerdiate representation. Serves the purpose of providing a
/// convenient representation enabling:
///
/// - parser generation;
/// - serializer generation with human-readable entities;
///
/// The idea is to be stupid, straightforward solution with 1-to-1 mapping
/// from user entities to parser/serializer backend, and vice versa.
///
/// Example:
///
/// Consider the message of the following structure:
///
/// struct SimpleUserMessage {
///	    sync: u8 = 0xFE,
///     payload: u8[4],
///     checksum_crc32: u32,
/// }
///
/// Here's how its representation would look like:
///
/// ```rust
/// // `SimpleUserMessage` message
/// let bpir = Message {
///     name: "SimpleUserMessage",
///     fields: vec![
///         // `sync`
///         Field {
///             name: std::string::String::new("sync"),
///             attributes: vec![
///                 FieldAttribute::ExpectConstValue(vec![0xfe as u8]),
///             ]
///         },
///         // `payload`
///         Field {
///             name: std::string::String::new("payload"),
///             attributes: vec![
///                 FieldAttribute::ExpectLength(4u),
///             ]
///         },
///         Field {
///             name: std::string::String::new("checksum"),
///             attributes: vec![
///                 FieldAttribute::Checksum(std::string::String::new("crc32")),
///                 FieldAttribute::ExpectLength(4),
///             ]
///         },
///     ],
/// }
/// ```
///

pub use std;

/// Every field is modified with a set of attributes, such as
/// - length (if the field is of constant length);
/// - accepted values;
/// - hooks (for calculating checksums),
/// etc.
pub enum FieldAttribute {
    /// Expected length
    ExpectLength(usize),

    /// Expect a certain sequence of bytes
    ExpectConstSequence(std::vec::Vec<u8>),

    /// Interpret the payload as a message
    InterpretAsMessageById,

    /// From the first byte of this field, checksum calculation (such as CRC32)
    /// calculation is started
    StartChecksum{algorithm: std::string::String},

    /// At the last byte of this field, checksum calculation (such as CRC32)
    /// will be stopped
    StopChecksum,

    Checksum{algorithm: std::string::String},
}

pub struct Message {
    name: std::string::String,
    fields: std::vec::Vec<Field>,
}

/// Regular field, such as byte sequence of fixed length, or u32
pub struct Field {
    name: std::string::String,
    attributes: std::vec::Vec<FieldAttribute>,
}
