use crate::utils;

pub fn b(input_file: &str) {
    let input = utils::file::file_to_str(input_file);
}

#[cfg(test)]
mod tests {
    use super::b;
    #[test]
    fn test_{{DAYNUM}}_b() {
        assert_eq!(b("src/day{{DAYNUM}}/input_example.txt"), 0);
        println!("Answer: {}", b("src/day{{DAYNUM}}/input.txt"));
    }
}
