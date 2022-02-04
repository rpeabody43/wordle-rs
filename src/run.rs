use std::io::stdin;
use rand::Rng;

mod game;
mod dictionary;
mod wordle;
mod word_finder;

pub fn run () -> Result<String, ()> {
    // let tests: Vec<(&str, &str)> = vec![("XXXXX", "KNIFE"),
    //                                     ("XOXXX", "ROBOT"),
    //                                     ("XOX-X", "POACH"),
    //                                     ("OOOXX", "COUCH"),
    //                                     ("OOOOO", "COULD")];
    // // let tests: Vec<(&str, &str)> = vec![("XOX-X", "POACH")];
    // game::test(&tests);

    //TODO more whiteboard
    let dict = dictionary::Dictionary::new("src/words.txt");
    let mut word = dict.get_random_word();
    let mut code = String::new();
    let mut game = game::Game::new();

    while !code.eq("OOOOO") {
        println!("{}", word);
        let mut code_valid = false;
        while !code_valid{
            code = String::new();
            let _ = stdin().read_line(&mut code).unwrap();
            code = code.to_uppercase();
            if wordle::is_valid_code(&code) {
                code_valid = true;
            }
            else {
                println!("invalid code");
            }
        }
        print!("{}: {}", word, code);
        game.update(&code, &word);
    }

    Ok(word)
}
