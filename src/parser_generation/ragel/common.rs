use crate::bpir::{self, representation::{self, Field, FieldType, FieldAttribute, RegexFieldType, MaxLengthFieldAttribute}};
use log;
/// Generates an AST-like tree of patterns common for languages supporting
/// procedural paradigm (Rust 2018, and ANSI C at this point).
///
/// To qualify for using this representation, a language must have the following
/// features:
///
/// - Support for C-like `struct`s;
/// - Support for functions;
/// - Support for mutable pointers or similar entities;
pub use std;

/// Represents an abstract syntactic tree for Ragel code, with the difference
/// that its leaves mostly consist of snippets rather than atomic language
/// constructs, i.e. it is a less detailed representation of Ragel code.
///
#[derive(Debug)]
pub struct ParsingFunctionAstNode {
    /// Each parsing function is supposed to be associated w/ a particular message
    pub message_name: std::string::String,
}

#[derive(Debug)]
pub struct RawStringSequenceAstNode {
    pub string_sequence: std::string::String,
}

#[derive(Debug)]
pub struct MachineHeaderAstNode {
    pub machine_name: std::string::String,
}

#[derive(Debug)]
pub struct MachineDefinitionAstNode {
    pub machine_name: std::string::String,
}

#[derive(Debug)]
pub struct MessageStructAstNode {
    pub message_name: std::string::String,
}

#[derive(Clone, Debug)]
pub enum FieldBaseType {
    I8,
}

#[derive(Clone, Debug)]
pub struct MessageStructMemberAstNode {
    pub name: std::string::String,
    pub field_base_type: FieldBaseType,

    /// If 0, it is considered just a field
    pub array_length: usize,
}

impl MessageStructMemberAstNode {
    pub fn is_array(&self) -> bool {
        self.array_length > 0
    }
}

#[derive(Debug)]
pub enum Ast {
    // C-specific elements (TBD)

    // Language-agnostic elements
    /// Just treat it as a mere sequence
    None,
    MessageStructMember(MessageStructMemberAstNode),

    /// Ragel-specific machine header
    MachineHeader(MachineHeaderAstNode),
    MessageStruct(MessageStructAstNode),
    MachineDefinition(MachineDefinitionAstNode),
    ParsingFunction(ParsingFunctionAstNode),
    RawStringSequence(RawStringSequenceAstNode),
}

#[derive(Debug)]
pub struct AstNode {
    pub ast_node_type: Ast,
    pub children: std::vec::Vec<AstNode>,
}

impl AstNode {
    pub fn from_protocol(protocol: &bpir::representation::Protocol) -> AstNode {
        let mut root = AstNode {
            ast_node_type: Ast::None,
            children: vec![],
        };
        for message in &protocol.messages {
            root.add_message_parser(message);
        }

        root
    }

    /// Adds a new child to a node. Returns reference to the new child
    fn add_child(&mut self, ast_node_type: Ast) -> &mut AstNode {
        let child = AstNode {
            ast_node_type,
            children: vec![],
        };
        self.children.push(child);
        self.children.last_mut().unwrap()
    }

    fn add_message_parser(&mut self, message: &bpir::representation::Message) {
        self.add_child(Ast::MachineHeader(MachineHeaderAstNode {
            machine_name: message.name.clone(),
        }));
        let mut message_struct = self.add_child(Ast::MessageStruct(MessageStructAstNode{
            message_name: message.name.clone(),
        }));

        for field in &message.fields {
            message_struct.add_child(Ast::MessageStructMember(MessageStructMemberAstNode {
                name: field.name.clone(),
                field_base_type: match field.field_type {
                    FieldType::Regex(_) => FieldBaseType::I8,
                },
                array_length: {
                    let mut value = 0;

                    match field.field_type {
                        FieldType::Regex(_) => {
                            for attribute in &field.attributes {
                                if let FieldAttribute::MaxLength(ref max_length) = attribute {
                                    value = max_length.value
                                }
                            }
                        }
                    }

                    if value == 0usize {
                        value = representation::MaxLengthFieldAttribute::get_default_value();

                        log::warn!(
                            "Did not get \"MaxLength\" attribute for field \"{}\" in message \"{}\", using default \"{}\"",
                            field.name,
                            message.name,
                            value,
                        );
                    }

                    value
                }
            }));
        }

        self.add_child(Ast::MachineDefinition(MachineDefinitionAstNode {
            machine_name: message.name.clone(),
        }));
        let mut parsing_function = self.add_child(Ast::ParsingFunction(ParsingFunctionAstNode {
            message_name: message.name.clone(),
        }));

        for field in &message.fields {
            parsing_function.add_field_parser(field);
        }
    }

    fn add_field_parser(&mut self, field: &bpir::representation::Field) {
        use std::fmt;

        match field.field_type {
            bpir::representation::FieldType::Regex(_) => self.add_regex_field_parser(field),
        }
        // Get field type
    }

    fn add_regex_field_parser(&mut self, field: &bpir::representation::Field) {
        if let bpir::representation::FieldType::Regex(ref regex) = field.field_type {
            self.add_child(Ast::RawStringSequence(RawStringSequenceAstNode {
                string_sequence: format!("%%{{ {} := {} %%}}", field.name, regex.regex),
            }));
        }
    }
}
