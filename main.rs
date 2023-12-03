mod cache;
mod utils;

mod day_1;
mod day_2;
mod day_3;

use cache::InputsCache;
use clap::Parser;
use day_1::trebuchet;
use day_2::cube_conundrum;
use day_3::gear_ratios;
use reqwest::{Client, Method};
use std::env;

#[derive(Parser)]
#[command(bin_name = ".run")]
struct ProgramArgs {
    day: u32,
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
        // 426, 985, 841, 463       
        5 => gear_ratios(
            r"
            .........426.............985.........40..........207............................841..463................................633........17.384...
            531&......+..........125....-..312..........#........895......998..945.....@......$.....-...33...................353.....*........*.........
            ........................#......*...........21..727..*..../..-./.............545......80...................602......@..272.......743.........
            ...........558.577..........486...186*925.....*....483.883.1....286...................................625..................#474.....491.....
            ..............*.........243.................287................*............$....245............830.........793......#..........306..*......
            238.685.................*................#.........%........807.........28.947.................*.....705.....*....573...500*781...#..496....
            ..................989..923.......713...539......917.................115..*.....-...........662.........-......413...........................
            ...........=......*..........886.*.........................442......*...........398........*.............%.............636...........%......
            ............976.413...498..../...266........796....................87.....................969.881..&.....815...........*.....279....415....."
                .to_string(),
        ),
        _ => println!("Unknown day"),
    }
}

async fn fetch_input(day: u32, session: String) -> String {
    println!("Fetching inputs for day {day}...");

    Client::new()
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
        .unwrap()
}
