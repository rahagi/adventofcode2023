use super::helper::find_gear;
use crate::utils;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;

pub fn b(input_file: &str) -> usize {
    let input = utils::file::file_to_str(input_file);
    let schematic: Vec<&str> = input.lines().collect();
    let mut sum: usize = 0;
    let mut seen_gear: HashMap<(usize, usize), usize> = HashMap::new();

    for (i, line) in schematic.iter().enumerate() {
        let mut buffer = vec![];
        let mut gear_loc: Option<(usize, usize)> = None;
        for (j, c) in line.chars().enumerate() {
            if let Some(digit) = c.to_digit(10) {
                buffer.push(digit as usize);

                if let Some(loc) = find_gear(&schematic, i, j) {
                    gear_loc = Some(loc);
                }
            }
            if !buffer.is_empty() && (!c.is_ascii_digit() || j + 1 >= line.len()) {
                if let Some(loc) = gear_loc {
                    // turn [1, 2, 3] into 123_usize
                    let part = (buffer[0] * 10_usize.pow((buffer.len() - 1) as u32))
                        + buffer[1..]
                            .iter()
                            .enumerate()
                            .map(|(i, b)| b * 10_usize.pow((buffer.len() - 2 - i) as u32))
                            .sum::<usize>();
                    match seen_gear.entry(loc) {
                        Occupied(entry) => {
                            sum += entry.get() * part;
                        }
                        Vacant(entry) => {
                            entry.insert(part);
                        }
                    }
                }
                buffer = vec![];
                gear_loc = None;
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::b;
    #[test]
    fn test_03_b() {
        assert_eq!(b("src/day03/input_example.txt"), 467835);
        println!("Answer: {}", b("src/day03/input.txt"));
    }
}
