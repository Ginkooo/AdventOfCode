use std::collections::HashMap;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum InvalidGame {
    #[error("invalid game")]
    InvalidSet,
}
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Color {
    GREEN,
    BLUE,
    RED,
}

#[derive(Debug)]
struct Set {
    samples: Vec<(i32, Color)>,
}

impl Set {
    pub fn get_max_color(&self) -> HashMap<Color, i32> {
        let mut ret = HashMap::new();
        for (count, color) in self.samples.iter() {
            let entry = ret.entry(*color).or_insert(0);
            if count > entry {
                *entry = *count;
            }
        }
        ret
    }
}

trait ColorMap {
    fn power(&self) -> i32;
    fn sum(&self) -> i32;
}

impl ColorMap for HashMap<Color, i32> {
    fn power(&self) -> i32 {
        self.values().fold(1, |acc, elem| acc * elem)
    }

    fn sum(&self) -> i32 {
        self.values().sum()
    }
}

#[derive(Debug)]
struct Game {
    id: i32,
    sets: Vec<Set>,
}

trait Sets {
    fn count_max_colors(&self) -> HashMap<Color, i32>;
    fn sum_powers(&self) -> i32;
}

impl Sets for Vec<Set> {
    fn count_max_colors(&self) -> HashMap<Color, i32> {
        let mut ret = HashMap::new();
        for set in self {
            for (color, count) in set.get_max_color() {
                let entry = ret.entry(color).or_insert(0);
                if count > *entry {
                    *entry = count;
                }
            }
        }
        ret
    }

    fn sum_powers(&self) -> i32 {
        self.iter().map(|set| set.get_max_color().power()).sum()
    }
}

fn is_set_valid(set: &Set) -> Result<(), InvalidGame> {
    let mut color_max = HashMap::new();
    color_max.insert(Color::RED, 12);
    color_max.insert(Color::GREEN, 13);
    color_max.insert(Color::BLUE, 14);
    for (count, color) in set.samples.iter() {
        let max = color_max[color];
        if *count > max {
            return Err(InvalidGame::InvalidSet);
        }
    }
    Ok(())
}

fn parse_line(line: &str) -> Result<Game, InvalidGame> {
    let game_id = line
        .split(':')
        .nth(0)
        .unwrap()
        .split(' ')
        .last()
        .unwrap()
        .parse::<i32>()
        .unwrap();
    let sets_part = line.split(':').last().unwrap().trim();
    let sets = sets_part.split(';');
    let mut all_sets = vec![];
    for set in sets {
        let samples = set.split(',').map(|s| s.trim());
        let mut all_samples = vec![];
        for sample in samples {
            let (count, color) = sample.split_once(' ').unwrap();
            let count = count.parse::<i32>().unwrap();
            let color = match color {
                "red" => Color::RED,
                "green" => Color::GREEN,
                "blue" => Color::BLUE,
                _ => {
                    panic!("bad color")
                }
            };
            all_samples.push((count, color));
        }
        let set = Set {
            samples: all_samples,
        };
        all_sets.push(set);
    }
    let game = Game {
        id: game_id,
        sets: all_sets,
    };
    Ok(game)
}

fn main() {
    let input = std::fs::read_to_string("resources/input2.txt").unwrap();
    let input = input.trim();
    let mut sum_of_powers = 0;
    for line in input.lines() {
        let game = parse_line(line);
        match game {
            Ok(game) => {
                let power: i32 = game.sets.count_max_colors().power();
                sum_of_powers += power;
                println!("{line}");
                dbg!(power);
                dbg!(game.sets.count_max_colors());
            }
            Err(_) => {
                eprintln!("We have an incorrect game here")
            }
        }
    }
    println!("{sum_of_powers}");
}
