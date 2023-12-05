use crate::utils;

const LETTER_DIGIT_MAP: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn b(input_file: &str) -> usize {
    let input = utils::file::file_to_str(input_file);
    let mut sum = 0;
    for calibration in input.lines() {
        let mut val: [usize; 2] = [0, 10];
        let mut found = 0;
        let mut idx = 0;
        while idx < calibration.len() {
            if let Ok(num) = char::from(calibration.as_bytes()[idx])
                .to_string()
                .parse::<usize>()
            {
                val[found] = num;
                found = 1.min(found + 1);
            } else {
                for (i, letter) in LETTER_DIGIT_MAP.iter().enumerate() {
                    if **letter == calibration[idx..calibration.len().min(idx + letter.len())] {
                        val[found] = i + 1;
                        found = 1.min(found + 1);
                        idx += letter.len() - 2;
                        break;
                    }
                }
            }
            idx += 1;
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
    use super::b;
    #[test]
    fn test_01_b() {
        assert_eq!(b("src/day01/input_example_b.txt"), 281);
        println!("Answer: {}", b("src/day01/input.txt"));
    }
}
