use crate::bpir::representation::Protocol;
use crate::parser_generation;
use crate::parser_generation::ragel::common;
use crate::utility;
use crate::utility::codegen;
use crate::utility::codegen::{CodeChunk, CodeGeneration, MockCodeGenerator};
use log;
use std::collections::LinkedList;
use std::string::String;
use std::vec::Vec;

#[derive(Debug)]
struct GenerationState {
    // Current indent.
    indent: usize,
}

impl GenerationState {
    fn new() -> GenerationState {
        GenerationState { indent: 0 }
    }
}

#[derive(Debug)]
struct ParsingFunciton {
    message_name: String,
}

#[derive(Debug)]
pub struct MessageStruct {
    pub message_name: std::string::String,
}

impl codegen::CodeGeneration for MessageStruct {
    fn generate_code(
        &self,
        code_generation_state: &mut codegen::CodeGenerationState,
    ) -> LinkedList<CodeChunk> {
        let mut ret = LinkedList::<codegen::CodeChunk>::new();

        // Generate struct header
        ret.push_back(CodeChunk::new(
            format!("struct {0}Message {{", self.message_name),
            code_generation_state.indent,
            1usize,
        ));

        // Generate the content
        code_generation_state.indent += 1;
        code_generation_state.indent -= 1;

        // Close the bracket
        ret.push_back(CodeChunk::new(
            "}".to_string(),
            code_generation_state.indent,
            1usize,
        ));

        ret
    }

    fn generate_code_post_iter(
        &self,
        code_generation_state: &mut codegen::CodeGenerationState,
    ) -> LinkedList<CodeChunk> {
        let mut ret = LinkedList::<codegen::CodeChunk>::new();
        code_generation_state.indent -= 1;

        // Close the bracket
        ret.push_back(CodeChunk::new(
            "}".to_string(),
            code_generation_state.indent,
            1usize,
        ));

        ret
    }
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

impl codegen::CodeGeneration for MessageStructMember {
    fn generate_code(
        &self,
        code_generation_state: &mut codegen::CodeGenerationState,
    ) -> LinkedList<CodeChunk> {
        let mut ret = LinkedList::<codegen::CodeChunk>::new();

        // Get a formatted C representation
        let formatted = format!(
            "{0} {1}{2};",
            match self.field_base_type {
                FieldBaseType::I8 => {
                    "uint8_t"
                }
                _ => {
                    panic!("Unsupported type {:?}", self.field_base_type)
                }
            },
            self.name,
            {
                if self.array_length == 0usize {
                    std::string::String::from("")
                } else {
                    format!("[{}]", self.array_length)
                }
            }
        );

        ret.push_back(CodeChunk::new(
            formatted,
            code_generation_state.indent,
            1usize,
        ));

        ret
    }
}

impl MessageStructMember {
    pub fn is_array(&self) -> bool {
        self.array_length > 0
    }
}

#[derive(Clone, Debug)]
struct ParserStateStruct {
    machine_name: String,
}

impl codegen::CodeGeneration for ParserStateStruct {
    fn generate_code(
        &self,
        code_generation_state: &mut codegen::CodeGenerationState,
    ) -> LinkedList<codegen::CodeChunk> {
        let mut ret = LinkedList::<codegen::CodeChunk>::new();
        ret.push_back(CodeChunk::new(
            format!("struct {0}ParserState {{", self.machine_name),
            code_generation_state.indent,
            1usize,
        ));
        ret.push_back(CodeChunk::new(
            "int machineInitRequired;".to_string(),
            code_generation_state.indent,
            1usize,
        ));
        ret.push_back(CodeChunk::new(
            "int cs;".to_string(),
            code_generation_state.indent,
            1usize,
        ));
        ret.push_back(CodeChunk::new(
            "};".to_string(),
            code_generation_state.indent,
            1usize,
        ));

        ret
    }
}

pub struct ParserStateInitFunction {
    pub machine_name: String,
}

impl codegen::CodeGeneration for ParserStateInitFunction {
    fn generate_code(
        &self,
        code_generation_state: &mut codegen::CodeGenerationState,
    ) -> LinkedList<CodeChunk> {
        let mut ret = LinkedList::<codegen::CodeChunk>::new();
        ret.push_back(CodeChunk::new(
            format!(
                "void machine{0}ParserStateInit(struct {0}ParserState *aParserState)",
                self.machine_name
            ),
            code_generation_state.indent,
            1usize,
        ));
        ret.push_back(CodeChunk::new(
            "{".to_string(),
            code_generation_state.indent,
            1usize,
        ));
        code_generation_state.indent += 1usize;
        ret.push_back(CodeChunk::new(
            "aParserState->machineInitRequired = 0;".to_string(),
            code_generation_state.indent,
            1usize,
        ));
        ret.push_back(CodeChunk::new(
            "aParserState->cs = 0;".to_string(),
            code_generation_state.indent,
            1usize,
        ));
        ret.push_back(CodeChunk::new(
            "%% write init;".to_string(),
            code_generation_state.indent,
            1usize,
        ));
        code_generation_state.indent -= 1usize;
        ret.push_back(CodeChunk::new(
            "}".to_string(),
            code_generation_state.indent,
            1usize,
        ));

        ret
    }
}

impl codegen::CodeGeneration for ParsingFunciton {
    fn generate_code(
        &self,
        code_generation_state: &mut codegen::CodeGenerationState,
    ) -> LinkedList<codegen::CodeChunk> {
        let mut ret = LinkedList::<codegen::CodeChunk>::new();
        ret.push_back(codegen::CodeChunk::new(
            format!("void parse{0}(struct {0}ParserState *aParserState, const char *aInputBuffer, int aInputBufferLength, struct {0} *a{0})", self.message_name),
            code_generation_state.indent,
            1usize
        ));
        ret.push_back(codegen::CodeChunk::new(
            "{".to_string(),
            code_generation_state.indent,
            1usize,
        ));
        code_generation_state.indent += 1usize;
        ret.push_back(codegen::CodeChunk::new(
            "const char *p = aInputBuffer;  // Iterator \"begin\" pointer -- Ragel-specific variable for C code generation".to_string(),
            code_generation_state.indent,
            1usize,
        ));
        ret.push_back(codegen::CodeChunk::new(
            "const char *pe = aInputBuffer + aInputBufferLength;  // Iterator \"end\" pointer -- Ragel-specific variable for C code generation".to_string(),
            code_generation_state.indent,
            1usize,
        ));
        ret.push_back(codegen::CodeChunk::new(
            "// Parse starting from the state defined in `aParserState`".to_string(),
            code_generation_state.indent,
            1usize,
        ));
        ret.push_back(codegen::CodeChunk::new(
            "%% write exec;".to_string(),
            code_generation_state.indent,
            1usize,
        ));
        code_generation_state.indent -= 1usize;
        ret.push_back(codegen::CodeChunk::new(
            "}".to_string(),
            code_generation_state.indent,
            1usize,
        ));

        ret
    }
}

enum AstNodeType {
    Root,
    ParsingFunction(ParsingFunciton),
    ParserStateStruct(ParserStateStruct),
    ParserStateInitFunction(ParserStateInitFunction),
    MessageStruct(MessageStruct),
    MessageStructMember(MessageStructMember),
    Common(common::AstNodeType),
}

struct AstNode {
    ast_node_type: AstNodeType,
    children: Vec<AstNode>,
}

impl AstNode {
    fn new() -> AstNode {
        AstNode {
            ast_node_type: AstNodeType::Root,
            children: Vec::new(),
        }
    }

