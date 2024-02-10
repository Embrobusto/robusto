use crate::{
    bpir::{
        self,
        representation::{
            self, Field, FieldAttribute, FieldType, MaxLengthFieldAttribute, RegexFieldType,
        },
    },
    utility,
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
use std::collections::{linked_list, LinkedList};
use std::io::BufWriter;
use std::string::String;
use std::{boxed::Box, io::LineWriter};

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

#[derive(Debug)]
pub struct GenericStruct {
    pub name: std::string::String,
    pub members: Vec<MessageStructMember>,
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
pub enum Ast {
    /// An empty representation for a subtre
    None,

    // C-specific elements (TBD)

    // Language-agnostic elements
    /// Just treat it as a mere sequence
    MessageStructMember(MessageStructMember),
    MessageStruct(MessageStruct),
    GenericStruct(GenericStruct),
    ParsingFunction(ParsingFunction),

    /// Ragel-specific machine header
    MachineHeader(MachineHeader),
    MachineActionHook(MachineActionHook),
    MachineDefinition(MachineDefinition),
    RegexMachineField(RegexMachineField),
}

struct CodeGenerationState {
    // Current indent.
    indent: usize,
}

impl CodeGenerationState {
    fn new() -> CodeGenerationState {
        CodeGenerationState { indent: 0 }
    }
}

struct CodeChunk {
    code: String,

    /// Indents in the code chunk's lines
    indent: usize,

    /// Number of new lines to add after the chunk
    newlines: usize,
}

impl CodeChunk {
    fn new(code: String, indent: usize, newlines: usize) -> CodeChunk {
        CodeChunk { code, indent, newlines }
    }
}

/// Generates a series of code chunks which can just be dumped as-is into
/// whatever output is used, e.g. file stream
trait CodeGeneration {
    fn generate_code(
        &self,
        ast_node: &AstNode,
        generation_state: &mut CodeGenerationState,
    ) -> LinkedList<CodeChunk>;
}


/// Orchestrated code generation for Ragel. Platform-dependent code generation
/// is delegated to `target_code_generation`
struct RagelCodeGeneration<'a> {
    /// Handles all the code
    common_code_generation: CommonCodeGeneration,
    target_code_generation: &'a dyn CodeGeneration,
}

struct CommonCodeGeneration {
}

impl CommonCodeGeneration {
    fn generate_machine_header(
        &self,
        ast_node: &AstNode,
        machine_header: &MachineHeader,
        generation_state: &mut CodeGenerationState,
    ) -> LinkedList<CodeChunk> {
        // Generate the representation
        let mut ret = LinkedList::<CodeChunk>::new();
        ret.push_back(CodeChunk::new("%%{".to_string(), generation_state.indent, 1usize));
        ret.push_back(CodeChunk::new(format!("machine {0};", machine_header.machine_name),
            generation_state.indent + 1, 1usize));
        ret.push_back(CodeChunk::new("write data;".to_string(), generation_state.indent + 1, 1usize));
        ret.push_back(CodeChunk::new("}%%".to_string(), generation_state.indent, 1usize));

//         utility::string::append_with_indent_or_panic(&mut sink, generation_state.indent, format!(
// "
// // TODO: parser state struct
// struct {machine_name}ParserState {{
//     int machineInitRequired;
//     int cs;  // Ragel-specific state variable
// }};

// // TODO: parser state initialization function
// void machine{machine_name}ParserStateInit(struct {machine_name}ParserState *aParserState)
// {{
//     aParserState->machineInitRequired = 0;
//     aParserState->cs = 0;
//     %% write init;
// }}
// "
//         ).as_bytes());

        ret
    }

    fn generate_regex_machine_field_parser<W: std::io::Write>(
        &self,
        ast_node: &AstNode,
        node: &RegexMachineField,
        generation_state: &mut CodeGenerationState,
    ) -> LinkedList<CodeChunk> {
        let mut ret = LinkedList::<CodeChunk>::new();
        ret.push_back(CodeChunk::new(format!("{0} = '{1}' @{0}; ", node.name, node.string_sequence), generation_state.indent, 1usize));

        ret
    }
}

trait AsCode {
    fn as_code(_ast: &Ast) -> Option<String> {
        None
    }
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

///  Generates an intermediate AST node which then will get transformed into the
///  actual AST node for a target language
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
        self.add_child(Ast::MachineHeader(MachineHeader {
            machine_name: message.name.clone(),
        }));
        let mut message_struct = self.add_child(Ast::MessageStruct(MessageStruct {
            message_name: message.name.clone(),
        }));

        for field in &message.fields {
            message_struct.add_child(Ast::MessageStructMember(MessageStructMember {
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
            self.add_child(Ast::MachineDefinition(MachineDefinition {
                machine_name: message.name.clone(),
                fields: message.fields.iter().map(|f| f.name.clone()).collect(),
            }));

        for field in &message.fields {
            machine_definition_node.add_machine_action_hook(field);
        }

        for field in &message.fields {
            machine_definition_node.add_machine_field_parser(field);
        }

        let mut parsing_function = self.add_child(Ast::ParsingFunction(ParsingFunction {
            message_name: message.name.clone(),
        }));

        for field in &message.fields {}
    }

    fn add_machine_action_hook(&mut self, field: &bpir::representation::Field) {
        self.add_child(Ast::MachineActionHook(MachineActionHook {
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
        self.add_child(Ast::RegexMachineField(RegexMachineField {
            string_sequence: regex.regex.clone(),
            name: field.name.clone(),
        }));
    }
}
