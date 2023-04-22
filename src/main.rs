use colored::*;
use rand::*;

fn main() {
    greeting();
    let wort = getwrd().to_lowercase();
    println!("The word is: {wort}");
    let diffchars = diffchar(wort);
    println!("{} different characters", diffchars.len());
    let choosen_char = charchooser(&diffchars);
    println!("{choosen_char}");
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
    i
}
fn diffchar(word: String) -> Vec<char> {
    let word = word.trim();
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
