use clap::{arg, Parser};
use std::fs::read_to_string;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    files: Vec<String>,

    #[arg(short('n'), long)]
    number_lines: bool,

    #[arg(short('o'), long)]
    number_nonblank_lines: bool,
}

fn main() {
    let cli = Cli::parse();
    let output = cli
        .files
        .iter()
        .map(|name| read_file(name, cli.number_lines, cli.number_nonblank_lines))
        .collect::<Vec<String>>()
        .join("\n");

    print!("{}", output);
}

fn read_file(name: &str, number_lines: bool, number_nonblank_lines: bool) -> String {
    let mut output = String::new();
    let mut index = 0;
    for line in read_to_string(name).unwrap().lines() {
        if (number_nonblank_lines && !line.is_empty()) || number_lines {
            index += 1;
            output.push_str(format!("{} {}\n", index, line).as_str());
        } else {
            output.push_str(format!("{}\n", line).as_str());
        }
    }
    output
}
