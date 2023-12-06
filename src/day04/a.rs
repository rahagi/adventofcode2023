use crate::{day04::card::Card, utils};

pub fn a(input_file: &str) -> usize {
    let input = utils::file::file_to_str(input_file);
    input
        .lines()
        .map(|l| l.parse::<Card>().unwrap())
        .map(|c| c.point())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::a;
    #[test]
    fn test_04_a() {
        assert_eq!(a("src/day04/input_example.txt"), 13);
        println!("Answer: {}", a("src/day04/input.txt"));
    }
}
