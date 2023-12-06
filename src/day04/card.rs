use std::{collections::HashSet, str::FromStr};

#[derive(Debug)]
pub struct Card {
    pub id: usize,
    numbers: Vec<usize>,
    winning_numbers: HashSet<usize>,
}

impl Card {
    pub fn new(id: usize, numbers: Vec<usize>, winning_numbers: HashSet<usize>) -> Self {
        Self {
            id,
            numbers,
            winning_numbers,
        }
    }

    pub fn copies(&self) -> std::ops::RangeInclusive<usize> {
        let num_of_win = self
            .numbers
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .count();

        self.id + 1..=self.id + num_of_win
    }

    pub fn point(&self) -> usize {
        let mut total = 0;
        let mut found = 0;
        for number in self.numbers.iter() {
            if self.winning_numbers.contains(number) {
                found += 1;
                total = 2_usize.pow(found - 1);
            }
        }
        total
    }
}

impl FromStr for Card {
    type Err = &'static str;

    // xd
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(": ");

        let card_section = split.next().unwrap();
        let mut id_iter = card_section.split_whitespace();
        id_iter.next();
        let id = id_iter.next().unwrap().parse::<usize>().unwrap();

        let cards = split.next().unwrap();
        let mut cards_split = cards.split(" | ");
        let winning_numbers = cards_split
            .next()
            .unwrap()
            .split_whitespace()
            .map(|card| card.parse::<usize>().unwrap())
            .collect::<HashSet<usize>>();
        let numbers = cards_split
            .next()
            .unwrap()
            .split_whitespace()
            .map(|card| card.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        Ok(Self::new(id, numbers, winning_numbers))
    }
}