    fn add_child(&mut self, ast_node_type: AstNodeType) -> &mut AstNode {
        self.children.push(AstNode {
            ast_node_type,
            children: Vec::new(),
        });

        self.children.last_mut().unwrap()
    }
}

impl CodeGeneration for AstNode {
    fn generate_code(
        &self,
        code_generation_state: &mut codegen::CodeGenerationState,
    ) -> LinkedList<CodeChunk> {
        // Erase the type to spare some code
        let mock_code_generator = codegen::MockCodeGenerator {};
        let code_generator: &dyn CodeGeneration = match self.ast_node_type {
            AstNodeType::ParsingFunction(ref node) => node,
            AstNodeType::ParserStateStruct(ref node) => node,
            AstNodeType::ParserStateInitFunction(ref node) => node,
            AstNodeType::MessageStruct(ref node) => node,
            AstNodeType::MessageStructMember(ref node) => node,
            _ => {
                log::warn!("Unhandled node, skipping");

                &mock_code_generator
            }
        };

        // Perform initial code generation
        let mut ret = code_generator.generate_code(code_generation_state);

        // Traverse children in the same manner
        for child in &self.children {
            ret.append(&mut child.generate_code(code_generation_state));
        }

        // Make a post-traverse generation
        ret.append(&mut code_generator.generate_code_post_iter(code_generation_state));

        ret
    }
}

/// AST tree for generating C source files
pub struct SourceAstNode {
    ast_node: AstNode,
}

impl CodeGeneration for SourceAstNode {
    fn generate_code(
        &self,
        code_generation_state: &mut codegen::CodeGenerationState,
    ) -> LinkedList<CodeChunk> {
        self.ast_node.generate_code(code_generation_state)
    }
}

impl From<&Protocol> for SourceAstNode {
    fn from(protocol: &Protocol) -> Self {
        let common_ast_node = common::AstNode::from(protocol);
        let mut ret = Self {
            ast_node: AstNode::new(),
        };

        ret
    }
}

pub struct HeaderAstNode {
    ast_node: AstNode,
}

impl From<&Protocol> for HeaderAstNode {
    fn from(protocol: &Protocol) -> Self {
        let mut ret = Self {
            ast_node: AstNode::new(),
        };

        // TODO: build the tree

        ret
    }
}

/// C-specific Ragel AST
pub struct Generator<'a> {
    ast: &'a parser_generation::ragel::common::AstNode,
}

