use super::game::Game;
use crate::utils;

pub fn a(input_file: &str) -> usize {
    let input = utils::file::file_to_str(input_file);
    input
        .lines()
        .map(|game| game.parse::<Game>().unwrap())
        .filter(|game| game.is_possible())
        .map(|game| game.id)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::a;
    #[test]
    fn test_02_a() {
        assert_eq!(a("src/day02/input_example.txt"), 8);
        println!("Answer: {}", a("src/day02/input.txt"));
    }
}
