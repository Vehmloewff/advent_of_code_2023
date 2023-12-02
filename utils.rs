pub fn into_lines(input: String) -> Vec<String> {
    input
        .split("\n")
        .map(|line| line.trim().to_owned())
        .filter(|line| line.len() > 0)
        .collect()
}

pub fn sum(items: Vec<u32>) -> u32 {
    let mut acc = 0;

    for item in items {
        acc += item;
    }

    acc
}

pub fn mul(items: Vec<u32>) -> u32 {
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

pub fn min(items: Vec<u32>) -> u32 {
    let mut lowest = items.first().unwrap().to_owned();

    for item in items {
        if item < lowest {
            lowest = item
        }
    }

    lowest.to_owned()
}

pub fn max(items: Vec<u32>) -> u32 {
    let mut highest = items.first().unwrap().to_owned();

    for item in items {
        if item > highest {
            highest = item
        }
    }

    highest.to_owned()
}
