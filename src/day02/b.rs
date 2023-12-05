use super::game::Game;
use crate::utils;

pub fn b(input_file: &str) -> usize {
    let input = utils::file::file_to_str(input_file);
    input
        .lines()
        .map(|game| game.parse::<Game>().unwrap().power())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::b;
    #[test]
    fn test_02_b() {
        assert_eq!(b("src/day02/input_example.txt"), 2286);
        println!("Answer: {}", b("src/day02/input.txt"));
    }
}
