use colored::*;
use rand::*;
use termsize;
use clearscreen;

fn main() {
    let mut guessed_w: Vec<char> = Vec::new();
    let mut guessed_r: Vec<char> = Vec::new();
    greeting();
    let wort = getwrd().to_lowercase();
    println!("The word is: {wort}");
    let diffchars = diffchar(wort);
    println!("{} different characters", diffchars.len());
    let choosen_char = charchooser(&diffchars);
    println!("{choosen_char}");
    while guessed_w.len() < 11 {
        draw(&guessed_r, &guessed_w, &choosen_char)
    }
}
fn greeting() {
    println!("{}", "Welcome! This is a small implementation of the hangman game as a cli.".green());
    println!("{} {} {}", "Made by".green(), "Lovis".green().bold() , "for fun and published under the EUPL.".green());
    println!("{} {}", "It is in Version".green(), "0.1.0".green().bold());
}
fn getwrd() -> String {
    println!("");
    println!("{}","Please enter a word.".yellow());
    let i = hangman::input();
    for c in i.trim().chars() {
        match c.is_alphabetic() {
            true => {},
            false => {
                println!("{}", "Please only enter letters.".red());
                println!("{}", "Plase try again".yellow());
                getwrd();
            }
        }
    }
    if i.trim().chars().count() == 0 {
        println!("{}", "No input, please try again!".red());
        getwrd().to_lowercase()
    } else {
        i
    }
}
fn diffchar(word: String) -> Vec<char> {
    let word = word.trim();
    // println!("{}", word.chars().count());
    // if word.chars().count() == 0 {
    //     println!("{}", "No input, please try again!".red());
    //     getwrd();
    // }
    let mut diffchars: Vec<char> = Vec::new();
    for c in word.chars() {
        if diffchars.len() == 0 {
            diffchars.push(c)
        } else {
            let mut counter = 0;
            for i in 0..diffchars.len() {
                if diffchars[i] != c {
                    counter += 1;
                } 
            }
            if counter == diffchars.len() {
                diffchars.push(c);
            }
        }
    }
    diffchars
}
fn charchooser(buchstaben: &Vec<char>) -> char {
    let buchstaben = buchstaben.to_owned();
    let mut rng = rand::thread_rng();
    buchstaben[rng.gen_range(0..buchstaben.len())]
}
fn draw(guessed_r: &Vec<char>, guessed_w: &Vec<char>, choosen_char: &char) {
    clearscreen::clear().expect("Error clearing the screen");
    let breite = termsize::get().map(|size| {
        size.cols
    });
    let breite = match breite {
        Some(v) => v,
        None => panic!("Error reading terminal width"),
    };
    let höhe = termsize::get().map(|size| {
        size.rows
    });
    let höhe = match höhe {
        Some(v) => v,
        None => panic!("Error reading terminal height"),
    };
    for _ in 0..höhe/2 {
        println!("");
    }
    println!("Test");
}
