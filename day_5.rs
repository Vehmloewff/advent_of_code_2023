use std::ops::Index;

use crate::utils::{collect_numbers, min, nest_vector, safe_sub, split};
use regex::Regex;

pub fn day_5(input: String) {
	let seed_list = SeedList::from_str(&input);
	let mappings = MappingBuilder(Mapping::many_from_str(&input));

	let locations = seed_list
		.0
		.iter()
		.map(|seed| mappings.map_seed_to_location(seed.to_owned()))
		.collect::<Vec<u64>>();

	let seed_bottoms = mappings
		.get_seed_bottoms()
		.iter()
		.filter(|seed| seed_list.is_within_list(seed.to_owned().to_owned()))
		.map(|seed| seed.to_owned())
		.collect::<Vec<u64>>();

	// dbg!(mappings.get_seed_bottoms());
	// dbg!(&seed_bottoms);

	// let seed_ranges = seed_list.get_ranges();
	// let mut location_ranges = mappings.map_seed_to_location_ranges(seed_ranges);

	// location_ranges.sort_by(|a, b| a.start.cmp(&b.start));
	// dbg!(&location_ranges);

	let ranged_locations = seed_bottoms
		.iter()
		.map(|seed| mappings.map_seed_to_location(seed.to_owned()))
		.collect::<Vec<u64>>();

	// mappings.0.last().unwrap().entries.

	// for range in location_ranges {
	// 	if range.start == 0 {
	// 		continue;
	// 	}

	// 	dbg!(range.start + 1);
	// 	break;
	// }
	// let lowest_location = location_ranges.first().unwrap().start;
	// 	.map(|seed| {
	// 		print!("\r{}%", (index as f64 / real_seed_list.len() as f64).round());
	// 		index += 1;

	// 		mappings.map_seed_to_location(seed.to_owned())
	// 	})
	// 	.collect::<Vec<u64>>();

	println!("closest_location={} real_locations={}", min(locations), min(ranged_locations));
	//min(real_locations))
}

#[derive(Debug, PartialEq)]
pub struct VirtualRange {
	start: u64,
	length: u64,
}

impl VirtualRange {
	pub fn has_content(&self) -> bool {
		self.length > 0
	}

	pub fn get_distance_to(&self, range: &VirtualRange) -> VirtualRange {
		let next_start = range.start;
		let this_end = self.start + self.length;
		let middle_length = next_start - this_end;

		VirtualRange {
			start: this_end,
			length: middle_length,
		}
	}

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

	pub fn map_seed_to_location_ranges(&self, seed_ranges: Vec<VirtualRange>) -> Vec<VirtualRange> {
		let soil_ranges = self.get_mapping("seed", "soil").unwrap().map_ranges(seed_ranges);
		let fertilizer_ranges = self.get_mapping("soil", "fertilizer").unwrap().map_ranges(soil_ranges);
		let water_ranges = self.get_mapping("fertilizer", "water").unwrap().map_ranges(fertilizer_ranges);
		let light_ranges = self.get_mapping("water", "light").unwrap().map_ranges(water_ranges);
		let temperature_ranges = self.get_mapping("light", "temperature").unwrap().map_ranges(light_ranges);
		let humidity_ranges = self.get_mapping("temperature", "humidity").unwrap().map_ranges(temperature_ranges);
		let location_ranges = self.get_mapping("humidity", "location").unwrap().map_ranges(humidity_ranges);

		location_ranges

		// fertilizer_ranges
	}

	pub fn map_seed_to_location(&self, seed: u64) -> u64 {
		let soil = self.get_mapping("seed", "soil").unwrap().map(seed.to_owned());
		let fertilizer = self.get_mapping("soil", "fertilizer").unwrap().map(soil);
		let water = self.get_mapping("fertilizer", "water").unwrap().map(fertilizer);
		let light = self.get_mapping("water", "light").unwrap().map(water);
		let temperature = self.get_mapping("light", "temperature").unwrap().map(light);
		let humidity = self.get_mapping("temperature", "humidity").unwrap().map(temperature);
		let location = self.get_mapping("humidity", "location").unwrap().map(humidity);

		location
	}

