use clap::{arg, Parser};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    text: Vec<String>,

    #[arg(short, long)]
    omit_newline: bool
}

fn main() {
    let cli =  Cli::parse();
    let text = cli.text.join(" ");
    let omit_newline = cli.omit_newline;
    print!("{}{}", text, if omit_newline {""} else {"\n"}); 
}
