/// Generates C code from procedural representation
use crate::bpir;
use crate::parser_generation;
use log;
use std;
use std::io::Write;

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
%%}}
"
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
    const char *p = aInputBuffer;  // Iterator \"begin\" pointer -- Ragel-specific variable for C code generation
    const char *pe = aInputBuffer + aInputBufferLength;  // Iterator \"end\" pointer -- Ragel-specific variable for C code generation
    int cs;  // Current state -- Ragel-specific variable for C code generation
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
