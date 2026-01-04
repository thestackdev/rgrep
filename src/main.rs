use clap::Parser;
use colored::Colorize;
use regex::RegexBuilder;
use std::sync::Arc;
use std::thread;
use std::{fs, path::PathBuf};
use walkdir::{self, WalkDir};

#[derive(Parser)]
struct Args {
    pattern: String,
    path: Vec<PathBuf>,

    #[arg(short = 'n', long)]
    line_numbers: bool,

    #[arg(short = 'i', long)]
    case_insensitive: bool,

    #[arg(short = 'r', long)]
    recursive: bool,
}

fn get_contents_from_gitignore() -> Vec<String> {
    let mut files = Vec::new();

    if let Ok(contents) = fs::read_to_string(".gitignore") {
        for line in contents.lines() {
            files.push(
                line.trim()
                    .trim_start_matches("/")
                    .trim_end_matches("/")
                    .to_string(),
            );
        }
    }

    // .git
    files.push(".git".to_string());

    files
}

fn main() {
    let args = Args::parse();
    let pattern = Arc::new(
        RegexBuilder::new(&args.pattern)
            .case_insensitive(args.case_insensitive)
            .build()
            .expect("Invalid serach pattern!"),
    );
    let line_numbers = args.line_numbers;
    let ignore_entries = get_contents_from_gitignore();

    let mut handles = Vec::new();

    let paths = if args.path.is_empty() {
        vec![PathBuf::from(".")]
    } else {
        args.path
    };

    let paths: Vec<PathBuf> = if args.recursive {
        paths
            .into_iter()
            .flat_map(|path| {
                WalkDir::new(path)
                    .into_iter()
                    .filter_entry(|entry| {
                        let name = entry.file_name().to_string_lossy();
                        !ignore_entries.iter().any(|e| name == *e)
                    })
                    .filter_map(|f| f.ok())
                    .filter(|entry| entry.file_type().is_file())
                    .map(|file| file.path().to_path_buf())
            })
            .collect()
    } else {
        paths
    };

    let show_file_names = paths.len() > 1;

    for path in paths {
        let pattern = Arc::clone(&pattern);

        let handle = thread::spawn(move || {
            let file_name = path
                .file_name()
                .and_then(|f| f.to_str())
                .unwrap_or("Unknown");

            if let Ok(file) = fs::read_to_string(&path) {
                for (index, line) in file.lines().enumerate() {
                    let prefix = match (show_file_names, line_numbers) {
                        (true, true) => format!("{}:{}:", file_name, index + 1),
                        (true, false) => format!("{}:", file_name),
                        (false, true) => format!("{}:", index + 1),
                        (false, false) => String::new(),
                    };

                    if pattern.is_match(line) {
                        let colored_line = pattern.replace_all(line, |caps: &regex::Captures| {
                            caps[0].red().bold().to_string()
                        });
                        println!("{}{}", prefix, colored_line);
                    }
                }
            } else {
                let formatted = format!("Failed to read {}", file_name);
                println!("{}", formatted.red());
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
