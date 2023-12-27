use std::collections::{HashMap, HashSet};

trait Engine {
    fn get_parts(&self) -> Vec<(i32, i32, char)>;
    fn get_numbers(&self) -> HashMap<(i32, i32), i32>;
    fn get_parts_with_numbers(&self) -> Vec<(char, Vec<i32>)>;
}

impl Engine for Vec<String> {
    fn get_parts_with_numbers(&self) -> Vec<(char, Vec<i32>)> {
        let mut ret = vec![];
        let parts = self.get_parts();
        for part in parts {
            dbg!(part);
            let neighbour_fields = vec![
                (-1, -1),
                (0, -1),
                (1, -1),
                (-1, 0),
                (1, 0),
                (-1, 1),
                (0, 1),
                (1, 1),
            ];
            for field in neighbour_fields {
                let mut x = part.0 + field.0;
                if x < 0 {
                    x = 0;
                }
                let mut y = part.1 + field.1;
                if y < 0 {
                    y = 0;
                }
                let field_value = self[x as usize].chars().nth(y as usize).unwrap();
                dbg!(field_value);
            }
        }
        ret
    }

    fn get_parts(&self) -> Vec<(i32, i32, char)> {
        let mut chars = vec![];
        for (i, line) in self.iter().enumerate() {
            for (j, ch) in line.chars().enumerate() {
                if !ch.is_ascii_alphanumeric() && ch != '.' {
                    chars.push((j as i32, i as i32, ch));
                }
            }
        }
        chars
    }

    fn get_numbers(&self) -> HashMap<(i32, i32), i32> {
        let mut numbers = HashMap::new();
        let mut current_number_str_buffer = String::new();
        for (y, line) in self.iter().enumerate() {
            let mut current_number = (0, 0, 0);
            let mut number_detected = false;
            for (x, ch) in line.char_indices() {
                if ch.is_ascii_digit() {
                    if !number_detected {
                        current_number.0 = x as i32;
                        current_number.1 = y as i32;
                    }
                    current_number_str_buffer.push(ch);
                    number_detected = true;
                } else {
                    if number_detected {
                        current_number.2 = current_number_str_buffer
                            .parse::<i32>()
                            .unwrap_or_else(|_| panic!("{}", current_number_str_buffer));
                        numbers.insert((current_number.0, current_number.1), current_number.2);
                        current_number_str_buffer.clear();
                    }
                    number_detected = false;
                }
            }
        }
        numbers
    }
}

fn main() {
    let input = std::fs::read_to_string("resources/example3.txt").unwrap();
    let input = input.trim();
    let schema = input
        .lines()
        .map(|l| l.to_string())
        .collect::<Vec<String>>();
    dbg!(schema.get_parts_with_numbers());
}
