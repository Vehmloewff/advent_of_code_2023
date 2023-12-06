use crate::utils::{collect_numbers, into_lines, mul, safe_sub, split};

pub fn wait_for_it(input: String) {
	let races = parse_out_races(input.clone());
	let winning_counts = races.iter().map(|race| race.get_winning_count()).collect::<Vec<u64>>();

	let one_race = parse_out_races_better(input);

	println!(
		"winning_counts={} single_winning={}",
		mul(winning_counts),
		one_race.get_winning_count()
	)
}

fn parse_out_races(input: String) -> Vec<Race> {
	let lines = into_lines(input);
	let mut times = collect_numbers(split(
		split(lines.get(0).unwrap().to_owned(), &[':']).last().unwrap().to_owned(),
		&[' '],
	));
	let mut distances = collect_numbers(split(
		split(lines.get(1).unwrap().to_owned(), &[':']).last().unwrap().to_owned(),
		&[' '],
	));
	let mut races = Vec::new();

	loop {
		let time = match times.pop() {
			Some(time) => time,
			None => break,
		};
		let distance = match distances.pop() {
			Some(distance) => distance,
			None => break,
		};

		races.push(Race { time, distance })
	}

	races
}

fn parse_out_races_better(input: String) -> Race {
	let lines = into_lines(input);

	let time = split(lines.get(0).unwrap().to_owned(), &[':'])
		.last()
		.unwrap()
		.to_owned()
		.replace(' ', "")
		.parse::<u64>()
		.unwrap();

	let distance = split(lines.get(1).unwrap().to_owned(), &[':'])
		.last()
		.unwrap()
		.to_owned()
		.replace(' ', "")
		.parse::<u64>()
		.unwrap();

	Race { time, distance }
}

#[derive(Debug)]
pub struct Race {
	distance: u64,
	time: u64,
}

impl Race {
	fn get_winning_count(&self) -> u64 {
		let mut holding_time = 0;
		let mut max_distance = 0;
		let mut winning_count = 0;

		loop {
			let time_left = safe_sub(self.time, holding_time);
			let travel_distance = holding_time * time_left;
			let will_win = travel_distance > self.distance;

			if will_win {
				winning_count += 1;
			}

			if max_distance > travel_distance && !will_win {
				break;
			}

			if travel_distance > max_distance {
				max_distance = travel_distance
			}

			holding_time += 1;
		}

		winning_count
	}
}
