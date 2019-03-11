use std::collections::BTreeSet;
use std::io::stdin;

use rand::seq::SliceRandom;
use rand::thread_rng;
use termion::{clear, cursor};

const WORD_BANK: [&str; 28] = [
    "attribute",
    "borrowing",
    "cargo",
    "closure",
    "collection",
    "compiler",
    "concurrency",
    "crate",
    "crustacean",
    "enumerable",
    "functional",
    "idiomatic",
    "immutable",
    "iterator",
    "lifetime",
    "macro",
    "module",
    "multithreaded",
    "nightly",
    "oxide",
    "panic",
    "polymorphism",
    "reference",
    "rust",
    "stack",
    "static",
    "trait",
    "unsafe",
];
const MAX_GUESSES: usize = 5;

struct Game {
    answer: String,
    letters_guessed: BTreeSet<char>,
    turns_left: usize,
}

impl Game {
    fn new() -> Game {
        let mut rng = thread_rng();
        Game {
            answer: WORD_BANK.choose(&mut rng).unwrap().to_string(),
            letters_guessed: BTreeSet::new(),
            turns_left: MAX_GUESSES,
        }
    }

    fn display_gameboard(&self) -> String {
        self.answer
            .chars()
            .map(|c| match self.letters_guessed.contains(&c) {
                true => c,
                false => '_',
            })
            .collect()
    }

    fn word_has_been_guessed(&self) -> bool {
        self.answer
            .chars()
            .collect::<BTreeSet<char>>()
            .is_subset(&self.letters_guessed)
    }

    fn is_valid(&self, c: &char) -> bool {
        match c {
            'a'..='z' => true,
            'A'..='Z' => true,
            _ => false,
        }
    }

    fn get_character(&self) -> char {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();
        buffer.to_lowercase().chars().next().unwrap()
    }

    fn get_character_until_valid_or_timeout(&mut self) -> Option<char> {
        let mut guess_attempts = 0;

        while guess_attempts < 10 {
            let c = self.get_character();
            if self.is_valid(&c) {
                return Some(c);
            } else {
                println!("Please enter a letter a-z.");
                guess_attempts += 1;
            }
        }

        println!("Oh no! Your invalid guesses broke our game :(");
        None
    }

    fn end_game(&mut self) {
        println!("Do you want to play again? y/n");
        match self.get_character() {
            'y' => {
                run();
            }
            _ => println!("Okay, bye!"),
        }
    }

    fn take_turn(&mut self) {
        if self.turns_left == 0 {
            println!("You ran out of guesses :(");
            println!("Your word was {}.", self.answer.to_uppercase());
            self.end_game();
        } else {
            let gameboard = self.display_gameboard();

            println!(
                "{}{}Gameboard: {}",
                clear::All,
                cursor::Goto(1, 1),
                gameboard
            );
            println!("Guesses left: {}", self.turns_left);
            println!("You've already guessed: {:?}", self.letters_guessed);
            println!("Please guess a letter.");

            if let Some(c) = self.get_character_until_valid_or_timeout() {
                if !self.letters_guessed.contains(&c) {
                    self.letters_guessed.insert(c);
                }

                if !self.answer.contains(&c.to_string()) {
                    self.turns_left -= 1;
                }

                if self.word_has_been_guessed() {
                    println!("Your word was {}.", self.answer.to_uppercase());
                    println!("You won! 🎉");
                    self.end_game();
                } else {
                    self.take_turn();
                }
            }
        }
    }
}

fn run() {
    let mut g = Game::new();
    g.take_turn();
}

fn main() {
    run();
}

#[cfg(test)]
mod tests;
