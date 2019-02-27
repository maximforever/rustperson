use std::io::stdin;

use rand::thread_rng;
use rand::seq::SliceRandom;

const WORD_BANK: [&str; 5] = ["enumerable", "rust", "collection", "borrowing", "iterator" ];
const MAX_GUESSES: u8 = 10;


struct Game {
	answer: String,
	letters_guessed: Vec<char>
}

impl Game {
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
		display_answer			// implicit return
	}


	fn word_has_been_guessed(&self) -> bool {
		for c in self.answer.chars() {
			if !self.letters_guessed.contains(&c) {
				return false;
			}
		}
		true			// implicit return
	}


	fn take_turn(&mut self) {
		let gameboard = self.display_gameboard();
		println!("Gameboard: {}", gameboard);

		println!("Please guess your next letter.");
		let mut s = String::new();
		stdin().read_line(&mut s); // read in user input string
		s = s.trim().to_string();

	    if s.len() != 1 {
	    	println!("Please only enter one alphabetical character.");
	    	self.take_turn();
	    } else {
	    	let c = s.pop().unwrap();
	    	// TODO: if characters are not in the Latin alphabet
	    	if !c.is_alphabetic() {
	    		println!("Please enter an alphabetical character.");
	    		self.take_turn();
	    	} else {
	    		self.letters_guessed.push(c);
	    		if self.word_has_been_guessed() {
	    			println!("You won!");
	    		} else {

		            self.take_turn();
	    		}
	    	}
	    }
	}

}

fn main() {
    let mut g = Game {
    	answer: String::from(""),
		letters_guessed: vec![]
    };

    g.setup_new_game();
    g.take_turn();
}


