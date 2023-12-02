use crate::utils::{into_lines, max, mul, sum};

pub fn cube_conundrum(input: String) {
    let limit = Drawing {
        red: Some(12),
        green: Some(13),
        blue: Some(14),
    };

    let games = into_lines(input)
        .iter()
        .map(|line| Game::parse(line.to_owned()))
        .collect::<Vec<Game>>();

    let playable_game_ids = games
        .iter()
        .filter(|game| game.is_playable_with(&limit))
        .map(|game| game.id)
        .collect::<Vec<u32>>();

    let lowest_drawing_powers = games
        .iter()
        .map(|game| game.get_lowest_counts().power())
        .collect::<Vec<u32>>();

    println!(
        "playable_games_sum={} lowest_drawing_powers_sum={}",
        sum(playable_game_ids),
        sum(lowest_drawing_powers)
    )
}

#[derive(Debug, Clone)]
struct Game {
    id: u32,
    drawings: Vec<Drawing>,
}

impl Game {
    fn parse(input: String) -> Game {
        let parts = input
            .split(":")
            .map(|item| item.to_owned())
            .collect::<Vec<String>>();

        let id = parts
            .first()
            .unwrap()
            .split(" ")
            .map(|item| item.trim().to_owned())
            .collect::<Vec<String>>()
            .last()
            .unwrap()
            .parse::<u32>()
            .unwrap();

        let drawings = parts
            .last()
            .unwrap()
            .split(";")
            .map(|item| Drawing::parse(item.trim().to_owned()))
            .collect::<Vec<Drawing>>();

        Game { id, drawings }
    }

    fn is_playable_with(&self, limit: &Drawing) -> bool {
        for drawing in &self.drawings {
            if !drawing.is_playable_with(limit) {
                return false;
            }
        }

        true
    }

    fn get_lowest_counts(&self) -> Drawing {
        Drawing {
            red: Some(max(self
                .drawings
                .iter()
                .map(|drawing| drawing.red)
                .filter(|count| count.is_some())
                .map(|count| count.unwrap())
                .collect::<Vec<u32>>())),
            blue: Some(max(self
                .drawings
                .iter()
                .map(|drawing| drawing.blue)
                .filter(|count| count.is_some())
                .map(|count| count.unwrap())
                .collect::<Vec<u32>>())),
            green: Some(max(self
                .drawings
                .iter()
                .map(|drawing| drawing.green)
                .filter(|count| count.is_some())
                .map(|count| count.unwrap().clone())
                .collect::<Vec<u32>>())),
        }
    }
}

#[derive(Debug, Clone)]
struct Drawing {
    red: Option<u32>,
    blue: Option<u32>,
    green: Option<u32>,
}

impl Drawing {
    fn parse(input: String) -> Drawing {
        let items = input
            .split(",")
            .map(|item| item.trim().to_owned())
            .collect::<Vec<String>>();

        let mut red = None;
        let mut blue = None;
        let mut green = None;

        for item in items {
            let sections = item
                .split(" ")
                .map(|inner| inner.trim().to_owned())
                .collect::<Vec<String>>();

            let count = sections.first().unwrap().parse::<u32>().unwrap();
            let color = sections.last().unwrap().as_str();

            match color {
                "red" => red = Some(count),
                "blue" => blue = Some(count),
                "green" => green = Some(count),
                _ => panic!("Invalid color: {color}"),
            }
        }

        Drawing { red, blue, green }
    }

    fn is_playable_with(&self, limit: &Drawing) -> bool {
        self.red <= limit.red && self.green <= limit.green && self.blue <= limit.blue
    }

    fn power(&self) -> u32 {
        let mut items = Vec::new();

        if self.blue.is_some() {
            items.push(self.blue.unwrap())
        }

        if self.green.is_some() {
            items.push(self.green.unwrap())
        }

        if self.red.is_some() {
            items.push(self.red.unwrap())
        }

        mul(items)
    }
}
