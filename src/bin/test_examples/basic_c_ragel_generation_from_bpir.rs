use std::str::FromStr;

use env_logger;
use robusto::utility;
/// This exapmple provides a test run of Ragel-based C code generation.
/// It uses raw BPIR which then is passed down the chain:
///
/// BPIR -> [ Ragel generator ] -> Ragel/C code -> [ Ragel ] -> Parser
use robusto::{self, parser_generation::Write};
use std;

const OUTPUT_FILE_NAME: &'static str = "output.c.rl";
const OUTPUT_HEADER_FILE_NAME: &'static str = "output.h.rl";

fn make_message_bpir() -> robusto::bpir::representation::Message {
    let mut message = robusto::bpir::representation::Message {
        name: std::string::String::from("TestMessage"),
        fields: std::vec::Vec::<robusto::bpir::representation::Field>::new(),
        attributes: std::vec::Vec::<robusto::bpir::representation::MessageAttribute>::new(),
    };

    message.fields.push(robusto::bpir::representation::Field {
        name: std::string::String::from("preamble"),
        field_type: robusto::bpir::representation::FieldType::Regex(
            robusto::bpir::representation::RegexFieldType {
                regex: "\\xfe".to_string(),
            },
        ),
        attributes: vec![
            robusto::bpir::representation::FieldAttribute::MaxLength(
                robusto::bpir::representation::MaxLengthFieldAttribute{value: 64usize}
            ),
        ]
    });

    message
}

fn main() {
    // Initialize logging
    env_logger::init();

    // Create a simple BPIR
    let protocol = robusto::bpir::representation::Protocol {
        messages: vec![make_message_bpir()],
        attributes: vec![],
    };
    robusto::bpir::validation::validate_protocol(&protocol);

    // Run Ragel code generation
    let file = std::fs::File::create(OUTPUT_FILE_NAME).unwrap();
    let mut buf_writer = std::io::BufWriter::new(file);
    // let ast = robusto::parser_generation::ragel::common::AstNode::from_protocol(&protocol);
    let mut c_ast = robusto::parser_generation::ragel::c::SourceAstNode::from(&protocol);
    c_ast.write(&mut buf_writer);

    let mut c_header_ast = robusto::parser_generation::ragel::c::HeaderAstNode::from(&protocol);
    let header_file = std::fs::File::create(OUTPUT_HEADER_FILE_NAME).unwrap();
    let mut header_buf_writer = std::io::BufWriter::new(header_file);
    c_header_ast.write(&mut header_buf_writer);
}
