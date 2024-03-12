use crate::parser_generation;
use crate::utility::string::write_newlines_or_panic;
use std::alloc::handle_alloc_error;
use std::array::IntoIter;
use std::collections::{linked_list, LinkedList};
use std::io::{BufWriter, Write};
use std::iter::Iterator;
use std::path::Iter;
use std::str::FromStr;
use std::string::String;

/// Precompiled code
#[derive(Clone, Debug)]
pub struct RawCode {
    code_chunk_pre_traverse: LinkedList<CodeChunk>,
    code_chunk_post_traverse: LinkedList<CodeChunk>,

    /// A diff for further increment.
    ///
    /// The `RawCode` instance will be traversed through later, so by the moment
    /// the code is generated, `CodeGenerationState` does not represent an actual
    /// state.
    indent_increment: isize,
}

impl From<&str> for RawCode {
    fn from(value: &str) -> Self {
        let mut ret = LinkedList::new();
        ret.push_back(CodeChunk {
            code: value.into(),
            indent: 0usize,
            newlines: 1usize,
        });

        RawCode {
            code_chunk_pre_traverse: ret,
            code_chunk_post_traverse: LinkedList::new(),
            indent_increment: 0isize,
        }
    }
}

impl<T: TreeBasedCodeGeneration> From<&T> for RawCode {
    fn from(value: &T) -> Self {
        let mut code_generation_state = CodeGenerationState::new();
        let code_chunk_pre_traverse = value.generate_code_pre_traverse(&mut code_generation_state);
        let traverse_indent = code_generation_state.indent;
        let code_chunk_post_traverse = value.generate_code_post_traverse(&mut code_generation_state);
        let indent_increment = traverse_indent as isize - code_generation_state.indent as isize;

        RawCode {
            code_chunk_pre_traverse,
            code_chunk_post_traverse,
            indent_increment,
        }
    }
}

impl TreeBasedCodeGeneration for RawCode {
    fn generate_code_pre_traverse(
        &self,
        code_generation_state: &mut CodeGenerationState,
    ) -> LinkedList<CodeChunk> {
        // Heuristic: apply whatever indent was used during creation of the object + the current indent
        // TODO: won't fit for the negative indents
        let mut ret = self.code_chunk_pre_traverse
            .iter()
            .map(|chunk| CodeChunk {
                code: chunk.code.clone(),
                indent: chunk.indent + code_generation_state.indent,
                newlines: chunk.newlines,
            })
            .collect();
        code_generation_state.increment_indent(self.indent_increment);

        ret
    }

    fn generate_code_post_traverse(
        &self,
        code_generation_state: &mut CodeGenerationState,
    ) -> LinkedList<CodeChunk> {
        code_generation_state.increment_indent(-self.indent_increment);
        self.code_chunk_post_traverse
            .iter()
            .map(|chunk| CodeChunk {
                code: chunk.code.clone(),
                indent: chunk.indent + code_generation_state.indent,
                newlines: chunk.newlines,
            })
            .collect()
    }
}

pub struct CodeGenerationState {
    // Current indent.
    pub indent: usize,
}

impl CodeGenerationState {
    fn new() -> CodeGenerationState {
        CodeGenerationState { indent: 0 }
    }

    fn increment_indent(&mut self, increment: isize) {
        if increment < 0isize && self.indent < increment.abs() as usize {
            log::warn!(
                "Indent value is less than 0, current indent: {0}, increment: {1}",
                self.indent,
                increment
            );
            self.indent = 0;
        } else {
            self.indent = (self.indent as isize + increment) as usize;
        }
    }
}

#[derive(Clone, Debug)]
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

pub trait TreeBasedCodeGeneration {
    fn generate_code_pre_traverse(
        &self,
        code_generation_state: &mut CodeGenerationState,
    ) -> LinkedList<CodeChunk>;

    /// A hook which gets invoked after the AST's children have been traversed.
    /// Usually it is used for generating content nested in brackets of some
    /// sort, such as struct members. The implementation may be omitted, if a
    /// node is only supposed to be used as a leaf.
    fn generate_code_post_traverse(
        &self,
        code_generation_state: &mut CodeGenerationState,
    ) -> LinkedList<CodeChunk> {
        LinkedList::<CodeChunk>::new()
    }
}

pub trait SubnodeAccess<T: CodeGeneration> {
    fn iter(&self) -> std::slice::Iter<'_, T>;
}

pub trait CodeGeneration {
    fn generate_code(
        &self,
        code_generation_state: &mut CodeGenerationState,
    ) -> LinkedList<CodeChunk>;
}

impl<T> CodeGeneration for T
where
    T: SubnodeAccess<T> + TreeBasedCodeGeneration,
{
    fn generate_code(
        &self,
        code_generation_state: &mut CodeGenerationState,
    ) -> LinkedList<CodeChunk> {
        let mut ret = LinkedList::new();
        ret.append(&mut self.generate_code_pre_traverse(code_generation_state));

        for subnode in self.iter() {
            ret.append(&mut subnode.generate_code(code_generation_state));
        }

        ret.append(&mut self.generate_code_post_traverse(code_generation_state));

        ret
    }
}

impl<T: CodeGeneration> parser_generation::Write for T {
    fn write<W: std::io::Write>(&self, buf_writer: &mut std::io::BufWriter<W>) {
        use crate::utility::string::write_with_indent_or_panic;
        let mut code_generation_state = CodeGenerationState::new();

        for code_chunk in self.generate_code(&mut code_generation_state).iter() {
            write_with_indent_or_panic(buf_writer, code_chunk.indent, code_chunk.code.as_bytes());
            write_newlines_or_panic(buf_writer, code_chunk.newlines);
        }
    }
}

pub struct MockCodeGenerator {}

impl TreeBasedCodeGeneration for MockCodeGenerator {
    fn generate_code_pre_traverse(
        &self,
        code_generation_state: &mut CodeGenerationState,
    ) -> LinkedList<CodeChunk> {
        LinkedList::new()
    }
}
