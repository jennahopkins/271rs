// A list of valid words, truncated for this example.
const WORDS: [&str; 5] = ["sator", "arepo", "tenet", "opera", "rotas"];

// ANSI color codes for colored text
// 31: Red, 32: Green, 33: Yellow
const R: u64 = 31;  // Red (letter not in word)
const G: u64 = 32;  // Green (letter in correct position)
const Y: u64 = 33;  // Yellow (letter in word but wrong position)

// Box-drawing characters for the game board
const T: &str = "┌───┬───┬───┬───┬───┐";  // Top border
const M: &str = "├───┼───┼───┼───┼───┤";  // Middle border
const B: &str = "└───┴───┴───┴───┴───┘";  // Bottom border

fn letter(a: String, c: u64) {
    /*
    Prints a single letter with a specified ANSI color.

    Args:
        a: The letter to print.
        c: The ANSI color code.
    */
    println!("| \u001b[{c}m{a}\u001b[0m ", end = "");
}

fn colors(s: String, answer: String) {
    /*
     Analyzes a guessed word and prints it with the appropriate colors.

     Args:
        s: The guessed word.
        answer: The correct answer word.
    */
    for i in 0..5 { 
        let ch: String = s.chars().nth(i).unwrap();
        let mut color_code: u64 = R.clone();
        if answer.chars().nth(i).unwrap() == ch {
            color_code = G.clone();
        } elif answer.chars().contains(ch) {
            color_code = Y.clone();
        }
        letter(ch, color_code);
    println!("|");
}

fn game(words: [&str, 6], answer: String) {
    /* Clears the screen and draws the game board with the current guesses.
    
    Args:
        words: A list of guessed words.
        answer: The correct answer word.
    */
    println!("\u001b[2J"); // Clear the screen
    println!("{:?}", T);
    for i in 0..5 {
        colors(words[i], answer);
        println!("{:?}", M);
    }
    colors(words[5], answer);
    println!("{:?}", B);
}

fn main() {
    /* 
    The main game loop.
    */
    let mut words: [&str, 6] = ["     ", "     ", "     ", "     ", "     ", "     "];
    //import random
    //answer = random.choice(WORDS)

    let mut attempts: u64 = 0;

    println!("\u001b[2J", end = ""); // Clear the screen
    println!("Use lowercase only btw.");
    while words[5] == "     " {
        let mut guess = String::new();
        std::io::stdin().read_line(&mut guess).unwrap().trim();
        if String::from(WORDS).contains(guess) {
            words[attempts] = guess;
            game(words, answer);
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

