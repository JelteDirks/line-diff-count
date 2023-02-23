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

    let mut map_a = HashMap::<String, Comparison>::new();
    let mut map_b = HashMap::<String, Comparison>::new();

    for line in buf_a.lines() {
        let line_str: String = line.unwrap();
        map_a
            .entry(line_str)
            .and_modify(|item: &mut Comparison| item.value = item.value + 1)
            .or_insert(Comparison::with_value(1));
    }

    for line in buf_b.lines() {
        let line_str: String = line.unwrap();
        map_b
            .entry(line_str)
            .and_modify(|item: &mut Comparison| item.value = item.value + 1)
            .or_insert(Comparison::with_value(1));
    }

    compare_map_results(&mut map_a, &mut map_b);

    let target = args_iter.next();
    let mut stream_out = BufWriter::new(stdout());

    if target.is_some() {
        let handle = File::create(target.unwrap());
        if handle.is_err() {
            write!(stderr_writer, "problem creating file for writing: ").unwrap();
            write!(stderr_writer, "{}\n", handle.err().unwrap().to_string()).unwrap();
            stderr_writer.flush().unwrap();
            exit(5);
        }
        let mut file_out = BufWriter::new(handle.unwrap());
        print_diff(&mut map_a, &mut map_b, &mut file_out);
    } else {
        print_diff(&mut map_a, &mut map_b, &mut stream_out);
    }

    stderr_writer.flush().unwrap();
    stream_out.flush().unwrap();
}

fn print_diff<T: std::io::Write>(
    map_a: &mut HashMap<String, Comparison>,
    map_b: &mut HashMap<String, Comparison>,
    stream: &mut BufWriter<T>,
) {
    write!(stream, "file a results: \n").unwrap();
    for a in map_a.iter() {
        let (k, c): (&String, &Comparison) = a;
        write!(stream, "{}\t{}\t{}\n", c.kind, c.value, k).unwrap();
    }

    write!(stream, "file b results: \n").unwrap();
    for b in map_b.iter() {
        let (k, c): (&String, &Comparison) = b;
        write!(stream, "{}\t{}\t{}\n", c.kind, c.value, k).unwrap();
    }
}

struct Comparison {
    value: u32,
    kind: String,
}

impl Comparison {
    fn with_value(value: u32) -> Comparison {
        return Comparison {
            value,
            kind: "++".to_string(),
        };
    }
}

fn compare_map_results(
    map_a: &mut HashMap<String, Comparison>,
    map_b: &mut HashMap<String, Comparison>,
) {
    for entries_a in map_a.iter_mut() {
        let (key_a, val_a): (&String, &mut Comparison) = entries_a;

        let b_entry = map_b.get_mut(key_a);

        if b_entry.is_none() {
            continue;
        }

        let val_b: &mut Comparison = b_entry.unwrap();

        if val_a.value > val_b.value {
            val_a.value -= val_b.value;
            val_a.kind = "+".to_string();
            val_b.value = 0;
        } else if val_b.value > val_a.value {
            val_b.value -= val_a.value;
            val_b.kind = "+".to_string();
            val_a.value = 0;
        } else {
            val_a.value = 0;
            val_b.value = 0;
        }
    }

    map_a.retain(|_, c| c.value != 0);
    map_b.retain(|_, c| c.value != 0);
}
