/// Generates C code from procedural representation
use crate::bpir;
use crate::parser_generation;
use log;
use std;
use std::io::Write;

struct GenerationState {
    /// Current indentation level
    indent_level: usize,

    indent_sequence: &'static str,
    newline_sequence: &'static str,
}

impl GenerationState {
    fn new() -> GenerationState {
        GenerationState {
            indent_level: 0,
            indent_sequence: "\t",
            newline_sequence: "\n",
        }
    }

    fn write_indent<W: std::io::Write>(&self, buf_writer: &mut std::io::BufWriter<W>) {
        use std::io::Write;

        for i in 0..self.indent_level {
            buf_writer.write(self.indent_sequence.as_bytes());
        }
    }

    fn write_newline<W: std::io::Write>(&self, buf_writer: &mut std::io::BufWriter<W>) {
        use std::io::Write;
        buf_writer.write(self.newline_sequence.as_bytes());
    }
}

/// C-specific Ragel AST
pub struct Generator<'a> {
    ast: &'a parser_generation::ragel::common::AstNode,
}

impl Generator<'_> {
    pub fn from_ragel_ast(ast_node: &parser_generation::ragel::common::AstNode) -> Generator {
        Generator {
            ast: ast_node,
        }
    }

    fn generate_traverse_ast_node<W: std::io::Write>(
        &self,
        ast_node: &parser_generation::ragel::common::AstNode,
        buf_writer: &mut std::io::BufWriter<W>,
    ) {
        match ast_node.ast_node_type {
            parser_generation::ragel::common::Ast::MachineHeader { ref machine_name } => {
                self.generate_machine_name(ast_node, buf_writer, machine_name)
            },
            parser_generation::ragel::common::Ast::None => {
                for ast_node_child in &ast_node.children {
                    self.generate_traverse_ast_node(ast_node_child, buf_writer);
                }
            },
            parser_generation::ragel::common::Ast::ParsingFunction { ref message_name } => {
                self.generate_parsing_function(ast_node, buf_writer, message_name);
            },
            _ => {
                log::error!("Unmatched node \"{:?}\", panicking!", ast_node.ast_node_type);
                panic!();
            }
        }
    }

    fn generate_machine_name<W: std::io::Write>(
        &self,
        ast_node: &parser_generation::ragel::common::AstNode,
        buf_writer: &mut std::io::BufWriter<W>,
        machine_name: &std::string::String,
    ) {
        buf_writer.write_fmt(format_args!(
"%%{{
    machine {machine_name};
    write data;
%%}}"
        ));
    }

    fn generate_parsing_function<W: std::io::Write>(
        &self,
        ast_node: &parser_generation::ragel::common::AstNode,
        buf_writer: &mut std::io::BufWriter<W>,
        message_name: &std::string::String
    ) {
        buf_writer.write_fmt(format_args!(
"
void parse{message_name}(const char *aInputBuffer, int aInputBufferLength, struct {message_name} *a{message_name})
{{
}}
"
        ));
    }
}

impl parser_generation::Write for Generator<'_> {
    fn write<W: std::io::Write>(&self, buf_writer: &mut std::io::BufWriter<W>) {
        self.generate_traverse_ast_node(self.ast, buf_writer);
    }
}
