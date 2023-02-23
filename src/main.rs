use std::collections::HashMap;
use std::env::args;
use std::fs::File;
use std::io::stderr;
use std::io::stdout;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Write;
use std::process::exit;

fn main() {
    let mut stdout_writer = BufWriter::new(stdout());
    let mut stderr_writer = BufWriter::new(stderr());

    let mut args_iter = args().skip(1);

    let file_a = args_iter.next();

    if file_a.is_none() {
        write!(stderr_writer, "no files specified\n").unwrap();
        stderr_writer.flush().unwrap();
        exit(1);
    }

    let file_b = args_iter.next();

    if file_b.is_none() {
        write!(stderr_writer, "only one file specified\n").unwrap();
        stderr_writer.flush().unwrap();
        exit(2);
    }

    write!(stderr_writer, "comparing {:?} with {:?}\n", file_a, file_b).unwrap();

    let file_a_handle = File::open(file_a.unwrap());

    if file_a_handle.is_err() {
        write!(stderr_writer, "error opening file a\n").unwrap();
        write!(
            stderr_writer,
            "{:?}\n",
            file_a_handle.err().unwrap().to_string()
        )
        .unwrap();
        stderr_writer.flush().unwrap();
        exit(3);
    }

    let file_b_handle = File::open(file_b.unwrap());

    if file_b_handle.is_err() {
        write!(stderr_writer, "error opening file b\n").unwrap();
        write!(
            stderr_writer,
            "{:?}\n",
            file_b_handle.err().unwrap().to_string()
        )
        .unwrap();
        stderr_writer.flush().unwrap();
        exit(4);
    }

    let buf_a = BufReader::new(file_a_handle.unwrap());
    let buf_b = BufReader::new(file_b_handle.unwrap());

    let mut map_a = HashMap::<String, u32>::new();
    let mut map_b = HashMap::<String, u32>::new();

    for line in buf_a.lines() {
        let line_str: String = line.unwrap();
        map_a
            .entry(line_str)
            .and_modify(|item| *item = *item + 1)
            .or_insert(1);
    }

    for line in buf_b.lines() {
        let line_str: String = line.unwrap();
        map_b
            .entry(line_str)
            .and_modify(|item| *item = *item + 1)
            .or_insert(1);
    }

    // compare results from a and b

    stderr_writer.flush().unwrap();
    stdout_writer.flush().unwrap();
}
