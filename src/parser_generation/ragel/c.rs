/// Generates C code from procedural representation
use crate::bpir;
use crate::parser_generation;
use log;
use std;
use std::fmt::write;
use std::io::Write;
use std::str::FromStr;

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

/// Boilerplate reducer
fn write_line_with_indent_or_panic<W: std::io::Write>(
    buf_writer: &mut std::io::BufWriter<W>,
    indent: usize,
    line: std::string::String,
) {
    for i in 0..indent {
        if let Err(_) = buf_writer.write("    ".as_bytes()) {
            log::error!("Failed to write into file, panicking!");
            panic!();
        }
    }

    if let Err(_) = buf_writer.write(line.as_bytes()) {
        log::error!("Failed to write into file, panicking!");
        panic!();
    }

    if let Err(_) = buf_writer.write(NEWLINE.as_bytes()) {
        log::error!("Failed to write into file, panicking!");
        panic!();
    }
}

impl Generator<'_> {
    pub fn from_ragel_ast(ast_node: &parser_generation::ragel::common::AstNode) -> Generator {
        Generator { ast: ast_node }
    }

    fn generate_traverse_ast_node<W: std::io::Write>(
        &self,
        ast_node: &parser_generation::ragel::common::AstNode,
        buf_writer: &mut std::io::BufWriter<W>,
        generation_state: &mut GenerationState,
    ) {
        match ast_node.ast_node_type {
            parser_generation::ragel::common::Ast::MachineHeader { ref machine_name } => {
                self.generate_machine_name(ast_node, buf_writer, machine_name, generation_state)
            }
            parser_generation::ragel::common::Ast::None => {
                for ast_node_child in &ast_node.children {
                    self.generate_traverse_ast_node(ast_node_child, buf_writer, generation_state);
                }
            }
            parser_generation::ragel::common::Ast::ParsingFunction { ref message_name } => {
                self.generate_parsing_function(
                    ast_node,
                    buf_writer,
                    message_name,
                    generation_state,
                );
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

    fn generate_machine_name<W: std::io::Write>(
        &self,
        ast_node: &parser_generation::ragel::common::AstNode,
        buf_writer: &mut std::io::BufWriter<W>,
        machine_name: &std::string::String,
        generation_state: &mut GenerationState,
    ) {
        let write_result = buf_writer.write_fmt(format_args!(
            "%%{{
    machine {machine_name};
    write data;
%%}}
"
        ));

        if let Err(_) = write_result {
            log::error!("Could not write into file, panicking!");
            panic!();
        }
    }

    fn generate_parsing_function<W: std::io::Write>(
        &self,
        ast_node: &parser_generation::ragel::common::AstNode,
        buf_writer: &mut std::io::BufWriter<W>,
        message_name: &std::string::String,
        generation_state: &mut GenerationState,
    ) {
        write_line_with_indent_or_panic(
            buf_writer,
            generation_state.indent,
            format!("void parse{message_name}(const char *aInputBuffer, int aInputBufferLength, struct {message_name} *a{message_name})")
        );
        write_line_with_indent_or_panic(
            buf_writer,
            generation_state.indent,
            std::string::String::from_str("{").unwrap(),
        );
        write_line_with_indent_or_panic(
            buf_writer,
            generation_state.indent,
            format!( "const char *p = aInputBuffer;  // Iterator \"begin\" pointer -- Ragel-specific variable for C code generation"),
        );
        write_line_with_indent_or_panic(
            buf_writer,
            generation_state.indent,
            format!("const char *pe = aInputBuffer + aInputBufferLength;  // Iterator \"end\" pointer -- Ragel-specific variable for C code generation"),
        );
        write_line_with_indent_or_panic(
            buf_writer,
            generation_state.indent,
            format!( "int cs;  // Current state -- Ragel-specific variable for C code generation"),
        );
    }
}

impl parser_generation::Write for Generator<'_> {
    fn write<W: std::io::Write>(&self, buf_writer: &mut std::io::BufWriter<W>) {
        let mut generation_state = GenerationState::new();
        self.generate_traverse_ast_node(self.ast, buf_writer, &mut generation_state);
    }
}
