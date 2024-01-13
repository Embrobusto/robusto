use crate::bpir::{
    self,
    representation::{
        self, Field, FieldAttribute, FieldType, MaxLengthFieldAttribute, RegexFieldType,
    },
};
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
pub struct RegexMachineFieldAstNode {
    /// Ragel machine string sequence (definition)
    pub string_sequence: std::string::String,
    pub name: std::string::String,
}

#[derive(Debug)]
pub struct MachineHeaderAstNode {
    pub machine_name: std::string::String,
}

#[derive(Debug)]
pub struct MachineDefinitionAstNode {
    pub machine_name: std::string::String,
    pub fields: std::vec::Vec<String>,
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

#[derive(Clone, Debug)]
pub struct MachineActionHookAstNode {
    /// Coincides w/ the field's name
    pub name: std::string::String,
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
    MachineActionHook(MachineActionHookAstNode),
    MessageStruct(MessageStructAstNode),
    MachineDefinition(MachineDefinitionAstNode),
    ParsingFunction(ParsingFunctionAstNode),
    RegexMachineField(RegexMachineFieldAstNode),
}

#[derive(Debug)]
pub struct AstNode {
    pub ast_node_type: Ast,
    pub children: std::vec::Vec<AstNode>,
}

impl Into<Ast> for AstNode {
    fn into(self) -> Ast {
        self.ast_node_type
    }
}

impl AsRef<Ast> for AstNode {
    fn as_ref(&self) -> &Ast {
        &self.ast_node_type
    }
}

impl AsMut<Ast> for AstNode {
    fn as_mut(&mut self) -> &mut Ast {
        &mut self.ast_node_type
    }
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
        let mut message_struct = self.add_child(Ast::MessageStruct(MessageStructAstNode {
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

        let mut machine_definition_node =
            self.add_child(Ast::MachineDefinition(MachineDefinitionAstNode {
                machine_name: message.name.clone(),
                fields: message.fields.iter().map(|f| f.name.clone()).collect(),
            }));

        for field in &message.fields {
            machine_definition_node.add_machine_action_hook(field);
        }

        for field in &message.fields {
            machine_definition_node.add_machine_field_parser(field);
        }

        let mut parsing_function = self.add_child(Ast::ParsingFunction(ParsingFunctionAstNode {
            message_name: message.name.clone(),
        }));

        for field in &message.fields {
        }
    }

    fn add_machine_action_hook(&mut self, field: &bpir::representation::Field) {
        self.add_child(Ast::MachineActionHook(MachineActionHookAstNode {
            name: field.name.clone(),
        }));
    }

    fn add_machine_field_parser(&mut self, field: &bpir::representation::Field) {
        use std::fmt;

        match field.field_type {
            bpir::representation::FieldType::Regex(ref node) => self.add_regex_machine_field_parser(field, node),
        }
        // Get field type
    }

    fn add_regex_machine_field_parser(&mut self, field: &bpir::representation::Field, regex: &bpir::representation::RegexFieldType) {
        self.add_child(Ast::RegexMachineField(RegexMachineFieldAstNode {
            string_sequence: regex.regex.clone(),
            name: field.name.clone(),
        }));
    }
}
