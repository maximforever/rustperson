use std::{iter, io::{stdin, stdout, Write}};

use rand::Rng;

const WORD_BANK: [&str; 5] = ["enumerable", "rust", "collection", "borrowing", "iterator" ];
const MAX_GUESSES: u8 = 10;

fn turn(letters_already_guessed: &mut Vec<char>, answer: &str) {
	println!("Please guess your next letter.");
	let mut s = String::new();
	stdin().read_line(&mut s); // read in user input string
	s = s.trim().to_string();

    if s.len() != 1 {
    	println!("Please only enter one alphabetical character.");
    } else {
    	let c = s.pop().unwrap();
    	// TODO: if characters are not in the Latin alphabet
    	if !c.is_alphabetic() {
    		println!("Please enter an alphabetical character.");
    	} else {
    		letters_already_guessed.push(c);

    		let gameboard = display_gameboard(answer, &letters_already_guessed);
            println!("Gameboard: {}", gameboard);
    	}
    }
    
    turn(letters_already_guessed, answer);
}

fn main() {
    

    println!("Rustman is a game where the robot gets more and more rusty as you guess letters");


    /*
    
    	1. pick random word from array and store it as a constant string
    	2. have 1 other vector for previously guessed letters
    	3. on each "turn", check if all the letters of the word have been guessed
    		check if we've hit the max guess limit

	    		- if NO: display:
	    			a. word so far
	    			b. letters you've guessed
	    		- else "you've won! play again?"
    		- if yes, reset, run this whole thing again

     */
    
	// the vec! can be used to initialize strings
    let answer = rand::thread_rng().choose(&WORD_BANK).unwrap();	
    let mut letters_already_guessed: Vec<char> = vec![]; 

    println!("Vector: {:?}", WORD_BANK);
    println!("Answer: {}", answer);

    let gameboard = display_gameboard(answer, &letters_already_guessed);
    println!("Gameboard: {}", gameboard);

    turn(&mut letters_already_guessed, answer);
}



fn display_gameboard(answer: &str, letters_already_guessed: &Vec<char>) -> String {
	let mut display_answer = String::new();
	for c in answer.chars() {
		if letters_already_guessed.contains(&c) {
			display_answer.push(c);
		} else {
			display_answer.push('_');
		}
	}
	display_answer
}
