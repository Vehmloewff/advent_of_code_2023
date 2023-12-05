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

pub fn mul(items: Vec<u64>) -> u64 {
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
    input
        .iter()
        .map(|item| item.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
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
