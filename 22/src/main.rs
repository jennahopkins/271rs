// A list of valid words, truncated for this example.
//let mut WORDS: [String; 5] = ["sator".to_string(), "arepo".to_string(), "tenet".to_string(), "opera".to_string(), "rotas".to_string()];

// ANSI color codes for colored text
// 31: Red, 32: Green, 33: Yellow
const R: u64 = 31;  // Red (letter not in word)
const G: u64 = 32;  // Green (letter in correct position)
const Y: u64 = 33;  // Yellow (letter in word but wrong position)

// Box-drawing characters for the game board
const T: &str = "┌───┬───┬───┬───┬───┐";  // Top border
const M: &str = "├───┼───┼───┼───┼───┤";  // Middle border
const B: &str = "└───┴───┴───┴───┴───┘";  // Bottom border

fn letter(a: char, c: u64) {
    /*
    Prints a single letter with a specified ANSI color.

    Args:
        a: The letter to print.
        c: The ANSI color code.
    */
    print!("| \u{001b}[{c}m{a}\u{001b}[0m ");
}

fn colors(s: String, answer: String) {
    /*
     Analyzes a guessed word and prints it with the appropriate colors.

     Args:
        s: The guessed word.
        answer: The correct answer word.
    */
    for i in 0..5 { 
        let ch: char = s.chars().nth(i).unwrap();
        let mut color_code: u64 = R;
        if answer.chars().nth(i).unwrap() == ch {
            color_code = G;
        } else if answer.contains(ch) {
            color_code = Y;
        }
        letter(ch, color_code);
    }
    println!("|");
}

fn game(words: [String; 6], answer: String) {
    /* Clears the screen and draws the game board with the current guesses.
    
    Args:
        words: A list of guessed words.
        answer: The correct answer word.
    */
    println!("\u{001b}[2J"); // Clear the screen
    println!("{}", T);
    for i in 0..5 {
        colors(words[i].clone(), answer.clone());
        println!("{}", M);
    }
    colors(words[5].clone(), answer.clone());
    println!("{}", B);
}

fn main() {
    /* 
    The main game loop.
    */

    let WORDS: [String; 5] = ["sator".to_string(), "arepo".to_string(), "tenet".to_string(), "opera".to_string(), "ropas".to_string()];

    let mut words: [String; 6] = ["     ".to_string(), "     ".to_string(), "     ".to_string(), "     ".to_string(), "     ".to_string(), "     ".to_string()];
    
    //let mut buffer = [0u8; (usize::BITS / 8) as usize];


    //import random
    //answer = random.choice(WORDS)
    
    let answer = String::from("sator");

    let mut attempts: usize = 0;

    print!("\u{001b}[2J"); // Clear the screen
    println!("Use lowercase only btw.");
    while words[5] == "     ".to_string() {
        let mut guess = String::new();
        std::io::stdin().read_line(&mut guess).unwrap();
        guess = guess.trim().to_string();
        println!("{:?}", guess);
        if WORDS.contains(&guess) {
            words[attempts] = guess.clone();
            game(words.clone(), answer.clone());
            if guess == answer {
              println!("Winner!");  
              return;
            }
            attempts += 1;
        } else {
            println!("Not a valid word!");
        }  
    }
    println!("Game over :(");
}

