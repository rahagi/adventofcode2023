use crate::utils;

pub fn a(input_file: &str) -> usize {
    let input = utils::file::file_to_str(input_file);
    let mut sum = 0;
    for calibration in input.lines() {
        let mut val: [usize; 2] = [0, 10];
        let mut found = 0;
        for c in calibration.split("") {
            if let Ok(num) = c.parse::<usize>() {
                val[found] = num;
                found = 1.min(found + 1);
            }
        }
        if val[1] == 10 {
            val[1] = val[0]
        }
        sum += val[0] * 10 + val[1];
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::a;
    #[test]
    fn test_01_a() {
        assert_eq!(a("src/day01/input_example.txt"), 142);
        println!("Answer: {}", a("src/day01/input.txt"));
    }
}
