use std::collections::HashMap;
use crate::solve::dictionary;
use crate::solve::game;
use crate::solve::wordle::match_wordle;


pub fn get_word<'a> (game: &game::Game, dict: &'a dictionary::Dictionary) -> &'a String {
    //TODO properly implement get_word
    let letters: &HashMap<char, (usize, bool)> = game.get_letter_map();
    let exact: &[String; 5] = game.get_exact_positions();

    let mut word = dict.get_word(0);
    let mut loops: usize = 0;
    while !match_wordle(word, letters, exact) {
        // TODO figure out how to remove things from dict
        //  Maybe return tuple that looks like (&'a String, Vec<&'a String>)
        word = dict.get_word(loops);
        loops += 1;
    }
    // println!("{}", loops);
    word
}

