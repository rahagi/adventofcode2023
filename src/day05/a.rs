use super::map::WhateverMap;
use crate::utils;

pub fn a(input_file: &str) -> usize {
    let input = utils::file::file_to_str(input_file);
    let mut input_split = input.split("\n\n");

    let seeds_raw = input_split.next().unwrap();
    let mut seeds_split = seeds_raw.split(": ");
    seeds_split.next();
    let mut root = seeds_split
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut current_min: usize = 0;
    for input_raw in input_split {
        let next_map: WhateverMap = input_raw.parse().unwrap();
        let child: Vec<usize> =
            root.iter()
                .map(|target| {
                    let valid_range = next_map.converts.iter().find(|conv| {
                        conv.src_start < *target && *target < conv.src_start + conv.range
                    });
                    if let Some(range) = valid_range {
                        target - range.src_start + range.dest_start
                    } else {
                        *target
                    }
                })
                .collect();
        current_min = *child.iter().min().unwrap();
        root = child;
    }

    current_min
}

#[cfg(test)]
mod tests {
    use super::a;
    #[test]
    fn test_05_a() {
        assert_eq!(a("src/day05/input_example.txt"), 35);
        println!("Answer: {}", a("src/day05/input.txt"));
    }
}
