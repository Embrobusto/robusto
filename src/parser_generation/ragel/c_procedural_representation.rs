/// Generates C code from procedural representation

use crate::bpir;
use crate::parser_generation;
use std;

struct GenerationState {
    /// Current indentation level
    indent_level: usize,

    indent_sequence: &'static str,
    newline_sequence: &'static str,
}

impl GenerationState {
    fn new() -> GenerationState {
        GenerationState{
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
pub struct CAst {
    ast: parser_generation::ragel::procedural_representation::Ast
}

impl CAst {
    pub fn new_from_protocol(protocol: &bpir::representation::Protocol) -> CAst {
        CAst{
            ast: parser_generation::ragel::procedural_representation::Ast::new_from_protocol(protocol),
        }
    }

    fn generate_impl<W: std::io::Write>(generation_state: &mut GenerationState,
        ast: &parser_generation::ragel::procedural_representation::Ast,
        buf_writer: &mut std::io::BufWriter<W>
    ) {
        use parser_generation::ragel::procedural_representation::Ast;
        use std::io::Write;

        // Convert AST into the actual Ragel+C code
        match ast {
            Ast::Sequence{ref blocks} => {
                for block in blocks {
                    CAst::generate_impl(generation_state, block, buf_writer);
                }
            },
            Ast::MachineHeader{machine_name} => {
                buf_writer.write_fmt(format_args!(
"%%{{
    machine {machine_name};
    write data;
%%}}

"
                ));
            },
            Ast::ParsingFunction => {
                buf_writer.write_fmt(format_args!(
"void parse(char *string, char *length)
{{
}}"
                ));
            },
            _ => {}
        }
    }
}

impl parser_generation::Generate for CAst {
    fn generate<W: std::io::Write>(&self, buf_writer: &mut std::io::BufWriter<W>) {
        let mut generation_state = GenerationState::new();
        CAst::generate_impl(&mut generation_state, &self.ast, buf_writer);
    }
}
