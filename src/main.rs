use clap::Parser;

#[derive(Parser)]
struct Cli {
    command: String,
    room_number: Option<String>,
}

fn main() {
    let args = Cli::parse();

    println!("command: {:?} | room_number: {:?}", args.command, args.room_number);
}
