//! Core intermerdiate representation. Serves the purpose of providing a
//! convenient representation enabling:
//!
//! - parser generation;
//! - serializer generation with human-readable entities;
//!
//! The idea is to make a stupid, straightforward solution with 1-to-1 mapping
//! from user entities to parser/serializer backend, and vice versa.
//!
//! Example:
//!
//! Consider the message of the following structure:
//!
//! ```
//! struct SimpleUserMessage {
//!	    sync: u8 = 0xFE,
//!     payload: u8[4],
//!     checksum_crc32: u32,
//! }
//! ```
//!
//!
//! Here's what its representation would look like:
//!
//! ```rust
//! // `SimpleUserMessage` message
//! use robusto::bpir::representation::Message;
//! use robusto::bpir::representation::Field;
//! use robusto::bpir::representation::FieldAttribute;
//! let bpir = Message {
//!     name: std::string::String::from("SimpleUserMessage"),
//!     fields: vec![
//!         // `sync`
//!         Field {
//!             name: std::string::String::from("sync"),
//!             attributes: vec![
//!                 FieldAttribute::ConstSequence(vec![0xfe as u8]),
//!             ]
//!         },
//!         // `payload`
//!         Field {
//!             name: std::string::String::from("payload"),
//!             attributes: vec![
//!                 FieldAttribute::Length(4usize),
//!             ]
//!         },
//!     ],
//!     attributes: vec![]
//! };
//! ```
//!

pub use std;
use log;

#[derive(Debug, Clone)]
pub struct MaxLengthFieldAttribute {
    pub value: usize,
}

impl MaxLengthFieldAttribute {
    pub fn get_default_value() -> usize {
        64usize
    }
}

/// Every field is modified with a set of attributes, such as
/// - length (if the field is of constant length);
/// - accepted values;
/// - hooks (for calculating checksums),
/// etc.
#[derive(Debug, Clone)]
pub enum FieldAttribute {
    MaxLength(MaxLengthFieldAttribute),
}

#[derive(Debug, Clone)]
pub struct RegexFieldType {
    pub regex: std::string::String,
}

#[derive(Debug, Clone)]
pub enum FieldType {
    /// Expect a certain sequence of bytes
    Regex(RegexFieldType),
}

#[derive(Debug)]
pub enum MessageAttribute {
    /// This message is the core of the protocol, which nests every other one
    Root,
}

#[derive(Debug)]
pub enum ProtocolAttribute {
}

/// Represents a protocol's message as a sequence of fields
#[derive(Debug)]
pub struct Message {
    pub name: std::string::String,
    pub fields: std::vec::Vec<Field>,
    pub attributes: std::vec::Vec<MessageAttribute>,
}

/// May be a regular field, such as byte sequence of fixed length, or u32, or a
/// payload (nested message))
#[derive(Debug, Clone)]
pub struct Field {
    pub name: std::string::String,
    pub field_type: FieldType,
    pub attributes: std::vec::Vec<FieldAttribute>,
}

/// Represents the entire protocol as a set of messages
#[derive(Debug)]
pub struct Protocol {
    pub messages: std::vec::Vec<Message>,
    pub attributes: std::vec::Vec<ProtocolAttribute>,
}

impl Protocol {
    /// Gets the root message. If absent, the first message is considered root
    pub fn root_message(&self) -> &Message {
        if self.messages.len() == 0 {
            log::error!("Empty messages list. Panicking");
            panic!();
        }

        for message in &self.messages {
            for attribute in &message.attributes {
                if let MessageAttribute::Root = attribute {
                    return message;
                }
            }
        }

        &self.messages[0]
    }
}
