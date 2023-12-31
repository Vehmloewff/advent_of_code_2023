use crate::utils::{into_lines, sum};

pub fn trebuchet(input: String) {
    let lines = into_lines(input);

    let values = lines
        .iter()
        .map(|line| decode_calibration_value(line))
        .collect::<Vec<u64>>();

    let better_values = lines
        .iter()
        .map(|line| better_decode_calibration_value(line))
        .collect::<Vec<u64>>();

    let calibration_sum = sum(values);
    let better_calibration_sum = sum(better_values);

    println!("sum={calibration_sum} better_sum={better_calibration_sum}");
}

fn decode_calibration_value(input: &String) -> u64 {
    let characters = input
        .chars()
        .filter(|character| character.is_numeric())
        .collect::<Vec<char>>();

    let joined = format!(
        "{}{}",
        characters.first().unwrap(),
        characters.last().unwrap()
    );

    joined.parse::<u64>().unwrap()
}

fn better_decode_calibration_value(input: &String) -> u64 {
    // We do these weird double-replacements because we don't want to mess up any existing words
    // For example, removing a nine may mess up an eight: nineight
    let new_input = input
        .replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine");

    decode_calibration_value(&new_input)
}
