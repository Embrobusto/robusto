use std::io::Write;

const NEWLINE: &'static str = "\n";

/// Boilerplate reducer
pub fn write_line_with_indent_or_panic<W: std::io::Write>(
    buf_writer: &mut std::io::BufWriter<W>,
    indent: usize,
    line: &[u8]
) {
    for i in 0..indent {
        if let Err(_) = buf_writer.write("    ".as_bytes()) {
            log::error!("Failed to write into file, panicking!");
            panic!();
        }
    }

    if let Err(_) = buf_writer.write(line) {
        log::error!("Failed to write into file, panicking!");
        panic!();
    }

    if let Err(_) = buf_writer.write(NEWLINE.as_bytes()) {
        log::error!("Failed to write into file, panicking!");
        panic!();
    }
}
