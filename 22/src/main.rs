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


fn letter(a: char, c: u64) {
    /*
    Prints a single letter with a specified ANSI color.

    Args:
        a: The letter to print.
        c: The ANSI color code.
    */
    print!("| \u{001b}[{c}m{a}\u{001b}[0m ");
}

fn assign_greens(answer: &String, guess: &String, colors_vec: &[String], matched_index_vec: &[usize]) {

}

fn colors(s: &String, answer: &String) {
    /*
     Analyzes a guessed word and prints it with the appropriate colors.

     Args:
        s: The guessed word.
        answer: The correct answer word.
    */
    let mut color_vec: Vec<char> = vec!['R', 'R', 'R', 'R', 'R']

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

fn game(words: &[String], answer: &String) {
    /* Clears the screen and draws the game board with the current guesses.
    
    Args:
        words: A list of guessed words.
        answer: The correct answer word.
    */
    println!("\u{001b}[2J"); // Clear the screen
    println!("{}", T);

    // printing the board's first 5 guesses
    for i in 0..5 {
        if i < words.len() {   // a word has been guessed
            colors(&words[i], &answer);
        } else {    // not guessed yet
            println!("|   |   |   |   |   |");
        }
        println!("{}", M);
    }

    // printing the 6th guess
    if words.len() == 6 {   // last guess exists
        colors(&words[5], &answer);
    } else {   // not guessed yet
        println!("|   |   |   |   |   |");
    }

    println!("{}", B);
}

fn main() {
    /* 
    The main game loop.
    */
    let mut words: Vec<String> = Vec::new();
    
    let mut devrnd = std::fs::File::open("/dev/urandom").unwrap();
    let mut buffer = [0u8; (usize::BITS / 8) as usize];
    std::io::Read::read_exact(&mut devrnd, &mut buffer).unwrap();
    let secret = usize::from_ne_bytes(buffer);
    let answer : String = String::from(WORDS[secret % WORDS.len()]);

    print!("\u{001b}[2J"); // Clear the screen
    println!("Use lowercase only btw.");

    while words.len() < 6 {   // there's still guesses left
        // getting the input guess
        let mut guess = String::new();
        std::io::stdin().read_line(&mut guess).unwrap();
        guess = guess.trim().to_string();

        // adding guess to vector, print board, winning logic
        if WORDS.contains(&guess.as_str()) {
            words.push(guess.clone());
            game(&words, &answer);
            if guess == answer {
              println!("Winner!");  
              return;
            }
        } else {   // guess not in WORDS array
            println!("Not a valid word!");
        }  

    }   // no more guesses left
    println!("Game over :(");
}

