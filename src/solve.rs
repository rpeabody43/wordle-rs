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
    let mut word = dict.get_word(finder.get_word(&dict, mode));
    // let mut word = dict.get_word(dict.index_of("CRANE").unwrap());

    let mut gameover = false;

    let mut rounds = 1;
    while !gameover {
        println!("{}", dictionary::string_from_char_arr(word));
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
        if code_str.eq("OOOOO") { gameover = true; }
        else {
            let mut code: u16 = 0;
            for i in 0..code_str.len() {
                let c = code_str.chars().nth(i).unwrap();
                code += match c {
                    'O' => 0,
                    '-' => 1,
                    'X' => 2,
                    _ => {
                        panic!("this can't happen lol");
                    }
                } * 10_u32.pow(i as u32) as u16;
            }
            println!("{}: {}", dictionary::string_from_char_arr(word), code_str);
            finder.rmv_words(word, code, &dict);
            println!("{} solutions", finder.remaining_words.len());
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
