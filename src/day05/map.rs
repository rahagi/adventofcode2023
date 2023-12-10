use std::str::FromStr;

#[derive(Debug)]
pub struct Convert {
    pub src_start: usize,
    pub dest_start: usize,
    pub range: usize,
}

#[derive(Debug)]
pub struct WhateverMap {
    pub id: String,
    pub converts: Vec<Convert>,
}

impl WhateverMap {
    pub fn new(id: &str, converts: Vec<Convert>) -> Self {
        Self { id: String::from(id), converts }
    }
}

impl FromStr for WhateverMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        // skip name
        let id = lines.next().unwrap();

        let mut converts: Vec<Convert> = vec![];
        for line in lines {
            let mut map_spec = line.split_whitespace();
            let dest_start = map_spec.next().unwrap().parse::<usize>().unwrap();
            let src_start = map_spec.next().unwrap().parse::<usize>().unwrap();
            let range = map_spec.next().unwrap().parse::<usize>().unwrap();
            converts.push(Convert {
                src_start,
                dest_start,
                range,
            });
        }

        Ok(Self::new(id, converts))
    }
}
