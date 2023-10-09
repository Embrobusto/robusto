pub mod ragel;
use std;

pub trait Write {
    fn write<W: std::io::Write>(&self, buf_writer: &mut std::io::BufWriter<W>);
}
