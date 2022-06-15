use std::io::stdin;

mod dictionary;
mod wordle;
mod word_finder;

// 1 : Wordle (Least guesses possible)
// 2 : Survivle (Most guesses possible)
fn set_mode () -> bool {
    println!("1 for Wordle, 2 for Survivle: ");
    let mut mode_str = String::new();
    let valid = false;
    while !valid {
        match stdin().read_line(&mut mode_str) {
            Ok(_) => { }
            Err(_) => {
                println!("something went wrong");
                break;
            }
        }
        mode_str = mode_str.trim().to_string();
        if !(mode_str.eq("1") || mode_str.eq("2")) {
            println!("INVALID");
        }
        else { return mode_str.eq("1"); }
    }
    false
}

pub fn run () -> Result<(), ()> {

    let mode = set_mode();
    println!("{}",
    match mode {
        true => "WORDLE:",
        false => "SURVIVLE:"
    });

    let dict_path = "resources/hiddenwords.txt";

    let dict: dictionary::Dictionary = dictionary::Dictionary::new(dict_path);
    let mut code_str = String::new();
    let mut finder = word_finder::Finder::new(dict.words.len() as u32);
    
    // The best starting word can be found by the program, 
    let mut word = dict.get_word(finder.get_word(&dict, mode));
    //or it can be explicitly defined
    // let mut word = dict.get_word(dict.index_of("CRANE").unwrap());

    let mut gameover = false;

    let mut rounds = 1;
    
    // Basic program loop:
    // 1. Guess the word with the lowest (or highest) standard deviation of possible answers per each code.
    // 2. Take in the Wordle response from the user.
    // 3. Evaluate and remove words from the dictionary that would not have returned the response if they were the answer.
    // 4. Repeat
    while !gameover {
        // Print the next guess
        println!("{}", dictionary::string_from_char_arr(word));

        // Handle user input of the 'code'
        let mut code_valid = false;
        while !code_valid{
            code_str = String::new();
            match stdin().read_line(&mut code_str) {
                Ok(_) => {}
                Err(_) => {
                    println!("something went wrong");
                    break;
                }
            }

            // Cast to uppercase, then take everything but the \n at the end
            code_str = code_str.to_uppercase().trim().to_string();
            if wordle::is_valid_code(&code_str) {
                code_valid = true;
            } else {
                println!("invalid code: {}", code_str);
            }
        }
        if code_str.eq("OOOOO") { gameover = true; } // 5 Greens
        else {
            // Convert from a human readable string to a base 3 number
            // i.e. X-XOX becomes 21202
            let mut code: u16 = 0;
            for i in 0..code_str.len() {
                let c = code_str.chars().nth(i).unwrap();
                code += match c {
                    'O' => 0,
                    '-' => 1,
                    'X' => 2,
                    _ => {
                        // Already checked for validity above, so this will never run
                        // Borrow checker throws a compile error if there's no _ pattern
                        panic!("this can't happen lol");
                    }
                } * 10_u32.pow(i as u32) as u16;
            }

            // Print out the word again, with the corresponding response code from wordle
            println!("{}: {}", dictionary::string_from_char_arr(word), code_str);
            // Remove everything that doesn't match that code with the same word
            finder.rmv_words(word, code, &dict);
            // How many solutions are left?
            println!("{} solutions", finder.remaining_words.len());
            // New word
            word = dict.get_word(finder.get_word(&dict, mode));
            rounds += 1;
        }
    }
    if mode {
        println!("LETS GOOOO");
    }
    else {println!("We survived {} rounds", rounds)}
    println!("Press enter to exit...");
    let mut exit = String::new();
    match stdin().read_line(&mut exit) {
        Ok(_) => {}
        Err(_) => {
            println!("how did you mess this up");
        }
    }

    Ok(())

}
