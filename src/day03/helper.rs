fn neighbor_ranges(
    vec: &Vec<&str>,
    i: usize,
    j: usize,
) -> (
    std::ops::RangeInclusive<usize>,
    std::ops::RangeInclusive<usize>,
) {
    let rows = vec.len();
    let cols = vec[0].len();

    let row_range = if i == 0 {
        0..=i + 1
    } else if i == rows - 1 {
        i - 1..=i
    } else {
        i - 1..=i + 1
    };
    let col_range = if j == 0 {
        0..=j + 1
    } else if j == cols - 1 {
        j - 1..=j
    } else {
        j - 1..=j + 1
    };

    (row_range, col_range)
}

pub fn neighbors(vec: &Vec<&str>, i: usize, j: usize) -> Vec<char> {
    let mut neighbors = vec![];

    let (row_range, col_range) = neighbor_ranges(vec, i, j);
    for row in row_range {
        for col in col_range.clone() {
            if row < vec.len() && col < vec[0].len() {
                neighbors.push(char::from(vec[row].as_bytes()[col]));
            }
        }
    }

    neighbors
}

pub fn find_gear(vec: &Vec<&str>, i: usize, j: usize) -> Option<(usize, usize)> {
    let (row_range, col_range) = neighbor_ranges(vec, i, j);
    for row in row_range {
        for col in col_range.clone() {
            if row < vec.len() && col < vec[0].len() && char::from(vec[row].as_bytes()[col]) == '*'
            {
                return Some((row, col));
            }
        }
    }
    None
}
