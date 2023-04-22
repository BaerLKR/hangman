use colored::*;

pub mod lib;

fn main() {
    greeting();
    let wort = getwrd().to_lowercase();
    println!("The word is: {wort}");
    let diffchar = diffchar(wort);
    println!("{:?}", diffchar);
}
fn greeting() {
    println!("{}", "Welcome! This is a small implementation of the hangman game as a cli.".green());
    println!("{} {} {}", "Made by".green(), "Lovis".green().bold() , "for fun and published under the EUPL.".green());
    println!("{} {}", "It is in Version".green(), "0.1.0".green().bold());
}
fn getwrd() -> String {
    println!("");
    println!("{}","Please enter a word.".yellow());
    let i = lib::input();
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
