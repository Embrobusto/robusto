use crate::bpir;
use crate::bpir::representation::{FieldAttribute, FieldType};
use crate::utility::codegen::{
    CodeChunk, CodeGenerationState, RawCode, SubnodeAccess, TreeBasedCodeGeneration,
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
use std::collections::LinkedList;
use std::string::String;

/// Represents an abstract syntactic tree for Ragel code, with the difference
/// that its leaves mostly consist of snippets rather than atomic language
/// constructs, i.e. it is a less detailed representation of Ragel code.
///
#[derive(Debug)]
pub struct ParsingFunction {
    /// Each parsing function is supposed to be associated w/ a particular message
    pub message_name: std::string::String,
}

#[derive(Debug)]
pub struct RegexMachineField {
    /// Ragel machine string sequence (definition)
    pub string_sequence: std::string::String,
    pub name: std::string::String,
}

#[derive(Debug)]
pub struct MachineHeader {
    pub machine_name: std::string::String,
}

#[derive(Debug)]
pub struct MachineDefinition {
    pub machine_name: std::string::String,
    pub fields: std::vec::Vec<String>,
}

#[derive(Debug)]
pub struct MessageStruct {
    pub message_name: std::string::String,
}

#[derive(Clone, Debug)]
pub enum FieldBaseType {
    I8,
}

#[derive(Clone, Debug)]
pub struct MessageStructMember {
    pub name: std::string::String,
    pub field_base_type: FieldBaseType,

    /// If 0, it is considered just a field
    pub array_length: usize,
}

impl MessageStructMember {
    pub fn is_array(&self) -> bool {
        self.array_length > 0
    }
}

#[derive(Clone, Debug)]
pub struct MachineActionHook {
    /// Coincides w/ the field's name
    pub name: std::string::String,
}

#[derive(Debug)]
pub enum AstNodeType {
    /// An empty representation for a subtre
    Root,

    // C-specific elements (TBD)

    // Language-agnostic elements
    /// Just treat it as a mere sequence
    MessageStructMember(MessageStructMember),
    MessageStruct(MessageStruct),
    ParsingFunction(ParsingFunction),

    /// Ragel-specific machine header
    MachineHeader(MachineHeader),
    MachineActionHook(MachineActionHook),
    MachineDefinition(MachineDefinition),
    RegexMachineField(RegexMachineField),
    RawCode(RawCode),
}

impl TreeBasedCodeGeneration for MachineHeader {
    fn generate_code_pre_traverse(
        &self,
        generation_state: &mut CodeGenerationState,
    ) -> LinkedList<CodeChunk> {
        let mut ret = LinkedList::<CodeChunk>::new();
        ret.push_back(CodeChunk::new(
            "%%{".to_string(),
            generation_state.indent,
            1usize,
        ));
        ret.push_back(CodeChunk::new(
            format!("machine {0};", self.machine_name),
            generation_state.indent + 1,
            1usize,
        ));
        ret.push_back(CodeChunk::new(
            "write data;".to_string(),
            generation_state.indent + 1,
            1usize,
        ));
        ret.push_back(CodeChunk::new(
            "}%%".to_string(),
            generation_state.indent,
            1usize,
        ));

        ret
    }
}

impl TreeBasedCodeGeneration for MachineActionHook {
    fn generate_code_pre_traverse(
        &self,
        code_generation_state: &mut CodeGenerationState,
    ) -> LinkedList<CodeChunk> {
        let mut ret = LinkedList::<CodeChunk>::new();
        ret.push_back(CodeChunk::new(
            format!("action {0} {{", self.name),
            code_generation_state.indent,
            1usize,
        ));
        ret.push_back(CodeChunk::new(
            "}".to_string(),
            code_generation_state.indent,
            1usize,
        ));

        ret
    }
}

impl TreeBasedCodeGeneration for MachineDefinition {
    fn generate_code_pre_traverse(
        &self,
        code_generation_state: &mut CodeGenerationState,
    ) -> LinkedList<CodeChunk> {
        let mut ret = LinkedList::<CodeChunk>::new();
        ret.push_back(CodeChunk::new(
            "%%{".to_string(),
            code_generation_state.indent,
            1usize,
        ));
        ret.push_back(CodeChunk::new(
            format!("machine {0}", self.machine_name),
            code_generation_state.indent + 1,
            1usize,
        ));
        ret.push_back(CodeChunk::new(
            "write data;".to_string(),
            code_generation_state.indent + 1,
            1usize,
        ));

        code_generation_state.indent += 1;

        ret
    }

    fn generate_code_post_traverse(
        &self,
        code_generation_state: &mut CodeGenerationState,
    ) -> LinkedList<CodeChunk> {
        code_generation_state.indent -= 1;

        let mut ret = LinkedList::<CodeChunk>::new();

        ret.push_back(CodeChunk::new(
            "}%%".to_string(),
            code_generation_state.indent,
            1usize,
        ));


        ret
    }
}

impl TreeBasedCodeGeneration for RegexMachineField {
    fn generate_code_pre_traverse(
        &self,
        code_generation_state: &mut CodeGenerationState,
    ) -> LinkedList<CodeChunk> {
        let mut ret = LinkedList::<CodeChunk>::new();
        ret.push_back(CodeChunk::new(
            format!("{0} = '{1}' @{0}; ", self.name, self.string_sequence),
            code_generation_state.indent,
            1usize,
        ));

        ret
    }
}

#[derive(Debug)]
pub struct AstNode {
    pub ast_node_type: AstNodeType,
    pub children: std::vec::Vec<AstNode>,
}

impl From<&bpir::representation::Protocol> for AstNode {
    fn from(protocol: &bpir::representation::Protocol) -> Self {
        let mut root = AstNode {
            ast_node_type: AstNodeType::Root,
            children: vec![],
        };

        for message in &protocol.messages {
            root.add_message_parser(message);
        }

        root
    }
}

impl TreeBasedCodeGeneration for AstNodeType {
    fn generate_code_pre_traverse(
        &self,
        code_generation_state: &mut CodeGenerationState,
    ) -> LinkedList<CodeChunk> {
        match self {
            AstNodeType::MachineHeader(ref node) => {
                node.generate_code_pre_traverse(code_generation_state)
            }
            AstNodeType::MachineActionHook(ref node) => {
                node.generate_code_pre_traverse(code_generation_state)
            }
            AstNodeType::MachineDefinition(ref node) => {
                node.generate_code_pre_traverse(code_generation_state)
            }
            AstNodeType::RegexMachineField(ref node) => {
                node.generate_code_pre_traverse(code_generation_state)
            }
            AstNodeType::RawCode(ref node) => {
                node.generate_code_pre_traverse(code_generation_state)
            }
            n => {
                log::warn!("Unhandled node {:?}, skipping", n);

                LinkedList::new()
            }
        }
    }
    fn generate_code_post_traverse(
        &self,
        code_generation_state: &mut CodeGenerationState,
    ) -> LinkedList<CodeChunk> {
        match self {
            AstNodeType::MachineHeader(ref node) => {
                node.generate_code_post_traverse(code_generation_state)
            }
            AstNodeType::MachineActionHook(ref node) => {
                node.generate_code_post_traverse(code_generation_state)
            }
            AstNodeType::MachineDefinition(ref node) => {
                node.generate_code_post_traverse(code_generation_state)
            }
            AstNodeType::RegexMachineField(ref node) => {
                node.generate_code_post_traverse(code_generation_state)
            }
            AstNodeType::RawCode(ref node) => {
                node.generate_code_post_traverse(code_generation_state)
            }
            n => {
                log::warn!("Unhandled node {:?}, skipping", n);

                LinkedList::new()
            }
        }
    }
}

impl TreeBasedCodeGeneration for AstNode {
    fn generate_code_pre_traverse(
        &self,
        code_generation_state: &mut CodeGenerationState,
    ) -> LinkedList<CodeChunk> {
        self.ast_node_type
            .generate_code_pre_traverse(code_generation_state)
    }

    fn generate_code_post_traverse(
        &self,
        code_generation_state: &mut CodeGenerationState,
    ) -> LinkedList<CodeChunk> {
        self.ast_node_type
            .generate_code_post_traverse(code_generation_state)
    }
}

impl SubnodeAccess<AstNode> for AstNode {
    fn iter(&self) -> std::slice::Iter<'_, AstNode> {
        self.children.iter()
    }
}

