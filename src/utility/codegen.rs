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
    ) -> LinkedList<CodeChunk>
    {
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


/// Will DFS-traverse the tree, first invoking its `generate_code` method, then
/// that of its children, and finally `generate_code_post_iter`.
///
/// The `T` type most provide the behavior of an AST.
pub fn generate_from_ast<T>(ast: &mut T) -> LinkedList<CodeChunk>
where
    T: CodeGeneration + Iterator<Item = dyn CodeGeneration>,
    T::Item: Sized,
{
    let mut code_generation_state = CodeGenerationState::new();
    let mut ret = ast.generate_code(&mut code_generation_state);

    for child in &mut *ast {
        ret.append(&mut child.generate_code(&mut code_generation_state));
    }

    ret.append(&mut ast.generate_code_post_iter(&mut code_generation_state));

    ret
}

// TODO: `struct Ast` for code chunks


pub struct MockCodeGenerator {
}

impl CodeGeneration for MockCodeGenerator {
}
