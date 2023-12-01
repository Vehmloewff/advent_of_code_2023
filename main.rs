mod day_1;

use clap::Parser;
use day_1::trebuchet;
use reqwest::{Client, Method};
use std::env;

#[derive(Parser)]
#[command(bin_name = "cargo run --")]
struct ProgramArgs {
    day: usize,
}

#[tokio::main]
async fn main() {
    let args = ProgramArgs::parse();
    let day = args.day;
    let session = env::var("ADVENT_OF_CODE_SESSION").expect("Expected an ADVENT_OF_CODE");
    let input = Client::new()
        .request(
            Method::GET,
            format!("https://adventofcode.com/2023/day/{day}/input"),
        )
        .header("Cookie", format!("session={session}"))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    match args.day {
        1 => trebuchet(input),
        _ => println!("Unknown day"),
    }
}
