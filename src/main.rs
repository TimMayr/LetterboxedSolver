use std::ascii::AsciiExt;
use std::collections::HashSet;
use std::io::Error;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let game = Game::from(&args[1..]);

    match game {
        Game::LetterBoxed(g) => {
            let res = g.solve();
            dbg!(res);
        }
        Game::Error => {
            println!("Game does not exist")
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Game {
    LetterBoxed(LetterBoxed),
    Error,
}

impl From<&[String]> for Game {
    fn from(args: &[String]) -> Self {
        let game_name = &args[0];

        match game_name.as_str() {
            "letterboxed" => return Self::LetterBoxed(LetterBoxed::new(&args[1..])),
            _ => {}
        }

        Self::Error
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LetterBoxed {
    sides: Vec<Vec<char>>,
}
impl LetterBoxed {
    pub fn new(args: &[String]) -> Self {
        let mut sides = Vec::new();
        for side_raw in args {
            let mut side = Vec::new();
            for c in side_raw.chars() {
                side.push(c.to_ascii_lowercase());
            }
            sides.push(side);
        }

        Self { sides }
    }

    pub fn solve(&self) -> Vec<String> {
        let dict = read_file("./words.txt");

        if dict.is_err() {
            println!("Dictionary wasn't found");
            panic!();
        }

        let mut dict = dict.unwrap();
        let mut possible_words = Vec::new();
        let chars = self.get_all_chars();

        'outer: for word in dict {
            let word_chars: Vec<char> = word.chars().collect();

            if word.len() < 3 {
                continue;
            }

            for current_char in &word_chars {
                if !chars.contains(current_char) {
                    continue 'outer;
                }
            }

            for (i, char) in word_chars.iter().enumerate() {
                let next_char = word_chars.get(i + 1);

                if next_char.is_none() {
                    continue;
                }

                let valid_connections = self.get_valid_connections(char);

                if !valid_connections.contains(next_char.unwrap()) {
                    continue 'outer;
                }
            }

            possible_words.push(word.clone());
        }

        self.find_recursive(&possible_words, possible_words.clone(), Vec::new())
    }

    fn find_recursive(
        &self,
        possible_words: &Vec<String>,
        current_possible_words: Vec<String>,
        candidate: Vec<String>,
    ) -> Vec<String> {
        let mut best = u32::MAX;
        let mut best_candidate = Vec::new();

        for word in current_possible_words {
            let mut curr = candidate.clone();
            curr.push(word.clone());
            let missing = self.check_missing_chars(&curr);

            if missing <= 0 {
                return curr;
            }

            if missing < best {
                best = missing;
                best_candidate = curr.clone();
            }
        }

        let mut current_possible_words = Vec::new();
        let last_char = best_candidate.last().unwrap().chars().last().unwrap();

        for word in possible_words {
            if word.starts_with(last_char) {
                current_possible_words.push(word.clone());
            }
        }

        self.find_recursive(possible_words, current_possible_words, best_candidate)
    }

    fn get_valid_connections(&self, c: &char) -> Vec<char> {
        let mut res = Vec::new();
        for side in &self.sides {
            if !side.contains(c) {
                for x in side {
                    res.push(*x);
                }
            }
        }

        res
    }

    fn get_all_chars(&self) -> Vec<char> {
        let mut res = Vec::new();
        for side in &self.sides {
            for x in side {
                res.push(x.to_ascii_lowercase());
            }
        }

        res
    }

    fn check_missing_chars(&self, words: &Vec<String>) -> u32 {
        let chars = self.get_all_chars();
        let mut used_chars = HashSet::new();

        for x in words {
            for x in x.chars() {
                used_chars.insert(x);
            }
        }

        return (chars.len() - used_chars.len()) as u32;
    }
}

pub fn read_file(path: &'static str) -> Result<Vec<String>, Error> {
    let read = fs::read_to_string(path);
    match read {
        Ok(content) => {
            let mut res = Vec::new();
            for x in content.split_terminator("\n") {
                res.push(x.to_ascii_lowercase());
            }

            Ok(res)
        }
        Err(e) => Err(e),
    }
}
