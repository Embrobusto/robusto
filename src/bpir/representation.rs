/// Core intermerdiate representation. Serves the purpose of providing a
/// convenient representation enabling:
///
/// - parser generation;
/// - serializer generation with human-readable entities;
///
/// The idea is to make a stupid, straightforward solution with 1-to-1 mapping
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
/// Here's what its representation would look like:
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

    /// The protocol's root message
    Root,
}

/// Represents a protocol's message as a sequence of bytes
pub struct Message {
    name: std::string::String,
    fields: std::vec::Vec<Field>,
}

/// May be a regular field, such as byte sequence of fixed length, or u32, or a
/// payload (nested message))
pub struct Field {
    name: std::string::String,
    attributes: std::vec::Vec<FieldAttribute>,
}

/// Represents the entire protocol as a set of messages
pub struct Protocol {
    messages: std::vec::Vec<Message>,
}
