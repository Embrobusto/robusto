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

/// Represents an abstract syntactic tree for Ragel code, with the difference
/// that its leaves mostly consist of snippets rather than atomic language
/// constructs, i.e. it is a less detailed representation of Ragel code.
///
#[derive(Debug)]
pub enum Ast {
    Sequence{blocks: std::vec::Vec<Ast>},
    MachineHeader{machine_name: std::string::String},
    ParsingFunction{
        user_context_struct_name: std::string::String,
    }
}

impl Ast {
    fn add_machine_header(&mut self, protocol: &bpir::representation::Protocol) {
        if let Ast::Sequence{ref mut blocks} = self {
            let root_message = protocol.root_message();

            blocks.push(Ast::MachineHeader{machine_name: root_message.name.clone()});

            return
        }

        log::error!("Unable to add machine header into a block of type {:?}", self);
        panic!();
    }

    fn add_parsing_function(&mut self, protocol: &bpir::representation::Protocol) {
        if let Ast::Sequence{ref mut blocks} = self {
            let root_message_name = protocol.root_message().name.clone();
            // TODO: add actions, and the rest of regular ragel stuff

            return
        }

        log::error!("Unable to add machine header into a block of type {:?}", self);
        panic!();
    }

    pub fn new_from_protocol(protocol: &bpir::representation::Protocol) -> Ast {
        let mut block = Ast::Sequence{blocks: std::vec::Vec::new()};
        block.add_machine_header(protocol);

        block
    }
}

