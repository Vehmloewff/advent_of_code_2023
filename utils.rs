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
