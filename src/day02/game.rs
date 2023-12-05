use std::str::FromStr;

#[derive(Debug)]
pub struct Game {
    pub id: usize,
    blue: usize,
    green: usize,
    red: usize,
}

impl Game {
    pub fn new(id: usize, blue: usize, green: usize, red: usize) -> Self {
        Self {
            id,
            blue,
            green,
            red,
        }
    }

    pub fn power(&self) -> usize {
        self.blue * self.green * self.red
    }

    pub fn is_possible(&self) -> bool {
        self.blue <= 14 && self.green <= 13 && self.red <= 12
    }
}

impl FromStr for Game {
    type Err = &'static str;

    // xd
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(": ");

        let game_section = split.next().unwrap();
        let mut id_iter = game_section.split(' ');
        id_iter.next();
        let id = id_iter.next().unwrap().parse::<usize>().unwrap();

        let mut blue = 0;
        let mut green = 0;
        let mut red = 0;
        let rounds = split.next().unwrap().split("; ");
        for round in rounds {
            let bags = round.split(", ");
            for bag in bags {
                let mut bag_split = bag.split(' ');

                let amount = bag_split.next().unwrap().parse::<usize>().unwrap();
                let color = bag_split.next().unwrap();

                match color {
                    "blue" => blue = blue.max(amount),
                    "green" => green = green.max(amount),
                    "red" => red = red.max(amount),
                    _ => {}
                }
            }
        }

        Ok(Self::new(id, blue, green, red))
    }
}
