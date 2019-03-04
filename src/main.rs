use std::io::{stdin, Read};

use rand::seq::SliceRandom;
use rand::thread_rng;

const WORD_BANK: [&str; 5] = ["enumerable", "rust", "collection", "borrowing", "iterator"];
const MAX_GUESSES: u8 = 10;

struct Game {
    answer: String,
    letters_guessed: Vec<char>,
}

// TODO: think about upper/lower cases
// TODO: limit the number of guesses
// TODO: validate input--if characters are not in the Latin alphabet
// TODO: flush stdout after each turn
// TODO: bigger words/from different source (API)
// TODO: multiplayer
// TODO: show gallows
// TODO: write tests
// TODO: code review
// TODO: display previously guessed letters
// TODO: do you want to play again?

impl Game {
    fn new() -> Game {
        let mut rng = thread_rng();
        Game {
            answer: WORD_BANK.choose(&mut rng).unwrap().to_string(),
            letters_guessed: vec![],
        }
    }

    fn setup_new_game(&mut self) {
        // the vec! can be used to initialize strings
        let mut rng = thread_rng();
        self.answer = WORD_BANK.choose(&mut rng).unwrap().to_string();
        self.letters_guessed = vec![];

        println!("Vector: {:?}", WORD_BANK);
        println!("Answer: {}", self.answer);
    }

    fn display_gameboard(&self) -> String {
        let mut display_answer = String::new();
        for c in self.answer.chars() {
            if self.letters_guessed.contains(&c) {
                display_answer.push(c);
            } else {
                display_answer.push('_');
            }
        }
        display_answer // implicit return
    }

    fn word_has_been_guessed(&self) -> bool {
        for c in self.answer.chars() {
            if !self.letters_guessed.contains(&c) {
                return false;
            }
        }
        true // implicit return
    }

    fn is_valid(&self, c: &char) -> bool {
        c.is_alphabetic()
    }

    fn get_character(&self) -> char {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();
        let mut c = buffer.to_lowercase().to_string().chars().next().unwrap();

        if self.is_valid(&c) {
            c
        } else {
            self.retry_get_character()
        }
    }

    fn retry_get_character(&self) -> char {
        println!("Please enter a letter a-z.");
        self.get_character()
    }

    fn end_game(&mut self) {
        println!("Do you want to play again? y/n");
        let c = self.get_character();
        if c == 'y' {
            self.setup_new_game();
        } else {
            return;
        }
    }

    fn take_turn(&mut self) {
        let gameboard = self.display_gameboard();
        println!("Gameboard: {}", gameboard);

        println!("Please guess your next letter.");
        let c = self.get_character();
        self.letters_guessed.push(c);
        if self.word_has_been_guessed() {
            println!("You won!");
        } else {
            self.take_turn();
        }
    }
}

fn main() {
    let mut g = Game::new();
    g.take_turn();
}
