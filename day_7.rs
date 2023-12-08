use crate::utils::{into_lines, split};
use core::panic;
use std::{cmp::Ordering, collections::HashMap};

pub fn camel_cards(input: String) {
	let mut hands = parse_hands(input);

	// Sort hands in reverse order so that the index will correspond to rank
	hands.sort_by(|a, b| b.cmp(a));

	let mut winnings = 0;

	for index in 0..hands.len() {
		let hand = hands.get(index).unwrap();
		let rank = index + 1;

		winnings += hand.get_winnings(rank as u64);
	}

	println!("total_winnings={winnings}");
}

fn parse_hands(input: String) -> Vec<Hand> {
	into_lines(input).iter().map(|line| Hand::from_string(line.to_string())).collect()
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Clone)]
enum Card {
	Ace,
	King,
	Queen,
	Thegn,
	Nine,
	Eight,
	Seven,
	Six,
	Five,
	Four,
	Three,
	Two,
	Joker,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
	FiveOfAKind,
	FourOfAKind,
	FullHouse,
	ThreeOfAKind,
	TwoPair,
	OnePair,
	HighCard,
}

impl HandType {
	fn get_card_counts(cards: &[Card; 5]) -> HashMap<Card, u64> {
		let mut counts = HashMap::<Card, u64>::new();

		for card in cards {
			if counts.contains_key(card) {
				*counts.get_mut(card).unwrap() += 1;
			} else {
				counts.insert(card.clone(), 1);
			}
		}

		counts
	}

	fn get_card_with_count<'a>(counts: &'a HashMap<Card, u64>, count: u64) -> Option<&'a Card> {
		for (key, value) in counts.iter() {
			if value == &count {
				return Some(&key);
			}
		}

		None
	}

	pub fn get_card_with_frequency(frequency: u64, cards: &[Card; 5]) -> Option<Card> {
		let mut counts = HandType::get_card_counts(cards);
		let jokers_available = counts.get(&Card::Joker).unwrap_or(&0).clone();

		counts.remove(&Card::Joker);

		if jokers_available == frequency {
			return Some(Card::Joker);
		}

		HandType::get_card_with_count(&counts, frequency - jokers_available).cloned()
	}

	pub fn get_cards_with_frequencies(frequency1: u64, frequency2: u64, cards: &[Card; 5]) -> Option<(Card, Card)> {
		let card_1 = match HandType::get_card_with_frequency(frequency1, cards) {
			Some(card) => card,
			None => return None,
		};

		let mut counts = HandType::get_card_counts(cards);
		counts.remove(&Card::Joker);
		counts.remove(&card_1);

		match HandType::get_card_with_count(&counts, frequency2) {
			Some(card_2) => Some((card_1, card_2.clone())),
			None => None,
		}
	}

	pub fn from_cards(cards: &[Card; 5]) -> HandType {
		if Self::get_card_with_frequency(5, cards).is_some() {
			HandType::FiveOfAKind
		} else if HandType::get_card_with_frequency(4, cards).is_some() {
			HandType::FourOfAKind
		} else if HandType::get_cards_with_frequencies(3, 2, cards).is_some() {
			HandType::FullHouse
		} else if HandType::get_card_with_frequency(3, cards).is_some() {
			HandType::ThreeOfAKind
		} else if HandType::get_cards_with_frequencies(2, 2, cards).is_some() {
			HandType::TwoPair
		} else if HandType::get_card_with_frequency(2, cards).is_some() {
			HandType::OnePair
		} else {
			HandType::HighCard
		}
	}
}

impl Card {
	pub fn from_char(character: char) -> Card {
		match character {
			'A' => Card::Ace,
			'K' => Card::King,
			'Q' => Card::Queen,
			'T' => Card::Thegn,
			'9' => Card::Nine,
			'8' => Card::Eight,
			'7' => Card::Seven,
			'6' => Card::Six,
			'5' => Card::Five,
			'4' => Card::Four,
			'3' => Card::Three,
			'2' => Card::Two,
			'J' => Card::Joker,
			_ => panic!("Invalid card character: {character}"),
		}
	}
}

#[derive(Debug, PartialEq, PartialOrd, Eq)]
struct Hand {
	cards: [Card; 5],
	bid: u64,
}

impl Hand {
	pub fn from_string(input: String) -> Hand {
		let segments = split(input, &[' ']);

		let mut cards_vec = segments
			.first()
			.unwrap()
			.chars()
			.map(|character| Card::from_char(character))
			.collect::<Vec<Card>>();

		cards_vec.reverse();

		let cards = [
			cards_vec.pop().unwrap(),
			cards_vec.pop().unwrap(),
			cards_vec.pop().unwrap(),
			cards_vec.pop().unwrap(),
			cards_vec.pop().unwrap(),
		];

		let bid = segments.last().unwrap().parse::<u64>().unwrap();

		Hand { cards, bid }
	}

	pub fn get_winnings(&self, rank: u64) -> u64 {
		self.bid * rank
	}

	pub fn get_type(&self) -> HandType {
		HandType::from_cards(&self.cards)
	}
}

impl Ord for Hand {
	fn cmp(&self, other: &Self) -> Ordering {
		let type_ordering = self.get_type().cmp(&other.get_type());

		if let Ordering::Equal = type_ordering {
			for index in 0..self.cards.len() {
				let self_card = match self.cards.get(index) {
					Some(card) => card,
					None => break,
				};

				let other_card = match other.cards.get(index) {
					Some(card) => card,
					None => break,
				};

				let card_ordering = self_card.cmp(other_card);

				if let Ordering::Greater = card_ordering {
					return Ordering::Greater;
				}

				if let Ordering::Less = card_ordering {
					return Ordering::Less;
				}
			}

			Ordering::Equal
		} else {
			type_ordering
		}
	}
}
