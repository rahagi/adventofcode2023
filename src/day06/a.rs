use crate::utils;

fn can_beat(velocity: usize, time: usize, distance_to_beat: usize) -> bool {
    velocity * time > distance_to_beat
}

pub fn a(input_file: &str) -> usize {
    let input = utils::file::file_to_str(input_file);
    let mut lines = input.lines();

    let mut time = lines.next().unwrap().split(": ");
    time.next();
    let time = time
        .next()
        .unwrap()
        .split_whitespace()
        .map(|t| t.parse::<usize>().unwrap());

    let mut distance = lines.next().unwrap().split(": ");
    distance.next();
    let distance = distance
        .next()
        .unwrap()
        .split_whitespace()
        .map(|t| t.parse::<usize>().unwrap());

    time.zip(distance)
        .map(|(t, d)| (1..t).filter(move |v| can_beat(*v, t - *v, d)).count())
        .reduce(|a, b| a * b)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::a;
    #[test]
    fn test_06_a() {
        assert_eq!(a("src/day06/input_example.txt"), 288);
        println!("Answer: {}", a("src/day06/input.txt"));
    }
}