	pub fn map<S: Into<String>>(&self, from: S, to: S, codes: Vec<u64>) -> Vec<u64> {
		let from_str: String = from.into();
		let to_str: String = to.into();

		let direction = [
			"seed",
			"soil",
			"fertilizer",
			"water",
			"light",
			"temperature",
			"humidity",
			"location",
		]
		.to_vec();

		let from_index = direction.iter().position(|item| &item.to_owned() == &from_str).unwrap();
		let to_index = direction.iter().position(|item| &item.to_owned() == &to_str).unwrap();
		let do_reverse = from_index > to_index;
		let slice = if do_reverse {
			direction[to_index..from_index + 1].to_vec()
		} else {
			direction[from_index..to_index].to_vec()
		};

		let mut pairs: Vec<(&str, &str)> = Vec::new();
		let mut last_item = None;

		for item in slice {
			if last_item.is_some() {
				pairs.push((last_item.unwrap(), item));
			};

			last_item = Some(item);
		}

		if do_reverse {
			pairs.reverse()
		}

		let mut codes = codes;

		for (from, to) in pairs {
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

	pub fn map_ranges(&self, source_ranges: Vec<VirtualRange>) -> Vec<VirtualRange> {
		let mut dest_ranges = Vec::new();

		for source_range in source_ranges {
			// println!("Source range {source_range:#?}");
			let ranges = self.map_range(source_range);
			// println!("Corresponds to {ranges:#?}");

			for range in ranges {
				dest_ranges.push(range)
			}
		}

		dest_ranges
	}

	/// Map a source range into a vector of destination ranges. A single range can become multiple ranges if it can only be mapped by
	/// splitting it across multiple mapping entries
	pub fn map_range(&self, source_range: VirtualRange) -> Vec<VirtualRange> {
		let mut dest_ranges = Vec::new();
		let mut last_source_number = source_range.start;
		let source_end = source_range.start + source_range.length;

		for entry in &self.entries {
			// The entry could start before the range
			// The entry could finish before the range
			// One, both, or neither could be true
			let entry_start = entry.source_start;
			let entry_end = entry.source_start + entry.length;
			let range_start = source_range.start;
			let range_end = source_range.start + source_range.length;

			// dbg!(entry_start, range_start, range_end);
			let entry_start_is_within_bounds = entry_start >= range_start && entry_start <= range_end;
			let entry_end_is_within_bounds = entry_end >= range_start && entry_end <= range_end;
			let entry_covers_range =
				!entry_start_is_within_bounds && !entry_end_is_within_bounds && entry_start < range_start && entry_end > range_end;

			// dbg!(entry_start_is_within_bounds, entry_start_is_within_bounds);

			if entry_start_is_within_bounds {
				// the amount of distance that entry is into range
				let inset = entry_start - range_start;

				// maybe there was no mapping found for the first part of the range. If so, add it in.
				let missing_length = safe_sub(entry_start, last_source_number);
				if missing_length > 0 {
					dest_ranges.push(VirtualRange {
						start: last_source_number,
						length: min(vec![missing_length, source_range.length]),
					});

					last_source_number += missing_length;
				}

				let remaining_range_distance = source_range.length - inset;
				let entry_distance = min(vec![remaining_range_distance, entry.length]);

				if entry_distance > 0 {
					dest_ranges.push(VirtualRange {
						start: last_source_number,
						length: entry_distance,
					});

					last_source_number += entry_distance;
				}
			}

			if entry_end_is_within_bounds {
				let reverse_inset = range_end - entry_end;
				let mapping_length = source_range.length - reverse_inset;

				dest_ranges.push(VirtualRange {
					start: range_start,
					length: mapping_length,
				});

				last_source_number += mapping_length;
			}

			if entry_covers_range {
				let new_range_start = if entry.source_start > entry.destination_start {
					let difference = entry.source_start - entry.destination_start;

					range_start - difference
				} else {
					let difference = entry.destination_start - entry.source_start;

					range_start + difference
				};
				// dbg!(new_range_start);

				let length = source_range.length;

				if length > 0 {
					dest_ranges.push(VirtualRange {
						start: new_range_start,
						length,
					});

					last_source_number += length;
				}
			}

			// The entry could be too large
			// The entry could be
			// let missing_length = safe_sub(entry.source_start, last_source_number);
			// if missing_length > 0 {
			// 	dest_ranges.push(VirtualRange {
			// 		start: last_source_number,
			// 		length: min(vec![missing_length, source_range.length]),
			// 	});

			// 	last_source_number += missing_length;
			// }

			// dbg!(entry.source_start, last_source_number);
			// if entry.source_start > last_source_number {
			// 	let difference = entry.source_start - last_source_number;
			// 	let source_range_remaining = safe_sub(source_end, entry.source_start + difference);
			// 	let dest_start = entry.destination_start + difference;
			// 	let length = min(vec![entry.length - difference, source_range_remaining]);

			// 	dest_ranges.push(VirtualRange {
			// 		start: dest_start,
			// 		length: length,
			// 	});
			// } else {
			// 	dbg!(source_end);
			// 	dbg!(entry.source_start);
			// 	let entry_offset = last_source_number - entry.source_start;
			// 	let destination_start = entry.destination_start + entry_offset;
			// 	let source_range_remaining = safe_sub(source_end, entry.source_start + entry_offset);
			// 	let mapped_length = min(vec![entry.length, source_range_remaining]);
			// 	dbg!(entry.length, source_range_remaining);

			// 	if mapped_length > 0 {
			// 		dbg!(last_source_number, source_end);
			// 		dest_ranges.push(VirtualRange {
			// 			start: destination_start,
			// 			length: mapped_length,
			// 		});

			// 		last_source_number += mapped_length;
			// 	}
			// }

			if last_source_number >= source_end {
				break;
			}
		}

		if last_source_number < source_end {
			dest_ranges.push(VirtualRange {
				start: last_source_number,
				length: source_end - last_source_number,
			})
		}

		// println!("{}", dest_ranges.len());

		dest_ranges
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

	pub fn get_source_bottoms(&self) -> Vec<u64> {
		self.entries.iter().map(|entry| entry.source_start).collect::<Vec<u64>>()
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

	pub fn map_range(&self, source_range: VirtualRange) -> Option<MappingAttempt> {
		let self_end = self.source_start + self.length - 1;

		if source_range.start >= self.source_start && source_range.start < self_end {
			let header_unmet_length = source_range.start - self.source_start;
			let destination_start = source_range.start + header_unmet_length;
			// the amount of codes we could map, if we wanted to map them all
			let mapping_space_left = self.length - header_unmet_length;
			// we don't want the dest range to extend past the mapping space we have left
			let destination_length = min(vec![source_range.length, mapping_space_left]);
			let footer_unmet_length = mapping_space_left - destination_length;

			Some(MappingAttempt {
				header_unmet_range: VirtualRange {
					start: self.source_start,
					length: header_unmet_length,
				},
				destination_range: VirtualRange {
					start: destination_start,
					length: destination_length,
				},
				footer_unmet_range: VirtualRange {
					start: source_range.start + destination_length,
					length: footer_unmet_length,
				},
			})
		} else {
			None
		}
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

pub struct MappingAttempt {
	header_unmet_range: VirtualRange,
	destination_range: VirtualRange,
	footer_unmet_range: VirtualRange,
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn seed_to_soil() {
		let mapping = Mapping {
			source: "test".into(),
			destination: "test".into(),
			entries: vec![
				MappingEntry {
					source_start: 98,
					destination_start: 50,
					length: 2,
				},
				MappingEntry {
					source_start: 50,
					destination_start: 52,
					length: 48,
				},
			],
		};

		let mapped = mapping.map_ranges(vec![VirtualRange { start: 79, length: 14 }, VirtualRange { start: 55, length: 13 }]);

		assert_eq!(
			mapped,
			vec![VirtualRange { start: 81, length: 14 }, VirtualRange { start: 57, length: 13 }]
		);
	}

	#[test]
	fn soil_to_fertilizer() {
		let mapping = Mapping {
			source: "test".into(),
			destination: "test".into(),
			entries: vec![
				MappingEntry {
					source_start: 15,
					destination_start: 0,
					length: 37,
				},
				MappingEntry {
					source_start: 52,
					destination_start: 37,
					length: 2,
				},
				MappingEntry {
					source_start: 0,
					destination_start: 39,
					length: 15,
				},
			],
		};

		assert_eq!(
			mapping.map_ranges(vec![VirtualRange { start: 81, length: 14 }, VirtualRange { start: 57, length: 13 }]),
			vec![VirtualRange { start: 81, length: 14 }, VirtualRange { start: 57, length: 13 }]
		);
	}

	fn fertilizer_to_water() {
		let mapping = Mapping {
			source: "test".into(),
			destination: "test".into(),
			entries: vec![
				MappingEntry {
					source_start: 53,
					destination_start: 49,
					length: 8,
				},
				MappingEntry {
					source_start: 11,
					destination_start: 0,
					length: 42,
				},
				MappingEntry {
					source_start: 0,
					destination_start: 42,
					length: 7,
				},
				MappingEntry {
					source_start: 7,
					destination_start: 57,
					length: 4,
				},
			],
		};

		assert_eq!(
			mapping.map_ranges(vec![VirtualRange { start: 81, length: 14 }, VirtualRange { start: 57, length: 13 }]),
			vec![VirtualRange { start: 81, length: 14 }, VirtualRange { start: 57, length: 13 }]
		);
	}

	#[test]
	fn water_to_light() {
		let mapping = Mapping {
			source: "test".into(),
			destination: "test".into(),
			entries: vec![
				MappingEntry {
					source_start: 18,
					destination_start: 88,
					length: 7,
				},
				MappingEntry {
					source_start: 25,
					destination_start: 18,
					length: 70,
				},
			],
		};

		assert_eq!(
			mapping.map_ranges(vec![VirtualRange { start: 81, length: 14 }, VirtualRange { start: 57, length: 13 }]),
			vec![VirtualRange { start: 74, length: 14 }, VirtualRange { start: 50, length: 13 }]
		);
	}

	#[test]
	fn light_to_temperature() {
		let mapping = Mapping {
			source: "test".into(),
			destination: "test".into(),
			entries: vec![
				MappingEntry {
					source_start: 77,
					destination_start: 45,
					length: 23,
				},
				MappingEntry {
					source_start: 45,
					destination_start: 81,
					length: 19,
				},
				MappingEntry {
					source_start: 64,
					destination_start: 68,
					length: 13,
				},
			],
		};

		assert_eq!(
			mapping.map_ranges(vec![VirtualRange { start: 74, length: 14 }, VirtualRange { start: 50, length: 13 }]),
			vec![
				VirtualRange { start: 74, length: 3 },
				VirtualRange { start: 77, length: 11 },
				VirtualRange { start: 86, length: 13 }
			]
		);
	}

	#[test]
	fn temperature_to_humidity() {
		let mapping = Mapping {
			source: "test".into(),
			destination: "test".into(),
			entries: vec![
				MappingEntry {
					source_start: 69,
					destination_start: 0,
					length: 1,
				},
				MappingEntry {
					source_start: 0,
					destination_start: 1,
					length: 69,
				},
			],
		};

		assert_eq!(
			mapping.map_ranges(vec![
				VirtualRange { start: 74, length: 3 },
				VirtualRange { start: 77, length: 11 },
				VirtualRange { start: 86, length: 13 }
			]),
			vec![VirtualRange { start: 50, length: 14 }, VirtualRange { start: 86, length: 13 }]
		);
	}

	#[test]
	fn humidity_to_location() {
		let mapping = Mapping {
			source: "test".into(),
			destination: "test".into(),
			entries: vec![
				MappingEntry {
					source_start: 56,
					destination_start: 60,
					length: 37,
				},
				MappingEntry {
					source_start: 93,
					destination_start: 56,
					length: 4,
				},
			],
		};

		assert_eq!(
			mapping.map_ranges(vec![VirtualRange { start: 50, length: 14 }, VirtualRange { start: 86, length: 13 }]),
			vec![
				VirtualRange { start: 50, length: 6 },
				VirtualRange { start: 56, length: 8 },
				VirtualRange { start: 86, length: 7 },
				VirtualRange { start: 93, length: 4 },
				VirtualRange { start: 86, length: 11 }
			]
		);
	}
}
