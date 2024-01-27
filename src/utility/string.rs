use std::io::Write;

const NEWLINE: &'static str = "\n";

pub fn append_indent_or_panic<T: std::fmt::Write>(
    sink: &mut T,
    indent: usize,
) {
    for i in 0..indent {
        if let Err(_) = write!(sink, "    ") {
            log::error!("Failed to write into file, panicking!");
            panic!();
        }
    }
}

/// Boilerplate reducer. Writes a line with indent. Panics when unable.
/// It is important to note that it will not seek for newline characters.
pub fn append_line_with_indent_or_panic<W: std::fmt::Write>(
    sink: &mut W,
    indent: usize,
    line: &[u8],
) {
    append_indent_or_panic(sink, indent);

    if let Err(_) = write!(sink, "{}", std::str::from_utf8(line).unwrap()) {
        log::error!("Failed to write into file, panicking!");
        panic!();
    }

    if let Err(_) = write!(sink, "{}", NEWLINE) {
        log::error!("Failed to write into file, panicking!");
        panic!();
    }
}

pub fn append_with_indent_or_panic<W: std::fmt::Write>(
    sink: &mut W,
    indent: usize,
    lines: &[u8],
) {
    let mut start_position = 0usize;
    let mut position = 0usize;
    let mut n_newlines = 0usize;

    append_indent_or_panic(sink, indent);

    for position in 0..lines.len() {
        if lines[position] == b'\r' || lines[position] == b'\n' {
            n_newlines += 1;
        } else if n_newlines > 0 {

            if let Err(_) = write!(sink, "{}", std::str::from_utf8(&lines[start_position..position]).unwrap()) {
                log::error!("Failed to write into file, panicking!");
                panic!();
            }

            append_indent_or_panic(sink, indent);

            start_position = position;
            n_newlines = 0;
        }
    }

    if (start_position < lines.len()) {
        if let Err(_) = write!(sink, "{}", std::str::from_utf8(&lines[start_position..lines.len()]).unwrap()) {
            log::error!("Failed to write into file, panicking!");
            panic!();
        }
    }
}

pub fn write_indent_or_panic<W: std::io::Write>(
    buf_writer: &mut std::io::BufWriter<W>,
    indent: usize,
) {
    for i in 0..indent {
        if let Err(_) = buf_writer.write("    ".as_bytes()) {
            log::error!("Failed to write into file, panicking!");
            panic!();
        }
    }
}

/// Boilerplate reducer. Writes a line with indent. Panics when unable.
/// It is important to note that it will not seek for newline characters.
pub fn write_line_with_indent_or_panic<W: std::io::Write>(
    buf_writer: &mut std::io::BufWriter<W>,
    indent: usize,
    line: &[u8],
) {
    write_indent_or_panic(buf_writer, indent);

    if let Err(_) = buf_writer.write(line) {
        log::error!("Failed to write into file, panicking!");
        panic!();
    }

    if let Err(_) = buf_writer.write(NEWLINE.as_bytes()) {
        log::error!("Failed to write into file, panicking!");
        panic!();
    }
}

pub fn write_with_indent_or_panic<W: std::io::Write>(
    buf_writer: &mut std::io::BufWriter<W>,
    indent: usize,
    lines: &[u8],
) {
    let mut start_position = 0usize;
    let mut position = 0usize;
    let mut n_newlines = 0usize;

    write_indent_or_panic(buf_writer, indent);

    for position in 0..lines.len() {
        if lines[position] == b'\r' || lines[position] == b'\n' {
            n_newlines += 1;
        } else if n_newlines > 0 {

            if let Err(_) = buf_writer.write(&lines[start_position..position]) {
                log::error!("Failed to write into file, panicking!");
                panic!();
            }

            write_indent_or_panic(buf_writer, indent);

            start_position = position;
            n_newlines = 0;
        }
    }

    if (start_position < lines.len()) {
        if let Err(_) = buf_writer.write(&lines[start_position..lines.len()]) {
            log::error!("Failed to write into file, panicking!");
            panic!();
        }
    }
}
