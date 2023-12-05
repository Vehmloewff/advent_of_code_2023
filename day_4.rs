use std::collections::HashMap;

use crate::utils::{into_lines, max, min, sum};

pub fn scratchcards(input: String) {
    let scratchcards = into_lines(input)
        .iter()
        .map(|line| Scratchcard::parse(line))
        .collect::<Vec<Scratchcard>>();

    let scores = scratchcards
        .iter()
        .map(|scratchcard| scratchcard.get_score())
        .collect::<Vec<u64>>();

    let mut pad = Scratchpad::new(scratchcards);
    pad.copy_scratchcards();

    println!(
        "scores={} total_cards={}",
        sum(scores),
        pad.get_total_cards()
    )
}

pub struct Scratchpad {
    first_card: u64,
    last_card: u64,
    scratchcards: HashMap<u64, (Scratchcard, u64)>,
}

impl Scratchpad {
    pub fn new(scratchcards: Vec<Scratchcard>) -> Scratchpad {
        let mut map = HashMap::new();
        let mut card_numbers = Vec::new();

        for scratchcard in scratchcards {
            card_numbers.push(scratchcard.card_number);
            map.insert(scratchcard.card_number, (scratchcard, 1));
        }

        Scratchpad {
            first_card: min(card_numbers.clone()),
            last_card: max(card_numbers),
            scratchcards: map,
        }
    }

    pub fn increment_by(&mut self, card_number: u64, amount: u64) {
        self.scratchcards.get_mut(&card_number).unwrap().1 += amount;
    }

    pub fn register_points(&mut self, card_number: u64, points: u64, multiplier: u64) {
        let start_copy = card_number + 1;
        let end_copy = card_number + points + 1;

        for copy_card_num in start_copy..end_copy {
            self.increment_by(copy_card_num, multiplier)
        }
    }

    pub fn copy_scratchcards(&mut self) {
        for card_number in self.first_card..self.last_card + 1 {
            let (card, current_count) = &self.scratchcards.get(&card_number).unwrap();

            self.register_points(card_number, card.get_matches(), current_count.to_owned());
        }
    }

    pub fn get_total_cards(&self) -> u64 {
        let mut amount = 0;

        for (_, count) in self.scratchcards.values() {
            amount += count
        }

        amount
    }
}

pub struct Scratchcard {
    card_number: u64,
    winning_numbers: Vec<u64>,
    real_numbers: Vec<u64>,
}

impl Scratchcard {
    pub fn parse(input: &String) -> Scratchcard {
        let card_basics = input.split(":").collect::<Vec<&str>>();
        let card_number = card_basics
            .first()
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .parse::<u64>()
            .unwrap();

        let card_contents = card_basics.last().unwrap().to_owned();
        let sections = card_contents.trim().split("|").collect::<Vec<&str>>();

        let winning_numbers = sections
            .first()
            .unwrap()
            .to_owned()
            .split(" ")
            .filter(|item| !item.is_empty())
            .map(|item| item.trim().parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        let real_numbers = sections
            .last()
            .unwrap()
            .to_owned()
            .split(" ")
            .filter(|item| !item.is_empty())
            .map(|item| item.trim().parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        Scratchcard {
            card_number,
            winning_numbers,
            real_numbers,
        }
    }

    fn get_score(&self) -> u64 {
        let mut winning_count = 0;

        for real_number in &self.real_numbers {
            if self.winning_numbers.contains(&real_number) {
                if winning_count == 0 {
                    winning_count = 1;
                } else {
                    winning_count = winning_count * 2;
                }
            };
        }

        winning_count
    }

    fn get_matches(&self) -> u64 {
        let mut winning_count = 0;

        for real_number in &self.real_numbers {
            if self.winning_numbers.contains(&real_number) {
                winning_count += 1;
            };
        }

        winning_count
    }
}
