use clap::Parser;

#[derive(Parser)]
#[command(bin_name = "cargo run --")]
struct ProgramArgs {
    day: usize,
}

fn main() {
    let args = ProgramArgs::parse();

    match args.day {
        _ => panic!("Unknown day"),
    }
}
