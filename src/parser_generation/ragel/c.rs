/// Generates C code from procedural representation
use crate::bpir;
use crate::bpir::representation;
use crate::bpir::representation::RegexFieldType;
use crate::parser_generation;
use crate::utility;
use log;
use std;
use std::fmt::write;
use std::io::Write;
use std::str::FromStr;

use super::common::MessageStructMemberAstNode;

const NEWLINE: &'static str = "\n";

struct GenerationState {
    // Current indent.
    indent: usize,
}

impl GenerationState {
    fn new() -> GenerationState {
        GenerationState { indent: 0 }
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
            parser_generation::ragel::common::Ast::MachineHeader(ref node) => self
                .generate_machine_header(ast_node, buf_writer, &node.machine_name, generation_state,),
            parser_generation::ragel::common::Ast::MachineDefinition(ref node) => {
                self.generate_machine_definition(ast_node, buf_writer, &node, generation_state);
            }
            parser_generation::ragel::common::Ast::None => {
                self.generate_traverse_ast_node_children(ast_node, buf_writer, generation_state);
            }
            parser_generation::ragel::common::Ast::ParsingFunction(ref node) => {
                self.generate_parsing_function(ast_node, buf_writer, &node, generation_state);
            }
            parser_generation::ragel::common::Ast::MessageStruct(ref node) => {
                self.generate_message_struct(ast_node, buf_writer, &node, generation_state);
            }
            parser_generation::ragel::common::Ast::MessageStructMember(ref node) => {
                self.generate_message_struct_member(ast_node, buf_writer, node, generation_state);
            }
            parser_generation::ragel::common::Ast::RegexMachineField(ref node) => {
                self.generate_regex_machine_field_parser(ast_node, buf_writer, node, generation_state,);
            }
            parser_generation::ragel::common::Ast::MachineActionHook(ref node) => {
                self.generate_machine_action_hook(ast_node, buf_writer, node, generation_state,);
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
%%}}

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
        node: &parser_generation::ragel::common::MachineDefinitionAstNode,
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

        generation_state.indent -= 1;
        utility::string::write_line_with_indent_or_panic(
            buf_writer,
            generation_state.indent,
            "%%}".as_bytes(),
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
        node: &parser_generation::ragel::common::ParsingFunctionAstNode,
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
int cs;  // Current state -- Ragel-specific variable for C code generation

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
        node: &parser_generation::ragel::common::MessageStructAstNode,
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
        node: &parser_generation::ragel::common::MessageStructMemberAstNode,
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
        node: &parser_generation::ragel::common::MachineActionHookAstNode,
        generation_state: &mut GenerationState,
    ) {
        utility::string::write_with_indent_or_panic(buf_writer, generation_state.indent, format!(
"action {0} {{
}}
",
        node.name).as_bytes());
    }

    fn generate_regex_machine_field_parser<W: std::io::Write>(
        &self,
        ast_node: &parser_generation::ragel::common::AstNode,
        buf_writer: &mut std::io::BufWriter<W>,
        node: &parser_generation::ragel::common::RegexMachineFieldAstNode,
        generation_state: &mut GenerationState,
    ) {
        utility::string::write_line_with_indent_or_panic(buf_writer, generation_state.indent, format!(
"{0} = {1} @{0};
",
        node.name, node.string_sequence).as_bytes());
    }
}

impl parser_generation::Write for Generator<'_> {
    fn write<W: std::io::Write>(&self, buf_writer: &mut std::io::BufWriter<W>) {
        let mut generation_state = GenerationState::new();
        self.generate_traverse_ast_node(self.ast, buf_writer, &mut generation_state);
    }
}
