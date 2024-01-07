//! Embedded binary protocol serializer / deserializer generation library.
//!
//! Robusto is a Rust-based framework for generating binary protocol serializers
//! and deserializers. Among its functions, the primary one is to provide a
//! convenient frontent to [Ragel](https://github.com/adrian-thurston/ragel)
//! which generates an intermediate languange-dependent code which then gets
//! processed by Ragel for generating a resulting parser.
//!
//! All serial embedded binary protocols share a great deal of similarities.
//! Robusto exploits this feature to off-load this responsibility from embedded
//! programmers. Robusto's mission is to provide one-stop solution for embedded
//! binary protocol implementation, to cut-off the risks and time expenses
//! entailed by hand-writing binary protocol-parsing / marshalling boilerplate
//! code.
//!
//! Projects similar to Robusto:
//! - https://docs.zeek.org/projects/spicy/en/latest/index.html
//! - https://www.protlr.com/
//! - https://github.com/commschamp/comms_champion
//! - https://github.com/google/wuffs
//! - https://doc.kaitai.io/faq.html

pub mod parser_generation;
pub mod bpir;
mod utility;
