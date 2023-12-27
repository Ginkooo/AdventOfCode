use lazy_static::lazy_static;
use std::cell::OnceCell;
use std::{borrow::BorrowMut, collections::HashMap, fs};

lazy_static! {
    static ref WORDS_TO_NUMBERS_MAP: HashMap<&'static str, &'static str> = [
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]
    .iter()
    .cloned()
    .collect();
}

fn main() {
    let lines = fs::read_to_string("resources/input1.txt").unwrap();
    let mut sum = 0;
    for line in lines.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let line = line.to_string();
        let line = replace_words_with_numbers(line);
        println!("");
        let numbers_only = line
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>();
        let mut number = String::new();
        number.push(numbers_only.chars().nth(0).unwrap());
        number.push(numbers_only.chars().last().unwrap());
        let number = number.parse::<i32>().unwrap();
        sum += number;
    }
    println!("{}", sum);
}

fn replace_words_with_numbers(line: String) -> String {
    let mut ret = line.chars().collect::<Vec<char>>();
    let mut digit_words = vec![];
    for (word, &value) in WORDS_TO_NUMBERS_MAP.iter() {
        let matches: Vec<(usize, &str)> = line.match_indices(word).collect();
        if matches.is_empty() {
            continue;
        }
        digit_words.extend(matches.iter().map(|m| (m.0, m.1, value)));
    }
    digit_words.sort_by_key(|word| word.0);
    println!("{line} => {:?}", digit_words);
    for digit_word in digit_words {
        ret[digit_word.0] = digit_word.2.chars().nth(0).unwrap();
        ret[digit_word.0 + 1] = digit_word.2.chars().nth(0).unwrap();
    }
    let ret = ret.into_iter().collect::<String>();
    println!("{line} -> {ret}");
    ret
}
