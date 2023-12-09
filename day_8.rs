use crate::utils::{into_lines, least_common_multiple, split};
use std::{collections::HashMap, str::FromStr};

pub fn haunted_wasteland(input: String) {
	let mut lines = into_lines(input);
	let directions = lines.remove(0).parse::<Directions>().unwrap();
	let network = Network::from_lines(lines);
	let starting_codes = network.get_codes_ending_with('A');
	let ending_codes = network.get_codes_ending_with('Z');
	let basic_steps = directions.map("AAA".to_string(), &["ZZZ".to_string()], &network);
	let group_steps = starting_codes
		.iter()
		.map(|code| directions.map(code.clone(), &ending_codes, &network))
		.collect::<Vec<u64>>();
	let complex_steps = least_common_multiple(group_steps);

	println!("basic_steps={basic_steps} complex_steps={complex_steps}");
}

pub struct Network(HashMap<String, (String, String)>);

impl Network {
	fn from_lines(lines: Vec<String>) -> Network {
		let mut map = HashMap::new();

		for line in lines {
			let sections = split(line.to_owned(), &['=']);
			let key = sections.first().unwrap().to_owned();
			let value_inner = sections.last().unwrap()[1..sections.last().unwrap().len() - 1].to_owned();
			let options = split(value_inner, &[',']);

			map.insert(key, (options.first().unwrap().to_owned(), options.last().unwrap().to_owned()));
		}

		Network(map)
	}

	pub fn map(&self, code: &String, direction: Direction) -> &String {
		let (left, right) = self.0.get(code).unwrap();

		match direction {
			Direction::Left => left,
			Direction::Right => right,
		}
	}

	pub fn get_codes_ending_with(&self, ending_char: char) -> Vec<String> {
		self.0
			.keys()
			.filter(|key| key.ends_with(ending_char))
			.map(|key| key.to_owned())
			.collect::<Vec<String>>()
	}
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
	Left,
	Right,
}

pub struct Directions(Vec<Direction>);

impl Directions {
	pub fn map(&self, from: String, to: &[String], network: &Network) -> u64 {
		let mut index = 0;
		let mut last_destination = &from;

		loop {
			let direction_index = index % self.0.len() as u64;
			let direction = self.0.get(direction_index as usize).unwrap().clone();

			if to.contains(last_destination) {
				break;
			}

			last_destination = network.map(last_destination, direction);

			index += 1;
		}

		index
	}
}

impl FromStr for Directions {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(Directions(
			s.trim()
				.chars()
				.map(|character| match character {
					'L' => Direction::Left,
					'R' => Direction::Right,
					_ => panic!("Invalid char: {character}"),
				})
				.collect::<Vec<Direction>>(),
		))
	}
}
