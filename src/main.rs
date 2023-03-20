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
    // create a handle to write errors to
    let mut stderr_writer = BufWriter::new(stderr());

    // arguments iterator
    let mut args_iter = args().skip(1);

    // first argument is the first file (file a)
    let file_a = args_iter.next();

    if file_a.is_none() {
        write!(stderr_writer, "no files specified\n").unwrap();
        stderr_writer.flush().unwrap();
        exit(1);
    }

    // second argument is the second file (file b)
    let file_b = args_iter.next();

    if file_b.is_none() {
        write!(stderr_writer, "only one file specified\n").unwrap();
        stderr_writer.flush().unwrap();
        exit(2);
    }

    let file_a_handle = File::open(&file_a.clone().unwrap());

    if file_a_handle.is_err() {
        write!(stderr_writer, "error opening {}\n", file_a.unwrap()).unwrap();
        write!(
            stderr_writer,
            "{:?}\n",
            file_a_handle.err().unwrap().to_string()
        )
        .unwrap();
        stderr_writer.flush().unwrap();
        exit(3);
    }

    let file_b_handle = File::open(file_b.clone().unwrap());

    if file_b_handle.is_err() {
        write!(stderr_writer, "error opening {}\n", file_b.unwrap()).unwrap();
        write!(
            stderr_writer,
            "{:?}\n",
            file_b_handle.err().unwrap().to_string()
        )
        .unwrap();
        stderr_writer.flush().unwrap();
        exit(4);
    }

    // optionally, a target file to write to, else it will be stdout
    let target = args_iter.next();

    let mut stream_out: BufWriter<Box<dyn std::io::Write>> = match target {
        Some(filename) => {
            let handle = File::create(filename);
            if handle.is_err() {
                write!(stderr_writer, "problem opening file for writing: ").unwrap();
                write!(stderr_writer, "{}\n", handle.err().unwrap().to_string()).unwrap();
                stderr_writer.flush().unwrap();
                exit(5);
            }
            BufWriter::new(Box::new(handle.unwrap()))
        }
        None => BufWriter::new(Box::new(stdout())),
    };

    // create buffered readers to read from the input files line by line
    // with better optimization
    let buf_a = BufReader::new(file_a_handle.unwrap());
    let buf_b = BufReader::new(file_b_handle.unwrap());

    // create the maps that will be counting the lines in each file
    let mut map_a = HashMap::<String, Comparison>::new();
    let mut map_b = HashMap::<String, Comparison>::new();

    // fill the maps with the line instances
    for line in buf_a.lines() {
        let line_str: String = line.unwrap();
        map_a
            .entry(line_str) // retrieve the entry for that specific line
            .and_modify(|item: &mut Comparison| item.count = item.count + 1) // increment count if it exists
            .or_insert(Comparison::with_value(1)); // insert with default value 1 if it doesn't exist
    }

    for line in buf_b.lines() {
        let line_str: String = line.unwrap();
        map_b
            .entry(line_str)
            .and_modify(|item: &mut Comparison| item.count = item.count + 1)
            .or_insert(Comparison::with_value(1));
    }

    compare_map_results(&mut map_a, &mut map_b);
    print_diff(&mut map_a, &mut map_b, &mut stream_out);

    // flush before dropping
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
        write!(stream, "{}\t{}\t{}\n", c.kind, c.count, k).unwrap();
    }

    write!(stream, "file b results: \n").unwrap();
    for b in map_b.iter() {
        let (k, c): (&String, &Comparison) = b;
        write!(stream, "{}\t{}\t{}\n", c.kind, c.count, k).unwrap();
    }
}

struct Comparison {
    count: u32,
    kind: String,
}

impl Comparison {
    fn with_value(value: u32) -> Comparison {
        return Comparison {
            count: value,
            kind: "++".to_string(),
        };
    }
}

fn compare_map_results(
    map_a: &mut HashMap<String, Comparison>,
    map_b: &mut HashMap<String, Comparison>,
) {
    // iterate over the entries from file a
    for entries_a in map_a.iter_mut() {
        let (key_a, val_a): (&String, &mut Comparison) = entries_a;

        // the the corresponding entry from file b
        let b_entry = map_b.get_mut(key_a);

        // if file b does not have this entry, we are done
        if b_entry.is_none() {
            continue;
        }

        // if file b has the entry, unwrap it to calculate diffs
        let val_b: &mut Comparison = b_entry.unwrap();

        if val_a.count > val_b.count {
            // if a has a higher value, substract the value from b to get the
            // difference
            val_a.count -= val_b.count;
            // if there are more entries in file a, the kind should be '+' since
            // the entry occured in both files
            val_a.kind = "+".to_string();
            // set the value of this entry in b to zero as this entry is
            // already processed (in file a)
            val_b.count = 0;
        } else if val_b.count > val_a.count {
            // do the same but reversed for a-b
            val_b.count -= val_a.count;
            val_b.kind = "+".to_string();
            val_a.count = 0;
        } else {
            // if they are equal, we can discard these entries in both a and b
            // since they do not matter for the diff anymore
            val_a.count = 0;
            val_b.count = 0;
        }
    }

    // filter out all entries where there is a 0 value since these values have
    // been calculated and are stored in the opposing file map
    map_a.retain(|_, c| c.count != 0);
    map_b.retain(|_, c| c.count != 0);
}
