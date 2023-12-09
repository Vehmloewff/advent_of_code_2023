mod build_bin;
mod cache;
mod utils;

use advent_of_code_2023::*;
use build_bin::build_bin;
use cache::InputsCache;
use clap::Parser;
use reqwest::{Client, Method};
use std::{env, time::Instant};

#[derive(Parser)]
#[command(bin_name = ".run")]
struct ProgramArgs {
	day: u64,

	#[arg(long)]
	build: bool,
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

	if args.build {
		build_bin(day, input).await
	} else {
		let start = Instant::now();

		match args.day {
			1 => trebuchet(input),
			2 => cube_conundrum(input),
			3 => gear_ratios(input),
			4 => scratchcards(input),
			5 => seeds(input),
			6 => wait_for_it(input),
			7 => camel_cards(input),
			8 => haunted_wasteland(input),
			_ => println!("Unknown day"),
		}

		let ms = start.elapsed().as_millis();
		println!("Executed day {day} in {ms}ms");
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
