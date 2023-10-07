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
#[derive(Debug)]
pub enum Ast {
    // C-specific elements (TBD)

    // Language-agnostic elements (TBD)

    /// Generic sequence of AST nodes
    Sequence{blocks: std::vec::Vec<Ast>},

    /// Ragel-specific machine header
    MachineHeader{machine_name: std::string::String},

    /// Entry point to the parser
    ParsingFunction {
        /// Name of the message which the parsing function is associated with
        message_name: std::string::String,
    }
}

impl Ast {
    fn add_machine_header(&mut self, message: &bpir::representation::Message) {
        if let Ast::Sequence{ref mut blocks} = self {
            blocks.push(Ast::MachineHeader{machine_name: message.name.clone()});

            return
        }

        log::error!("Unable to add machine header into a block of type {:?}", self);
        panic!();
    }

    fn add_parsing_function(&mut self, message: &bpir::representation::Message) {
        if let Ast::Sequence{ref mut blocks} = self {
            let message_name = message.name.clone();
            blocks.push(Ast::ParsingFunction{
                message_name: message.name.clone(),
            });
            // TODO: parsing function will require a bit more than that

            return
        }

        log::error!("Unable to add machine header into a block of type {:?}", self);
        panic!();
    }

    pub fn new_from_protocol(protocol: &bpir::representation::Protocol) -> Ast {
        let mut block = Ast::Sequence{blocks: std::vec::Vec::new()};

        for message in &protocol.messages {
            // Add machine header
            block.add_machine_header(message);
            block.add_parsing_function(message);
        }


        block
    }
}
