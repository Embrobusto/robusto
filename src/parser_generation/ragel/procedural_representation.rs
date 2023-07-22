/// Generates an AST-like tree of patterns common for languages supporting
/// procedural paradigm (Rust 2018, and ANSI C at this point).
///
/// To qualify for using this representation, a language must have the following
/// features:
///
/// - Support for C-like `struct`s;
/// - Support for functions accepting mutable pointers;

pub use std;
use crate::bpir;

pub enum Block {
    Block{blocks: std::vec::Vec<Block>},
}

pub fn generate(protocol: &bpir::representation::Protocol) -> Block {
    let mut block = Block::Block{blocks: std::vec::Vec::new()};

    block
}
