mod cache;
mod utils;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;

use cache::InputsCache;
use clap::Parser;
use day_1::trebuchet;
use day_2::cube_conundrum;
use day_3::gear_ratios;
use day_4::scratchcards;
use day_5::seeds;
use reqwest::{Client, Method};
use std::env;

#[derive(Parser)]
#[command(bin_name = ".run")]
struct ProgramArgs {
	day: u64,
}

#[tokio::main]
async fn main() {
	let args = ProgramArgs::parse();
	let day = args.day;
	let session = env::var("ADVENT_OF_CODE_SESSION").expect("Expected an ADVENT_OF_CODE_SESSION");
	let cache = InputsCache::new().await;

	let input = match cache.get(day).await {
		Some(input) => input,
		None => {
			let input = fetch_input(day, session).await;
			cache.set(day, input.clone()).await;

			input
		}
	};

	match args.day {
		1 => trebuchet(input),
		2 => cube_conundrum(input),
		3 => gear_ratios(input),
		4 => scratchcards(input),
		5 => seeds(input),
		_ => println!("Unknown day"),
	}
}

async fn fetch_input(day: u64, session: String) -> String {
	println!("Fetching inputs for day {day}...");

	Client::new()
		.request(Method::GET, format!("https://adventofcode.com/2023/day/{day}/input"))
		.header("Cookie", format!("session={session}"))
		.send()
		.await
		.unwrap()
		.text()
		.await
		.unwrap()
}
