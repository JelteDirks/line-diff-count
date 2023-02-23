use std::env::args;
use std::io::BufWriter;
use std::io::stderr;
use std::io::stdout;
use std::io::Write;
use std::process::exit;

fn main() {
    let mut stdout_writer = BufWriter::new(stdout());
    let mut stderr_writer = BufWriter::new(stderr());

    let mut args_iter = args().skip(1);

    let file_a = args_iter.next();
    
    if file_a.is_none() {
        write!(stderr_writer, "no files specified\n").unwrap();
        exit(1);
    }

    let file_b = args_iter.next();

    if file_b.is_none() {
        write!(stderr_writer, "only one file specified\n").unwrap();
        exit(2);
    }

    write!(stdout_writer, "comparing {:?} with {:?}\n", file_a, file_b).unwrap();
}
