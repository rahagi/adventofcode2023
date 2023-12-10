use std::ops::Range;

use super::map::WhateverMap;
use crate::utils;

pub fn b(input_file: &str) -> usize {
    let input = utils::file::file_to_str(input_file);
    let mut input_split = input.split("\n\n");

    let seeds_raw = input_split.next().unwrap();
    let mut seeds_split = seeds_raw.split(": ");
    seeds_split.next();
    let mut seeds_ranges = seeds_split
        .clone()
        .next()
        .unwrap()
        .split_whitespace()
        .enumerate()
        .filter(|(i, _)| i % 2 != 0)
        .map(|(_, s)| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let seeds_init = seeds_split
        .next()
        .unwrap()
        .split_whitespace()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .map(|(_, s)| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let mut root: Vec<Range<usize>> = seeds_init
        .iter()
        .enumerate()
        .map(|(i, si)| *si..*si + seeds_ranges[i] + 1)
        .collect();

    let mut current_min: usize = usize::MAX;
    for input_raw in input_split {
        let next_map: WhateverMap = input_raw.parse().unwrap();
        let mut child = vec![];
        for seq in root.clone().iter() {
            println!("{:?}", seq);
            for r in seq.clone() {
                current_min = current_min.min(r)
            }
        }
        for (i, ranges) in root.iter_mut().enumerate() {
            let mut next_target = 0;
            let target = ranges.next().unwrap();
            let valid_range = next_map
                .converts
                .iter()
                .find(|conv| conv.src_start < target && target < conv.src_start + conv.range);
            if let Some(range) = valid_range {
                next_target = target - range.src_start + range.dest_start;
                println!();
                println!("for {:?}", ranges);
                println!("map {:?}", range);
                println!("original_range: {:?}", seeds_ranges[i]);
                println!("range: {:?}", range.range);
                println!("target: {:?}", target);
                println!("next_target: {:?}", next_target);
                let remainder_range = range.range - (target - range.src_start);
                println!(
                    "remainder_range: {}",
                    range.range - (target - range.src_start)
                );
                println!("seeds_ranges: {:?}", seeds_ranges[i]);
                if remainder_range < seeds_ranges[i] {
                    let new_interval_range = remainder_range + 1;
                    let new_interval_cut = next_target + new_interval_range;
                    println!(
                        "new_interval_start: {:?}",
                        range.range - (target - next_target)
                    );
                    child.push(next_target..new_interval_cut);
                    child.push(
                        target + remainder_range + 1
                            ..target + remainder_range + 1 + seeds_ranges[i] - remainder_range,
                    );
                    seeds_ranges.push(new_interval_range);
                } else {
                    let next_upper_bound = next_target + seeds_ranges[i] + 1;
                    child.push(next_target..next_upper_bound);
                }
            } else {
                next_target = target;
                let next_upper_bound = next_target + seeds_ranges[i] + 1;
                child.push(next_target..next_upper_bound);
            };
        }

        println!();
        println!("after {}", next_map.id);
        root = child;
    }

    current_min
}

#[cfg(test)]
mod tests {
    use super::b;
    #[test]
    fn test_05_b() {
        assert_eq!(b("src/day05/input_example.txt"), 46);
        println!("Answer: {}", b("src/day05/input.txt"));
    }
}
