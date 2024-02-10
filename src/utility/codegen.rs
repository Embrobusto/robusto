use std::collections::LinkedList;

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
		CodeChunk {code, indent, newlines }
	}
}

pub trait CodeGeneration {
    fn generate_code(
        &self,
        code_generation_state: &mut CodeGenerationState,
    ) -> LinkedList<CodeChunk>;
}
