pub(crate) mod dictionary;
mod word_finder;

// OOP makes this harder than it has to be
// but it's the only way I can think to separate frontend stuff and the solving algo
#[derive(Clone, PartialEq, Eq,)]
pub struct Session {
    mode: bool,
    dict: dictionary::Dictionary,
    finder: word_finder::Finder,
    pub word_idx: usize,
    pub gameover: bool,
    rounds: i32,
}

fn default_words (mode: bool) -> usize {
    match mode {
        true => 105,
        false => 830
    }
}

impl Session {
    pub fn new (mode: bool) -> Self {
        let dict = dictionary::Dictionary::new();
        let finder = word_finder::Finder::new(dict.words.len() as u32);
        // The best starting word can be found by the program, or explicitly defined
        // or it can be explicitly defined
        // BEST WORDS (according to this program)
        // words.txt:
        //      good: LARES, bad: FUFFY
        // hiddenwords.txt
        //      good: ARISE (105), bad: FUZZY (830)
        // let word_idx = finder.get_word(&dict, mode);
        // let word_idx = dict.index_of(match mode {
        //     true => "ARISE",
        //     false => "FUZZY",
        // }).unwrap();
        let word_idx = default_words(mode);
        Self {
            mode,
            dict,
            finder,
            word_idx,
            gameover: false,
            rounds: 1,
        }
    }

    pub fn from (mode: bool, prev_session: Self) -> Self {
        let dict = prev_session.dict;
        Self {
            mode,
            finder: prev_session.finder,
            word_idx: default_words(mode),
            dict,
            gameover: false,
            rounds: 1
        }
    }

    pub fn new_guess (&mut self, code: u16) {
        let word = self.dict.get_word(self.word_idx);

        if code == 0 { // 5 Greens
            self.gameover = true;
            return;
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
        self.dict.get_word(self.word_idx).iter().collect()
    }
}
