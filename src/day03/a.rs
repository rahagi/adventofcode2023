use super::helper::neighbors;
use crate::utils;

pub fn a(input_file: &str) -> usize {
    let input = utils::file::file_to_str(input_file);
    let schematic: Vec<&str> = input.lines().collect();
    let mut sum: usize = 0;

    for (i, line) in schematic.iter().enumerate() {
        let mut buffer: Vec<usize> = vec![];
        let mut is_adjacent_to_symbol = false;
        for (j, c) in line.chars().enumerate() {
            if let Some(digit) = c.to_digit(10) {
                buffer.push(digit as usize);

                let neighbors = neighbors(&schematic, i, j);
                if !is_adjacent_to_symbol
                    && neighbors.iter().any(|c| *c != '.' && !c.is_ascii_digit())
                {
                    is_adjacent_to_symbol = true;
                }
            }
            if !c.is_ascii_digit() || j + 1 >= line.len() {
                if is_adjacent_to_symbol {
                    // turn [1, 2, 3] into 123_usize
                    sum += (buffer[0] * 10_usize.pow((buffer.len() - 1) as u32))
                        + buffer[1..]
                            .iter()
                            .enumerate()
                            .map(|(i, b)| b * 10_usize.pow((buffer.len() - 2 - i) as u32))
                            .sum::<usize>();
                }
                is_adjacent_to_symbol = false;
                buffer = vec![];
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::a;
    #[test]
    fn test_03_a() {
        assert_eq!(a("src/day03/input_example.txt"), 4361);
        println!("Answer: {}", a("src/day03/input.txt"));
    }
}