///  Generates an intermediate AST node which then will get transformed into the
///  actual AST node for a target language
impl AstNode {
    /// Adds a new child to a node. Returns reference to the new child
    fn add_child(&mut self, ast_node_type: AstNodeType) -> &mut AstNode {
        let child = AstNode {
            ast_node_type,
            children: vec![],
        };
        self.children.push(child);
        self.children.last_mut().unwrap()
    }

    /// Visitor.
    ///
    /// Changes nodes of the tree recursively
    pub fn apply_replacement_recursive(&mut self, apply_replacement: fn(&mut Self)) {
        apply_replacement(self);

        for subnode in &mut self.children {
            subnode.apply_replacement_recursive(apply_replacement);
        }
    }

    fn add_message_parser(&mut self, message: &bpir::representation::Message) {
        self.add_child(AstNodeType::MachineHeader(MachineHeader {
            machine_name: message.name.clone(),
        }));
        let mut message_struct = self.add_child(AstNodeType::MessageStruct(MessageStruct {
            message_name: message.name.clone(),
        }));

        for field in &message.fields {
            message_struct.add_child(AstNodeType::MessageStructMember(MessageStructMember {
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
                        value = bpir::representation::MaxLengthFieldAttribute::get_default_value();

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
            self.add_child(AstNodeType::MachineDefinition(MachineDefinition {
                machine_name: message.name.clone(),
                fields: message.fields.iter().map(|f| f.name.clone()).collect(),
            }));

        for field in &message.fields {
            machine_definition_node.add_machine_action_hook(field);
        }

        for field in &message.fields {
            machine_definition_node.add_machine_field_parser(field);
        }

        let mut parsing_function = self.add_child(AstNodeType::ParsingFunction(ParsingFunction {
            message_name: message.name.clone(),
        }));

        for field in &message.fields {}
    }

    fn add_machine_action_hook(&mut self, field: &bpir::representation::Field) {
        self.add_child(AstNodeType::MachineActionHook(MachineActionHook {
            name: field.name.clone(),
        }));
    }

    fn add_machine_field_parser(&mut self, field: &bpir::representation::Field) {
        use std::fmt;

        match field.field_type {
            bpir::representation::FieldType::Regex(ref node) => {
                self.add_regex_machine_field_parser(field, node)
            }
        }
        // Get field type
    }

    fn add_regex_machine_field_parser(
        &mut self,
        field: &bpir::representation::Field,
        regex: &bpir::representation::RegexFieldType,
    ) {
        self.add_child(AstNodeType::RegexMachineField(RegexMachineField {
            string_sequence: regex.regex.clone(),
            name: field.name.clone(),
        }));
    }
}
