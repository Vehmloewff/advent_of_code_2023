use crate::utils::sum;

pub fn gear_ratios(input: String) {
    let schematic = Schematic::parse(input);

    let valid_part_numbers = schematic
        .get_parts_with_symbols()
        .iter()
        .map(|part| part.number)
        .collect::<Vec<u32>>();

    let gear_ratios = schematic
        .get_gears_with_parts()
        .iter()
        .map(|gear| gear.get_ratio())
        .collect::<Vec<u32>>();

    println!(
        "valid_part_numbers={} gear_ratios={}",
        sum(valid_part_numbers),
        sum(gear_ratios)
    );
}

#[derive(Debug, Clone)]
struct Schematic {
    part_numbers: Vec<PartNumber>,
    symbols: Vec<Symbol>,
}

impl Schematic {
    pub fn parse(input: String) -> Schematic {
        let mut current_number_chars = Vec::new();
        let mut part_numbers = Vec::new();
        let mut symbols = Vec::new();
        let mut line_number: u32 = 0;
        let mut col_number: u32 = 0;

        for character in input.chars() {
            if character == ' ' || character == '\t' {
                continue;
            }

            if character.is_numeric() {
                current_number_chars.push(character);
                col_number += 1;

                continue;
            }

            // Character is not numeric, so we need to reduce any backflow of numeric characters into a part number
            if !current_number_chars.is_empty() {
                let joined = current_number_chars
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<String>>()
                    .join("");

                let number = joined.parse::<u32>().unwrap();
                let start_col = col_number - joined.len() as u32;

                part_numbers.push(PartNumber {
                    number,
                    line: line_number,
                    start_col,
                    end_col: col_number,
                });

                current_number_chars.clear();
            }

            if character == '\n' {
                col_number = 0;
                line_number += 1;
            } else {
                if character != '.' {
                    symbols.push(Symbol {
                        line: line_number,
                        col: col_number,
                        could_be_gear: character == '*',
                    })
                }

                col_number += 1;
            }
        }

        Schematic {
            part_numbers,
            symbols,
        }
    }

    fn get_parts_with_symbols(&self) -> Vec<&PartNumber> {
        let mut part_numbers_with_symbol = Vec::new();

        for part_number in &self.part_numbers {
            for symbol in &self.symbols {
                if part_number.is_symbol_adjacent(&symbol) {
                    part_numbers_with_symbol.push(part_number);
                    break;
                }
            }
        }

        part_numbers_with_symbol
    }

    fn get_gears_with_parts(&self) -> Vec<Gear<'_>> {
        let mut gears = Vec::new();

        for symbol in &self.symbols {
            if !symbol.could_be_gear {
                continue;
            }

            let mut part_1 = None;
            let mut part_2 = None;

            for part_number in &self.part_numbers {
                if !part_number.is_symbol_adjacent(symbol) {
                    continue;
                }

                if part_1.is_none() {
                    part_1 = Some(part_number)
                } else if part_2.is_none() {
                    part_2 = Some(part_number);

                    gears.push(Gear {
                        symbol,
                        part_1: part_1.unwrap(),
                        part_2: part_2.unwrap(),
                    })
                }
            }
        }

        gears
    }
}

#[derive(Debug, Clone)]
struct PartNumber {
    line: u32,
    start_col: u32,
    end_col: u32,
    number: u32,
}

impl PartNumber {
    pub fn is_symbol_adjacent(&self, symbol: &Symbol) -> bool {
        // Ensure that symbol is on same line, or line before or after
        if symbol.line + 1 < self.line || symbol.line - 1 > self.line {
            return false;
        };

        // ensure that symbol's columns align enough to touch
        // tricky thing to note here: end_col<=>symbol does not have 1-col-freedom because symbol's col is it's start col.
        symbol.col + 1 >= self.start_col && symbol.col <= self.end_col
    }
}

#[derive(Debug, Clone)]
struct Symbol {
    line: u32,
    col: u32,
    could_be_gear: bool,
}

struct Gear<'a> {
    #[allow(unused)]
    symbol: &'a Symbol,
    part_1: &'a PartNumber,
    part_2: &'a PartNumber,
}

impl Gear<'_> {
    fn get_ratio(&self) -> u32 {
        self.part_1.number * self.part_2.number
    }
}
