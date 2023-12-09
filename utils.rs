use std::ops::Mul;

pub fn into_lines(input: String) -> Vec<String> {
	input
		.split("\n")
		.map(|line| line.trim().to_owned())
		.filter(|line| line.len() > 0)
		.collect()
}

pub fn sum(items: Vec<u64>) -> u64 {
	let mut acc = 0;

	for item in items {
		acc += item;
	}

	acc
}

pub fn mul<N: Mul<Output = N>>(items: Vec<N>) -> N {
	let mut acc = None;

	for item in items {
		if acc.is_none() {
			acc = Some(item)
		} else {
			acc = Some(acc.unwrap() * item);
		}
	}

	acc.unwrap()
}

pub fn min(items: Vec<u64>) -> u64 {
	let mut lowest = items.first().unwrap().to_owned();

	for item in items {
		if item < lowest {
			lowest = item
		}
	}

	lowest.to_owned()
}

pub fn max(items: Vec<u64>) -> u64 {
	let mut highest = items.first().unwrap().to_owned();

	for item in items {
		if item > highest {
			highest = item
		}
	}

	highest.to_owned()
}

pub fn split(text: String, delimiters: &[char]) -> Vec<String> {
	text.split(delimiters)
		.map(|item| item.trim().to_owned())
		.filter(|item| !item.is_empty())
		.collect::<Vec<String>>()
}

pub fn collect_numbers(input: Vec<String>) -> Vec<u64> {
	input.iter().map(|item| item.parse::<u64>().unwrap()).collect::<Vec<u64>>()
}

pub fn nest_vector<T>(vec: Vec<T>, every_n: usize) -> Vec<Vec<T>> {
	let mut nested_vec = Vec::new();
	let mut counter = 1;

	for item in vec {
		if nested_vec.is_empty() {
			nested_vec.push(Vec::new());
		}

		if counter > every_n {
			counter = 1;
			nested_vec.push(Vec::new());
		}

		nested_vec.last_mut().unwrap().push(item);

		counter += 1;
	}

	nested_vec
}

pub fn safe_sub(a: u64, b: u64) -> u64 {
	if b > a {
		0
	} else {
		a - b
	}
}

pub fn get_factors(number: u64) -> Vec<u64> {
	(1..number + 1).into_iter().filter(|&x| number % x == 0).collect::<Vec<u64>>()
}

fn get_common_numbers(mut numbers: Vec<Vec<u64>>) -> Vec<u64> {
	let driver = numbers.pop().unwrap();
	let mut common = Vec::new();

	for number in driver {
		let mut is_missing = false;

		for list in &numbers {
			if !list.contains(&number) {
				is_missing = true
			}
		}

		if is_missing {
			common.push(number)
		}
	}

	common
}

pub fn least_common_multiple(numbers: Vec<u64>) -> u64 {
	let mut answer = numbers.get(0).unwrap().clone();

	for index in 1..numbers.len() {
		let number = numbers.get(index).unwrap().clone();
		answer = (number * answer) / (greatest_common_factor(number, answer));
	}

	answer
}

pub fn greatest_common_factor(a: u64, b: u64) -> u64 {
	if b == 0 {
		a
	} else {
		greatest_common_factor(b, a % b)
	}
}
