use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::{io, env, process};
use roommate_matcher::match_roommates_from_csv_lines;

fn main() {
    let (_, filepath) = get_cli_args();

    // TODO:2 better handle file error
    let lines = read_lines_from_file(filepath).expect("Failed to read file");

    // TODO:2 handle err
    match_roommates_from_csv_lines(lines).expect("Failed to generate matches");
}

fn get_cli_args() -> (String, String) {
    let mut cli_args = env::args();

    // Arg 0
    let program_name = cli_args.next().unwrap_or_else(|| {
        eprintln!("Program name is somehow missing? You should never see this.");
        process::exit(1);
    });

    // Arg 1
    let filepath = cli_args.next().unwrap_or_else(|| {
        print_usage_exit(&program_name)
    });

    (program_name, filepath)
}

fn print_usage_exit(program_name: &str) -> ! {
    eprintln!();
    eprintln!("Usage:  \t{} <filename>", program_name);
    eprintln!("Example:\t{} ./preferences.csv", program_name);
    eprintln!();
    process::exit(1);
}

fn read_lines_from_file<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line?);
    }

    Ok(lines)
}