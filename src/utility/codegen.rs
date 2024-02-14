use crate::parser_generation;
use crate::utility::string::write_newlines_or_panic;
use std::array::IntoIter;
use std::collections::LinkedList;
use std::io::{BufWriter, Write};
use std::iter::Iterator;

pub struct CodeGenerationState {
    // Current indent.
    pub indent: usize,
}

impl CodeGenerationState {
    fn new() -> CodeGenerationState {
        CodeGenerationState { indent: 0 }
    }
}

pub struct CodeChunk {
    code: String,

    /// Indents in the code chunk's lines
    indent: usize,

    /// Number of new lines to add after the chunk
    newlines: usize,
}

impl CodeChunk {
    pub fn new(code: String, indent: usize, newlines: usize) -> CodeChunk {
        CodeChunk {
            code,
            indent,
            newlines,
        }
    }
}

pub trait CodeGeneration {
    fn generate_code(
        &self,
        code_generation_state: &mut CodeGenerationState,
    ) -> LinkedList<CodeChunk> {
        LinkedList::<CodeChunk>::new()
    }

    /// A hook which gets invoked after the AST's children have been traversed.
    /// Usually it is used for generating content nested in brackets of some
    /// sort, such as struct members. The implementation may be omitted, if a
    /// node is only supposed to be used as a leaf.
    fn generate_code_post_iter(
        &self,
        code_generation_state: &mut CodeGenerationState,
    ) -> LinkedList<CodeChunk> {
        LinkedList::<CodeChunk>::new()
    }
}

impl<T: CodeGeneration> parser_generation::Write for T
where
    T: CodeGeneration,
{
    fn write<W: std::io::Write>(&self, buf_writer: &mut std::io::BufWriter<W>) {
        use crate::utility::string::write_with_indent_or_panic;
        let mut code_generation_state = CodeGenerationState::new();

        for code_chunk in &self.generate_code(&mut code_generation_state) {
            write_with_indent_or_panic(
                buf_writer,
                code_generation_state.indent,
                code_chunk.code.as_bytes(),
            );
            write_newlines_or_panic(buf_writer, code_chunk.newlines);
        }
    }
}

pub struct MockCodeGenerator {}

impl CodeGeneration for MockCodeGenerator {}
