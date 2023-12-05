use core::panic;

use crate::utils::{collect_numbers, min, nest_vector, split};
use regex::Regex;

pub fn seeds(input: String) {
	let seed_list = SeedList::from_str(&input);
	let mappings = MappingBuilder(Mapping::many_from_str(&input));

	let incorrect_seeds = &seed_list.0;
	let incorrect_locations = mappings.map("seed", "location", incorrect_seeds.clone());

	let seed_bottoms = mappings
		.get_seed_bottoms()
		.iter()
		.map(|seed| seed.to_owned())
		.filter(|seed| seed_list.is_within_list(seed.to_owned()))
		.collect::<Vec<u64>>();

	let locations = mappings.map("seed", "location", seed_bottoms);

	println!("closest_location={} real_locations={}", min(incorrect_locations), min(locations));
}

#[derive(Debug, PartialEq)]
pub struct VirtualRange {
	start: u64,
	length: u64,
}

impl VirtualRange {
	pub fn is_within_range(&self, point: u64) -> bool {
		point >= self.start && point < self.start + self.length
	}
}

#[derive(Debug)]
pub struct SeedList(Vec<u64>);

impl SeedList {
	pub fn from_str(input: &str) -> SeedList {
		let captures = Regex::new(r"seeds:((?:\s\d+)+)").unwrap().captures(input).unwrap();

		let codes = captures.get(1).unwrap().as_str().to_owned();
		let list = collect_numbers(split(codes.to_owned(), &[' ']));

		SeedList(list)
	}

	pub fn get_ranges(&self) -> Vec<VirtualRange> {
		let pairs = nest_vector(self.0.clone(), 2);
		let mut ranges = Vec::new();

		for range in pairs {
			let start = range.first().unwrap().to_owned();
			let length = range.last().unwrap().to_owned();

			ranges.push(VirtualRange { start, length })
		}

		ranges
	}

	pub fn is_within_list(&self, seed: u64) -> bool {
		let ranges = self.get_ranges();

		for range in ranges {
			if range.is_within_range(seed) {
				return true;
			};
		}

		false
	}
}

pub struct MappingStrategy {
	do_reverse: bool,
	mappings: Vec<(String, String)>,
}

pub struct MappingBuilder(Vec<Mapping>);

impl MappingBuilder {
	pub fn get_mapping<S: Into<String>>(&self, source: S, destination: S) -> Option<&Mapping> {
		let source_str: String = source.into();
		let destination_str: String = destination.into();

		for mapping in &self.0 {
			if &mapping.source == &source_str && &mapping.destination == &destination_str {
				return Some(mapping);
			}
		}

		None
	}

	pub fn infer_mapping_strategy(&self, from: &str, to: &str) -> MappingStrategy {
		let mut mappings = Vec::<(String, String)>::new();
		let mut reverse_mappings = Vec::<(String, String)>::new();

		let completed = loop {
			let last_thing = mappings.last().map(|m| m.1.to_owned()).unwrap_or(from.into());

			let local_to = self.0.iter().find_map(|mapping| {
				if mapping.source == last_thing {
					Some(&mapping.destination)
				} else {
					None
				}
			});

			if local_to.is_none() {
				break false;
			}

			mappings.push((last_thing, local_to.unwrap().into()));

			if &local_to.unwrap() == &to {
				break true;
			}
		};

		if completed {
			return MappingStrategy {
				mappings,
				do_reverse: false,
			};
		}

		let completed_reverse = loop {
			let last_thing = reverse_mappings.last().map(|m| m.0.to_owned()).unwrap_or(from.into());

			let local_from = self.0.iter().find_map(|mapping| {
				if mapping.destination == last_thing {
					Some(&mapping.source)
				} else {
					None
				}
			});

			if local_from.is_none() {
				break false;
			}

			reverse_mappings.push((local_from.unwrap().into(), last_thing));

			if &local_from.unwrap() == &to {
				break true;
			}
		};

		if completed_reverse {
			return MappingStrategy {
				mappings: reverse_mappings,
				do_reverse: true,
			};
		}

		panic!("Could not find mappings from {} to {}", from, to);
	}

