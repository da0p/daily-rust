use clap::{arg, Command, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    text: String,
}

fn main() {
    let matches = Command::new("echor")
        .version("0.1.0")
        .author("Phuong Dao")
        .about("echor")
        .args([
            arg!(-t --text <TEXT> "Input text").required(false),
            arg!(-n --omit_newline "Omit new line").required(false),
        ])
        .get_matches();

    let text = matches.get_one::<String>("text");
    println!("{}", text.unwrap());
}
