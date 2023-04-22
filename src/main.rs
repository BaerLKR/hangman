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
    let diffchars = diffchar(&wort);
    println!("{} different characters", diffchars.len());
    let choosen_char = charchooser(&diffchars);
    guessed_r.push(choosen_char);
    println!("{choosen_char}");
    loop {
        draw(&guessed_r, &guessed_w, &choosen_char, &wort);
        match check_end(&guessed_r, &guessed_w, &diffchars) {
            State::Cont => {},
            State::Win => break,
            State::Loose => break,
        };
        guess(&mut guessed_r, &mut guessed_w, &diffchars);
    }
}
fn greeting() {
    println!("{}", "Welcome! This is a small implementation of the hangman game as a cli.".green());
    println!("{} {} {}", "Made by".green(), "Lovis".green().bold() , "for fun and published under the EUPL.".green());
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
    } else if i.trim().chars().count() == 1 {
        println!("{}", "Word cannot contain only one letter, try again.".red());
        getwrd().to_lowercase()
    } else {
        i.to_lowercase()
    }
}
fn diffchar(word: &String) -> Vec<char> {
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
fn margin() {
    let höhe = termsize::get().map(|size| {
        size.rows
    });
    let höhe = match höhe {
        Some(v) => v,
        None => panic!("Error reading terminal height"),
    };
    for _ in 2..höhe/2 {
        println!("");
    }
}
fn margin_l(string: &usize) {
    let breite = termsize::get().map(|size| {
        size.cols
    });
    let breite = match breite {
        Some(v) => v,
        None => panic!("Error reading terminal width"),
    };
    for _ in 0..(breite/2 - string.to_owned() as u16) {
        print!(" ");
    }
}
fn draw(guessed_r: &Vec<char>, guessed_w: &Vec<char>, choosen_char: &char, word: &String) {
    clearscreen::clear().expect("Error clearing the screen");
    margin();
    margin_l(&word.chars().count());
    for c in word.trim().chars() {
        if c == choosen_char.to_owned() {
            print!("{} ", choosen_char)
        } else {
            let mut check = false;
            for i in guessed_r {
                if i.to_owned() == c {
                    print!("{} ", String::from(c).green());
                    check = true;
                }
            }
            if !check {
                print!("_ ");
            }
        }
    }
    print!("\n");
    print!("\n");
    margin_l(&guessed_w.to_owned().len());
    for w in guessed_w {
        print!("{} ", String::from(w.to_owned()).red());
    }
    println!("");
    println!("");
    println!("");
}
fn guess(right: &mut Vec<char>, wrong: &mut Vec<char>, diffchars: &Vec<char>) {
    println!("");
    println!("");
    println!("Next guess?");
    let i = hangman::input();
    let i = i.to_lowercase();
    let c = i.chars().nth(0).unwrap();
    if i.trim().chars().count() > 1 {
        println!("{}", "Please don't enter more than one letter".red());
        guess(right, wrong, diffchars);
    } else if right.contains(&c) || wrong.contains(&c) {
        println!("{}", "Already guessed that letter".yellow());
        guess(right, wrong, diffchars);
    } else {
        let mut counter = 0;
        for n in diffchars {
            if c == n.to_owned() {
                right.push(c);
            } else {
                counter += 1;
            }
        }
        if counter == diffchars.len() {
            wrong.push(c);
        }
    }
}
enum State {
    Win,
    Loose,
    Cont,
}
fn check_end(right: &Vec<char>, wrong: &Vec<char>, diffchars: &Vec<char>) -> State {
    let re = if right.len() == diffchars.len() {
        println!("{}", "GG!".green().bold());
        State::Win
    } else if wrong.len() == 11 {
        println!("{}", "U LOST!".red().bold());
        State::Loose
    } else {
        State::Cont
    };
    re
}
