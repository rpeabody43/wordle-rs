use std::io::stdin;

mod game;
mod dictionary;
mod wordle;
mod word_finder;

pub fn run () -> Result<(), ()> {
    // let tests: Vec<(String, String)> = vec![("XXXXX".to_string(), "AAHED".to_string()),
    //                                         ("XOXX-".to_string(), "BIBBS".to_string()),
    //                                         // ("-XO-O".to_string(), "ADEPT".to_string()),
    //                                         // ("OOOOO".to_string(), "PLEAT".to_string()),
    // ];
    // game::test(&tests);

    let dict_path = "resources/words.txt";

    let dict = dictionary::Dictionary::new(dict_path);
    let mut word = dict.get_word(dict.index_of("CRANE").unwrap());
    let mut code = String::new();
    let mut game = game::Game::new();

    let mut gameover = false;

    while !gameover {
        println!("{}", word);
        let mut code_valid = false;
        while !code_valid{
            code = String::new();
            match stdin().read_line(&mut code) {
                Ok(_) => {}
                Err(_) => {
                    println!("something went wrong");
                    break;
                }
            }

            // Cast to uppercase, then take everything but the \n at the end
            code = code.to_uppercase().trim().to_string();
            if wordle::is_valid_code(&code) {
                code_valid = true;
            } else {
                println!("invalid code: {}", code);
            }
        }
        if code.eq("OOOOO") { gameover = true; }
        else {
            println!("{}: {}", word, code);
            game.update(&code, &word);
            word = word_finder::get_word(&game, &dict);
        }
    }
    println!("LETS GOOOO");
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
