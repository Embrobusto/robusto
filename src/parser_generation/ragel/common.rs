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

    // Language-agnostic elements

    /// Just treat it as a mere sequence
    None,

    /// Generic sequence of AST nodes
    Sequence{
        blocks: std::vec::Vec<Ast>
    },

    /// Ragel-specific machine header
    MachineHeader{
        machine_name: std::string::String
    },

    /// Entry point to the parser
    ParsingFunction {
        /// Name of the message which the parsing function is associated with
        message_name: std::string::String,
    }
}

pub struct AstNode {
    pub ast_node_type: Ast,
    pub children: std::vec::Vec<AstNode>,
}

impl AstNode {
    pub fn from_protocol(protocol: &bpir::representation::Protocol) -> AstNode {
        let mut root = AstNode{
            ast_node_type: Ast::None,
            children: vec![]
        };
        for message in &protocol.messages {
            root.add_message_parser(message);
        }

        root
    }

    /// Adds a new child to a node. Returns reference to the new child
    fn add_child(&mut self, ast_node_type: Ast) -> &mut AstNode {
        let child = AstNode {
            ast_node_type,
            children: vec![],
        };
        self.children.push(child);

        self.children.last_mut().unwrap()
    }

    fn add_message_parser(&mut self, message: &bpir::representation::Message) {
        self.add_child(Ast::MachineHeader{
            machine_name: message.name.clone(),
        });
        let mut parsing_function = self.add_child(Ast::ParsingFunction{
            message_name: message.name.clone()
        });
    }
}
