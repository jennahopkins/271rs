use std::fs;

// ANSI color codes for colored text
// 31: Red, 32: Green, 33: Yellow
const R: u64 = 31;  // Red (letter not in word)
const G: u64 = 32;  // Green (letter in correct position)
const Y: u64 = 33;  // Yellow (letter in word but wrong position)

// Box-drawing characters for the game board
const T: &str = "┌───┬───┬───┬───┬───┐";  // Top border
const M: &str = "├───┼───┼───┼───┼───┤";  // Middle border
const B: &str = "└───┴───┴───┴───┴───┘";  // Bottom border

fn read_wordlist() -> Vec<String> {
    /*
    Populate a wordlist of English words to be used in the game.

    Returns:
        Vec<String>: Vector of all valid words for the game.
    */
    let wordlist: Vec<String> = fs::read_to_string("words.txt")
        .unwrap()
        .lines()
        .map(str::to_string)
        .collect();
    wordlist
}

fn print_guess(guess: &String, colors_vec: &Vec<u64>) {
    /*
    Prints a single letter with a specified ANSI color.

    Args:
        a: The letter to print.
        c: The ANSI color code.
    */
    for i in 0..5 {
        let ch: char = guess.chars().nth(i).unwrap();
        let c: u64 = colors_vec[i];
        print!("| \u{001b}[{c}m{ch}\u{001b}[0m ");
    }
    println!("|");
}

fn assign_greens(answer: &String, guess: &String, colors_vec: &mut Vec<u64>, matched_index_vec: &mut Vec<usize>) {
    /*
    Finds which letters in the user's guess should be green.

    Args:
        answer: The correct word the user is trying to guess.
        guess: The most recent word the user guessed.
        colors_vec: Vector holding the colors each letter in the guess are.
        matched_index_vec: Which indexes in the answer are matched with a color
    */
    for i in 0..5 {
        if answer.chars().nth(i).unwrap() == guess.chars().nth(i).unwrap() {
            colors_vec[i] = G;
            matched_index_vec.push(i);
        }
    }
}

fn count_repeated_letter(answer: &String, letter: char) -> Vec<usize> {
    /*
    Finds the indexes of a repeated letter in a word.

    Args:
        answer: The correct word the user is trying to guess.
        letter: Letter whose repeats are tracked.

    Returns:
        Vec<usize>: Which indexes the letter appears in the word.
    */
    let mut indexes: Vec<usize> = Vec::new();
    let mut start = 0;
    while let Some(id) = answer[start..].find(letter) {
        indexes.push(start + id);
        start = start + id + 1;
    }
    indexes
}

fn colors(guess: &String, answer: &String) {
    /*
     Analyzes a guessed word and prints it with the appropriate colors.

     Args:
        guess: The guessed word.
        answer: The correct answer word.
    */
    let mut colors_vec: Vec<u64> = vec![R, R, R, R, R];
    let mut matched_index_vec: Vec<usize> = Vec::new();
    assign_greens(&answer, &guess, &mut colors_vec, &mut matched_index_vec);

    // logic for assigning yellows
    for i in 0..5 {
        let ch: char = guess.chars().nth(i).unwrap();
        if answer.contains(ch) && colors_vec[i] != G {
            let repeat_indexes = count_repeated_letter(&answer, ch);
            for id in repeat_indexes {
                if ! matched_index_vec.contains(&id) {
                    // index has not been assigned green or yellow yet
                    colors_vec[i] = Y;
                    matched_index_vec.push(id);
                    break; // only want one yellow to be matched to this index
                }
            }
        }
    }
    print_guess(&guess, &colors_vec);
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
    let wordlist: Vec<String> = read_wordlist();
    let mut words: Vec<String> = Vec::new();
    
    // selecting a random word as the answer
    let mut devrnd = std::fs::File::open("/dev/urandom").unwrap();
    let mut buffer = [0u8; (usize::BITS / 8) as usize];
    std::io::Read::read_exact(&mut devrnd, &mut buffer).unwrap();
    let secret = usize::from_ne_bytes(buffer);
    let answer: String = String::from(wordlist[secret % wordlist.len()].clone());

    print!("\u{001b}[2J"); // Clear the screen
    println!("Welcome to Wordle! All standard rules apply. Word list from https://github.com/tabatkins/wordle-list.git.");

    while words.len() < 6 {   // there's still guesses left
        // getting the input guess
        let mut guess = String::new();
        std::io::stdin().read_line(&mut guess).unwrap();
        guess = guess.trim().to_string().to_lowercase();

        // adding guess to vector, print board, winning logic
        if wordlist.contains(&guess) {
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
    println!("The word was {answer}.");
}