	pub fn map<S: Into<String>>(&self, from: S, to: S, codes: Vec<u64>) -> Vec<u64> {
		let from_str: String = from.into();
		let to_str: String = to.into();

		let MappingStrategy { do_reverse, mappings } = self.infer_mapping_strategy(&from_str, &to_str);
		let mut codes = codes;

		for (from, to) in mappings {
			let mapping = self.get_mapping(from, to).unwrap();

			codes = if do_reverse {
				mapping.map_many_reverse(codes)
			} else {
				mapping.map_many(codes)
			};
		}

		codes
	}

	pub fn get_seed_bottoms(&self) -> Vec<u64> {
		let mut bottoms = Vec::new();

		for mapping in &self.0 {
			let mapping_bottoms = mapping.get_dest_bottoms();
			let mut mappings_seed_bottoms = self.map(mapping.destination.as_str(), "seed", mapping_bottoms);

			bottoms.append(&mut mappings_seed_bottoms);
		}

		bottoms
	}
}

#[derive(Debug)]
pub struct Mapping {
	source: String,
	destination: String,
	entries: Vec<MappingEntry>,
}

impl Mapping {
	pub fn many_from_str(input: &str) -> Vec<Mapping> {
		let regex = Regex::new(r"(\w+)-to-(\w+)\s+map:((?:\s+\d+)+)").unwrap();
		let mut mappings = Vec::new();

		for captures in regex.captures_iter(input) {
			let source = captures.get(1).unwrap().as_str().to_owned();
			let destination = captures.get(2).unwrap().as_str().to_owned();
			let codes = captures.get(3).unwrap().as_str();
			let mut entries = MappingEntry::many_from_str(codes);

			entries.sort_by(|a, b| a.source_start.cmp(&b.source_start));

			mappings.push(Mapping {
				source,
				destination,
				entries,
			})
		}

		mappings
	}

	pub fn map(&self, source_code: u64) -> u64 {
		let mut entry_mapping = None;

		for entry in &self.entries {
			match entry.map(source_code) {
				Some(code) => {
					entry_mapping = Some(code);
					break;
				}
				None => (),
			}
		}

		match entry_mapping {
			Some(code) => code,
			_ => source_code,
		}
	}

	pub fn map_many(&self, codes: Vec<u64>) -> Vec<u64> {
		codes.iter().map(|code| self.map(code.to_owned())).collect::<Vec<u64>>()
	}

	pub fn map_reverse(&self, source_code: u64) -> u64 {
		let mut entry_mapping = None;

		for entry in &self.entries {
			match entry.map_reverse(source_code) {
				Some(code) => {
					entry_mapping = Some(code);
					break;
				}
				None => (),
			}
		}

		match entry_mapping {
			Some(code) => code,
			_ => source_code,
		}
	}

	pub fn map_many_reverse(&self, codes: Vec<u64>) -> Vec<u64> {
		codes.iter().map(|code| self.map_reverse(code.to_owned())).collect::<Vec<u64>>()
	}

	pub fn get_dest_bottoms(&self) -> Vec<u64> {
		self.entries.iter().map(|entry| entry.destination_start).collect::<Vec<u64>>()
	}
}

#[derive(Debug)]
pub struct MappingEntry {
	source_start: u64,
	destination_start: u64,
	length: u64,
}

impl MappingEntry {
	pub fn many_from_str(text: &str) -> Vec<MappingEntry> {
		let number_groups = nest_vector(collect_numbers(split(text.to_owned(), &[' ', '\n'])), 3);
		let mut entries = Vec::new();

		for number_group in number_groups {
			let destination_start = number_group.get(0).unwrap().to_owned();
			let source_start = number_group.get(1).unwrap().to_owned();
			let length = number_group.get(2).unwrap().to_owned();

			entries.push(MappingEntry {
				source_start,
				destination_start,
				length,
			})
		}

		entries
	}

	pub fn map(&self, source_code: u64) -> Option<u64> {
		let self_end = self.source_start + self.length - 1;

		if source_code >= self.source_start && source_code <= self_end {
			let inner_index = source_code - self.source_start;

			Some(self.destination_start + inner_index)
		} else {
			None
		}
	}

	pub fn map_reverse(&self, dest_code: u64) -> Option<u64> {
		let self_end = self.destination_start + self.length - 1;

		if dest_code >= self.destination_start && dest_code <= self_end {
			let inner_index = dest_code - self.destination_start;

			Some(self.source_start + inner_index)
		} else {
			None
		}
	}
}
