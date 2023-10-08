use std::str::FromStr;

/// This exapmple provides a test run of Ragel-based C code generation.
/// It uses raw BPIR which then is passed down the chain:
///
/// BPIR -> [ Ragel generator ] -> Ragel/C code -> [ Ragel ] -> Parser

use robusto::{self, parser_generation::Write};
use std;

const OUTPUT_FILE_NAME: &'static str = "output.c.rl";

fn make_message_bpir() -> robusto::bpir::representation::Message {
	let mut message = robusto::bpir::representation::Message{
		name: std::string::String::from("TestMessage"),
		fields: std::vec::Vec::<robusto::bpir::representation::Field>::new(),
		attributes: std::vec::Vec::<robusto::bpir::representation::MessageAttribute>::new(),
	};

	message.fields.push(robusto::bpir::representation::Field{
		name: std::string::String::from("preamble"),
		attributes: vec![
			robusto::bpir::representation::FieldAttribute::ConstSequence(vec![0xfe]),
		]
	});
	message.fields.push(robusto::bpir::representation::Field{
		name: std::string::String::from("payload"),
		attributes: vec![
			robusto::bpir::representation::FieldAttribute::Length(3),
		]
	});

	message
}

fn main() {
	use robusto::parser_generation::Generate;
	use std::io::Write;

	let protocol = robusto::bpir::representation::Protocol{messages: vec![make_message_bpir()]};
	let file = std::fs::File::create(OUTPUT_FILE_NAME).unwrap();
	let mut buf_writer = std::io::BufWriter::new(file);
	let ast = robusto::parser_generation::ragel::common::AstNode::from_protocol(&protocol);
	let c_generator = robusto::parser_generation::ragel::c::Generator::from_ragel_ast(&ast);
	c_generator.write(&mut buf_writer);
	// c_ast.generate(&mut buf_writer);
}