impl Generator<'_> {
    pub fn from_ragel_ast(ast_node: &parser_generation::ragel::common::AstNode) -> Generator {
        Generator { ast: ast_node }
    }

    fn generate_traverse_ast_node_children<W: std::io::Write>(
        &self,
        ast_node: &parser_generation::ragel::common::AstNode,
        buf_writer: &mut std::io::BufWriter<W>,
        generation_state: &mut GenerationState,
    ) {
        for child in &ast_node.children {
            self.generate_traverse_ast_node(child, buf_writer, generation_state);
        }
    }

    fn generate_traverse_ast_node<W: std::io::Write>(
        &self,
        ast_node: &parser_generation::ragel::common::AstNode,
        buf_writer: &mut std::io::BufWriter<W>,
        generation_state: &mut GenerationState,
    ) {
        match ast_node.ast_node_type {
            parser_generation::ragel::common::AstNodeType::MachineHeader(ref node) => self
                .generate_machine_header(
                    ast_node,
                    buf_writer,
                    &node.machine_name,
                    generation_state,
                ),
            parser_generation::ragel::common::AstNodeType::MachineDefinition(ref node) => {
                self.generate_machine_definition(ast_node, buf_writer, &node, generation_state);
            }
            parser_generation::ragel::common::AstNodeType::None => {
                self.generate_traverse_ast_node_children(ast_node, buf_writer, generation_state);
            }
            parser_generation::ragel::common::AstNodeType::ParsingFunction(ref node) => {
                self.generate_parsing_function(ast_node, buf_writer, &node, generation_state);
            }
            parser_generation::ragel::common::AstNodeType::MessageStruct(ref node) => {
                self.generate_message_struct(ast_node, buf_writer, &node, generation_state);
            }
            parser_generation::ragel::common::AstNodeType::MessageStructMember(ref node) => {
                self.generate_message_struct_member(ast_node, buf_writer, node, generation_state);
            }
            parser_generation::ragel::common::AstNodeType::RegexMachineField(ref node) => {
                self.generate_regex_machine_field_parser(
                    ast_node,
                    buf_writer,
                    node,
                    generation_state,
                );
            }
            parser_generation::ragel::common::AstNodeType::MachineActionHook(ref node) => {
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
        ast_node: &parser_generation::ragel::common::AstNode,
        buf_writer: &mut std::io::BufWriter<W>,
        machine_name: &std::string::String,
        generation_state: &mut GenerationState,
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
        ast_node: &parser_generation::ragel::common::AstNode,
        buf_writer: &mut std::io::BufWriter<W>,
        node: &parser_generation::ragel::common::MachineDefinition,
        generation_state: &mut GenerationState,
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
        ast_node: &parser_generation::ragel::common::AstNode,
        buf_writer: &mut std::io::BufWriter<W>,
        node: &parser_generation::ragel::common::ParsingFunction,
        generation_state: &mut GenerationState,
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
        ast_node: &parser_generation::ragel::common::AstNode,
        buf_writer: &mut std::io::BufWriter<W>,
        node: &parser_generation::ragel::common::MessageStruct,
        generation_state: &mut GenerationState,
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
        ast_node: &parser_generation::ragel::common::AstNode,
        buf_writer: &mut std::io::BufWriter<W>,
        node: &parser_generation::ragel::common::MessageStructMember,
        generation_state: &mut GenerationState,
    ) {
        let formatted = format!(
            "{0} {1}{2};",
            match node.field_base_type {
                parser_generation::ragel::common::FieldBaseType::I8 => {
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
        ast_node: &parser_generation::ragel::common::AstNode,
        buf_writer: &mut std::io::BufWriter<W>,
        node: &parser_generation::ragel::common::MachineActionHook,
        generation_state: &mut GenerationState,
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
        ast_node: &parser_generation::ragel::common::AstNode,
        buf_writer: &mut std::io::BufWriter<W>,
        node: &parser_generation::ragel::common::RegexMachineField,
        generation_state: &mut GenerationState,
    ) {
        utility::string::write_line_with_indent_or_panic(
            buf_writer,
            generation_state.indent,
            format!("{0} = '{1}' @{0}; ", node.name, node.string_sequence).as_bytes(),
        );
    }
}

impl parser_generation::Write for Generator<'_> {
    fn write<W: std::io::Write>(&self, buf_writer: &mut std::io::BufWriter<W>) {
        let mut generation_state = GenerationState::new();
        self.generate_traverse_ast_node(self.ast, buf_writer, &mut generation_state);
    }
}
