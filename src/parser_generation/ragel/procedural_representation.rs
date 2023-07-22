/// Generates an AST-like tree of patterns common for languages supporting
/// procedural paradigm (Rust 2018, and ANSI C at this point).
///
/// To qualify for using this representation, a language must have the following
/// features:
///
/// - Support for C-like `struct`s;
/// - Support for functions;
/// - Support for mutable pointers or similar entities;

pub use std;
use crate::bpir;
use log;

#[derive(Debug)]
pub enum Block {
    Block{blocks: std::vec::Vec<Block>},
    MachineHeader{machine_name: std::string::String},
    ParsingFunction{
        user_context_struct_name: std::string::String,
    }
}

impl Block {
    fn add_machine_header(&mut self, protocol: &bpir::representation::Protocol) {
        if let Block::Block{ref mut blocks} = self {
            let root_message = protocol.root_message();

            blocks.push(Block::MachineHeader{machine_name: root_message.name.clone()});

            return
        }

        log::error!("Unable to add machine header into a block of type {:?}", self);
        panic!();
    }

    fn add_parsing_function(&mut self, protocol: &bpir::representation::Protocol) {
        if let Block::Block{ref mut blocks} = self {
            let root_message_name = protocol.root_message().name.clone();
            // TODO: add actions, and the rest of regular ragel stuff

            return
        }

        log::error!("Unable to add machine header into a block of type {:?}", self);
        panic!();
    }

    pub fn new_from_protocol(protocol: &bpir::representation::Protocol) -> Block {
        let mut block = Block::Block{blocks: std::vec::Vec::new()};
        block.add_machine_header(protocol);

        block
    }
}

