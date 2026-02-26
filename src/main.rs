use std::collections::{HashMap, HashSet};
use std::io::Error;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let game = Game::from(&args[1..]);

    match game {
        Game::LetterBoxed(g) => {
            let res = g.solve();
            println!("{:#?}", res)
        }
        Game::SpellingBee(g) => {
            let res = g.solve();
            println!("{:#?}", res)
        }
        Game::Crossplay(g) => {
            let res = g.solve();
            println!("{:?}", res)
        }
        Game::Error => {
            println!("Game does not exist")
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Game {
    LetterBoxed(LetterBoxed),
    SpellingBee(SpellingBee),
    Crossplay(Crossplay),
    Error,
}

impl From<&[String]> for Game {
    fn from(args: &[String]) -> Self {
        let game_name = &args[0];

        match game_name.as_str() {
            "letterboxed" => return Self::LetterBoxed(LetterBoxed::new(&args[1..])),
            "spellingbee" => return Self::SpellingBee(SpellingBee::new(&args[1..])),
            "crossplay" => return Self::Crossplay(Crossplay::new(&args[1..])),
            _ => {}
        }

        Self::Error
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SpellingBee {
    letters: Vec<char>,
    center: char,
}

impl SpellingBee {
    pub fn new(args: &[String]) -> Self {
        let mut letters = Vec::new();
        for c in args[1].chars() {
            letters.push(c.to_ascii_lowercase());
        }

        let center = args[0].chars().nth(0).unwrap().to_ascii_lowercase();
        letters.push(center);

        Self { letters, center }
    }

    pub fn solve(&self) -> Vec<String> {
        let dict = read_file("./words_big.txt");

        if dict.is_err() {
            println!("Dictionary wasn't found");
            panic!();
        }

        let dict = dict.unwrap();

        let words: Vec<&String> = dict
            .iter()
            .filter(|s| {
                s.contains(self.center)
                    && s.len() > 8
                    && Self::contains_only_from(s, &self.letters[..])
            })
            .collect();

        let mut res = Vec::new();
        for x in words {
            res.push(x.clone());
        }

        res
    }

    fn contains_only_from(s: &str, allowed: &[char]) -> bool {
        s.chars().all(|c| allowed.contains(&c))
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

        let dict = dict.unwrap();
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

            if missing == 0 {
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

        (chars.len() - used_chars.len()) as u32
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Crossplay {
    letters: HashMap<char, u32>,
    pattern: Vec<char>,
}

impl Crossplay {
    fn new(args: &[String]) -> Crossplay {
        let mut allowed_char_counts: HashMap<char, u32> = HashMap::new();

        for c in args[0].chars() {
            let old = if allowed_char_counts.contains_key(&c) {
                *allowed_char_counts.get(&c).unwrap()
            } else {
                0u32
            };

            allowed_char_counts.insert(c, old + 1);
        }

        Self {
            letters: allowed_char_counts,
            pattern: args[1].chars().to_owned().collect(),
        }
    }
}

impl Crossplay {
    pub fn solve(&self) -> Vec<String> {
        let dict = read_file("./words_big.txt");

        if dict.is_err() {
            println!("Dictionary wasn't found");
            panic!();
        }

        let dict = dict.unwrap();
        let mut possible_words = Vec::new();
        let mut res = Vec::new();

        for word in dict {
            if word.len() == self.pattern.len() {
                possible_words.push(word)
            }
        }

        for word in possible_words {
            let mut is_possible = true;

            let mut used_letters: HashMap<char, u32> = HashMap::new();

            for (i, c) in self.pattern.iter().enumerate() {
                if *c != '.' {
                    is_possible = word.chars().nth(i).unwrap() == *c;
                }
            }

            for (i, c) in word.chars().enumerate() {
                if self.pattern[i] != '.' {
                    continue;
                }

                let old = if used_letters.contains_key(&c) {
                    *used_letters.get(&c).unwrap()
                } else {
                    0
                };

                used_letters.insert(c, old + 1);
            }

            is_possible &= Self::check_if_subset(&self.letters, &used_letters);

            if is_possible {
                res.push(word)
            }
        }

        res
    }

    fn check_if_subset(
        playable_chars: &HashMap<char, u32>,
        chars_to_play: &HashMap<char, u32>,
    ) -> bool {
        let mut blanks = *playable_chars.get(&'*').unwrap_or(&0);

        for (c, i) in chars_to_play {
            if playable_chars.get(&c).unwrap_or(&0) + blanks < *i {
                return false;
            } else if *playable_chars.get(&c).unwrap_or(&0) < *i {
                blanks -= 1;
            }
        }

        true
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
