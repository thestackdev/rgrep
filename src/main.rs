use clap::Parser;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::thread;

#[derive(Parser)]
struct Args {
    pattern: String,
    path: Vec<PathBuf>,

    #[arg(short = 'n', long)]
    line_numbers: bool,
}

fn main() {
    let args = Args::parse();

    let pattern = Arc::new(args.pattern);
    let line_numbers = args.line_numbers;

    let mut handles = vec![];

    let paths = if args.path.is_empty() {
        vec![PathBuf::from(".")]
    } else {
        args.path
    };

    let show_file_names = paths.len() > 1;

    for path in paths {
        let pattern = Arc::clone(&pattern);

        let handle = thread::spawn(move || {
            let file_name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown");

            if let Ok(file) = fs::read_to_string(&path) {
                for (index, line) in file.lines().enumerate() {
                    let prefix = match (show_file_names, line_numbers) {
                        (true, true) => format!("{}:{}:", file_name, index + 1),
                        (true, false) => format!("{}:", file_name),
                        (false, true) => format!("{}:", index + 1),
                        (false, false) => String::new(),
                    };

                    if line.contains(pattern.as_str()) {
                        println!("{}{}", prefix, line);
                    }
                }
            } else {
                println!("{} not found", file_name);
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
