pub mod ragel;
use std;

pub trait Generate {
	fn generate<W: std::io::Write>(&self, buf_writer: &mut std::io::BufWriter<W>);
}
