use crate::utils;

pub fn a(input_file: &str) -> usize {
    let input = utils::file::file_to_str(input_file);
    0
}

#[cfg(test)]
mod tests {
    use super::a;
    #[test]
    fn test_{{DAYNUM}}_a() {
        assert_eq!(a("src/day{{DAYNUM}}/input_example.txt"), 0);
        println!("Answer: {}", a("src/day{{DAYNUM}}/input.txt"));
    }
}
