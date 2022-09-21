pub(crate) mod dictionary;
mod word_finder;

// OOP makes this harder than it has to be
// but it's the only way I can think to separate frontend stuff and the solving algo
pub struct Session {
    mode: bool,
    dict: dictionary::Dictionary,
    finder: word_finder::Finder,
    pub word_idx: usize,
    pub gameover: bool,
    rounds: i32,
}

impl Session {
    pub fn new (mode: bool, dict_path: &str) -> Self {
        let dict = dictionary::Dictionary::new(dict_path);
        let finder = word_finder::Finder::new(dict.words.len() as u32);
        // The best starting word can be found by the program,
        let word_idx = finder.get_word(&dict, mode);
        //or it can be explicitly defined
        // let word = dict.get_word(dict.index_of("CRANE").unwrap());
        Self {
            mode,
            dict,
            finder,
            word_idx,
            gameover: false,
            rounds: 1,
        }
    }

    pub fn new_guess (&mut self, code_str: &String) {
        let word = self.dict.get_word(self.word_idx);
        if code_str.eq("OOOOO") { // 5 Greens
            self.gameover = true;
            return;
        }
        // Convert from a human readable string to a base 3 number
        // The b3 number is reversed
        // i.e. XX-OX becomes 20122
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

        // Remove everything that doesn't match that code with the same word
        self.finder.rmv_words(word, code, &self.dict);

        self.rounds += 1;

        self.word_idx = self.finder.get_word(&self.dict, self.mode);
    }

    pub fn solutions (&self) -> usize {
        self.finder.remaining_words.len()
    }

    pub fn current_word (&self) -> String {
        dictionary::string_from_char_arr(self.dict.get_word(self.word_idx))
    }
}
