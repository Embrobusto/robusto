use std;

/// Every field is modified with a set of attributes, such as
/// - length (if the field is of constant length);
/// - accepted values;
/// - hooks (for calculating checksums),
/// etc.
pub enum FieldAttribute {
    /// Expected length
    ExpectLength(usize),

    /// Expect a certain sequence of bytes
    ExpectConstValue(std::vec::Vec<u8>),

    /// Interpret the payload as a message
    PayloadById()
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
