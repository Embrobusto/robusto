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
use std::boxed::Box;
use std::io::BufWriter;
use std::string::String;

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

struct CodeGenerationState {
    // Current indent.
    indent: usize,
}

impl CodeGenerationState {
    fn new() -> CodeGenerationState {
        CodeGenerationState { indent: 0 }
    }
}

pub struct CodeGeneration<'a> {
    ast: &'a AstNode,
}

impl CodeGeneration<'_> {
    pub fn from_ragel_ast(ast_node: &AstNode) -> CodeGeneration {
        CodeGeneration { ast: ast_node }
    }

    fn generate_traverse_ast_node_children<W: std::io::Write>(
        &self,
        ast_node: &AstNode,
        buf_writer: &mut std::io::BufWriter<W>,
        generation_state: &mut CodeGenerationState,
    ) {
        for child in &ast_node.children {
            self.generate_traverse_ast_node(child, buf_writer, generation_state);
        }
    }

    fn generate_traverse_ast_node<W: std::io::Write>(
        &self,
        ast_node: &AstNode,
        buf_writer: &mut std::io::BufWriter<W>,
        generation_state: &mut CodeGenerationState,
    ) {
        match ast_node.ast_node_type {
            Ast::MachineHeader(ref node) => self
                .generate_machine_header(
                    ast_node,
                    buf_writer,
                    &node.machine_name,
                    generation_state,
                ),
            Ast::MachineDefinition(ref node) => {
                self.generate_machine_definition(ast_node, buf_writer, &node, generation_state);
            }
            Ast::None => {
                self.generate_traverse_ast_node_children(ast_node, buf_writer, generation_state);
            }
            Ast::ParsingFunction(ref node) => {
                self.generate_parsing_function(ast_node, buf_writer, &node, generation_state);
            }
            Ast::MessageStruct(ref node) => {
                self.generate_message_struct(ast_node, buf_writer, &node, generation_state);
            }
            Ast::MessageStructMember(ref node) => {
                self.generate_message_struct_member(ast_node, buf_writer, node, generation_state);
            }
            Ast::RegexMachineField(ref node) => {
                self.generate_regex_machine_field_parser(
                    ast_node,
                    buf_writer,
                    node,
                    generation_state,
                );
            }
            Ast::MachineActionHook(ref node) => {
                self.generate_machine_action_hook(ast_node, buf_writer, node, generation_state);
            }
            _ => {
                log::error!(
                    "Unmatched node \"{:?}\", panicking!",
                    ast_node.ast_node_type
                );
                panic!();
            }
        }
    }

    fn generate_machine_header<W: std::io::Write>(
        &self,
        ast_node: &AstNode,
        buf_writer: &mut std::io::BufWriter<W>,
        machine_name: &std::string::String,
        generation_state: &mut CodeGenerationState,
    ) {
        utility::string::write_with_indent_or_panic(
            buf_writer,
            generation_state.indent,
            format!(
                "%%{{
    machine {machine_name};
    write data;
}}%%

struct {machine_name}ParserState {{
    int machineInitRequired;
    int cs;  // Ragel-specific state variable
}};

void machine{machine_name}ParserStateInit(struct {machine_name}ParserState *aParserState)
{{
    aParserState->machineInitRequired = 0;
    aParserState->cs = 0;
    %% write init;
}}
"
            )
            .as_bytes(),
        );
    }

    fn generate_machine_definition<W: std::io::Write>(
        &self,
        ast_node: &AstNode,
        buf_writer: &mut std::io::BufWriter<W>,
        node: &MachineDefinitionAstNode,
        generation_state: &mut CodeGenerationState,
    ) {
        utility::string::write_with_indent_or_panic(
            buf_writer,
            generation_state.indent,
            format!(
                "%%{{
    machine {0};
    access aParserState->;
",
                node.machine_name
            )
            .as_bytes(),
        );
        generation_state.indent += 1;

        self.generate_traverse_ast_node_children(ast_node, buf_writer, generation_state);
        utility::string::write_line_with_indent_or_panic(
            buf_writer,
            generation_state.indent,
            format!("main := {0};", node.fields.join(" ")).as_bytes(),
        );

        generation_state.indent -= 1;
        utility::string::write_line_with_indent_or_panic(
            buf_writer,
            generation_state.indent,
            "}%%".as_bytes(),
        );
        utility::string::write_line_with_indent_or_panic(
            buf_writer,
            generation_state.indent,
            "".as_bytes(),
        );
    }

    fn generate_parsing_function<W: std::io::Write>(
        &self,
        ast_node: &AstNode,
        buf_writer: &mut std::io::BufWriter<W>,
        node: &ParsingFunctionAstNode,
        generation_state: &mut CodeGenerationState,
    ) {
        // Generate ragel parsing function state
        utility::string::write_line_with_indent_or_panic(
            buf_writer,
            generation_state.indent,
            format!(
                "void parse{0}(struct {3}ParserState *aParserState, const char *aInputBuffer, int aInputBufferLength, struct {1} *a{2})",
                node.message_name, node.message_name, node.message_name, node.message_name
            )
            .as_bytes(),
        );
        utility::string::write_line_with_indent_or_panic(
            buf_writer,
            generation_state.indent,
            "{".as_bytes(),
        );
        generation_state.indent += 1;
        utility::string::write_with_indent_or_panic(buf_writer, generation_state.indent, format!(
"const char *p = aInputBuffer;  // Iterator \"begin\" pointer -- Ragel-specific variable for C code generation
const char *pe = aInputBuffer + aInputBufferLength;  // Iterator \"end\" pointer -- Ragel-specific variable for C code generation

// Parse starting from the state defined in `aParserState`
%% write exec;
"
        ).as_bytes());
        generation_state.indent -= 1;
        utility::string::write_line_with_indent_or_panic(
            buf_writer,
            generation_state.indent,
            "}".as_bytes(),
        );
        self.generate_traverse_ast_node_children(ast_node, buf_writer, generation_state);
    }

    fn generate_message_struct<W: std::io::Write>(
        &self,
        ast_node: &AstNode,
        buf_writer: &mut std::io::BufWriter<W>,
        node: &MessageStructAstNode,
        generation_state: &mut CodeGenerationState,
    ) {
        utility::string::write_line_with_indent_or_panic(
            buf_writer,
            generation_state.indent,
            format!("struct {0}Message {{", node.message_name).as_bytes(),
        );
        generation_state.indent += 1;
        self.generate_traverse_ast_node_children(ast_node, buf_writer, generation_state);
        generation_state.indent -= 1;
        utility::string::write_line_with_indent_or_panic(
            buf_writer,
            generation_state.indent,
            "};".as_bytes(),
        );
    }

    fn generate_message_struct_member<W: std::io::Write>(
        &self,
        ast_node: &AstNode,
        buf_writer: &mut std::io::BufWriter<W>,
        node: &MessageStructMemberAstNode,
        generation_state: &mut CodeGenerationState,
    ) {
        let formatted = format!(
            "{0} {1}{2};",
            match node.field_base_type {
                FieldBaseType::I8 => {
                    "uint8_t"
                }
                _ => {
                    panic!("Unsupported type {:?}", node.field_base_type)
                }
            },
            node.name,
            {
                if node.array_length == 0usize {
                    std::string::String::from("")
                } else {
                    format!("[{}]", node.array_length)
                }
            }
        );
        utility::string::write_line_with_indent_or_panic(
            buf_writer,
            generation_state.indent,
            formatted.as_bytes(),
        );
    }

    fn generate_machine_action_hook<W: std::io::Write>(
        &self,
        ast_node: &AstNode,
        buf_writer: &mut std::io::BufWriter<W>,
        node: &MachineActionHookAstNode,
        generation_state: &mut CodeGenerationState,
    ) {
        utility::string::write_with_indent_or_panic(
            buf_writer,
            generation_state.indent,
            format!(
                "action {0} {{
}}
",
                node.name
            )
            .as_bytes(),
        );
    }

    fn generate_regex_machine_field_parser<W: std::io::Write>(
        &self,
        ast_node: &AstNode,
        buf_writer: &mut std::io::BufWriter<W>,
        node: &RegexMachineFieldAstNode,
        generation_state: &mut CodeGenerationState,
    ) {
        utility::string::write_line_with_indent_or_panic(
            buf_writer,
            generation_state.indent,
            format!("{0} = '{1}' @{0}; ", node.name, node.string_sequence).as_bytes(),
        );
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

        for field in &message.fields {}
    }

    fn add_machine_action_hook(&mut self, field: &bpir::representation::Field) {
        self.add_child(Ast::MachineActionHook(MachineActionHookAstNode {
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
        self.add_child(Ast::RegexMachineField(RegexMachineFieldAstNode {
            string_sequence: regex.regex.clone(),
            name: field.name.clone(),
        }));
    }
}
