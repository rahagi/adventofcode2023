use super::card::Card;
use crate::utils;
use std::collections::VecDeque;

pub fn b(input_file: &str) -> usize {
    let input = utils::file::file_to_str(input_file);
    let original_cards = input
        .lines()
        .map(|c| c.parse::<Card>().unwrap())
        .collect::<Vec<Card>>();
    let mut stack_cards = original_cards
        .iter()
        .map(|c| c.id)
        .collect::<VecDeque<usize>>();
    let mut card_count: Vec<usize> = vec![1; original_cards.len()];

    while let Some(id) = stack_cards.pop_front() {
        let card = &original_cards[id - 1];
        let n_copy = card_count[id - 1];

        for id in card.copies() {
            card_count[id - 1] += n_copy;
        }
    }

    card_count.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::b;
    #[test]
    fn test_04_b() {
        assert_eq!(b("src/day04/input_example.txt"), 30);
        println!("Answer: {}", b("src/day04/input.txt"));
    }
}
