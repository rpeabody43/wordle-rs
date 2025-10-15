use std::collections::HashSet;

use crate::game::wordle;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Mode {
    Normal,
    HardMode,
    WorstGuess,
}

// OOP makes this harder than it has to be
// but it's the only way I can think to separate frontend stuff and the solving algo
#[derive(Clone, Eq, PartialEq)]
pub struct Session {
    mode: Mode,
    remaining_words: HashSet<usize>,
    all_words: Vec<[char; 5]>,
    current_word: usize,
    gameover: bool,
    rounds: i32,
}

fn default_word_idx(mode: Mode) -> usize {
    match mode {
        Mode::Normal | Mode::HardMode => 9490, // SALET
        Mode::WorstGuess => 8728,              // QAJAQ
    }
}

// Char arrays are more efficient to index into
fn string_to_chars(word: &String) -> [char; 5] {
    let mut ret: [char; 5] = [' '; 5];
    let chars: Vec<char> = word.chars().collect();
    ret[..chars.len()].copy_from_slice(&chars[..]);
    ret
}

// Returns a vector, split from file path; panics if file does not exist
fn file_to_vec(file_str: &'static str) -> Vec<[char; 5]> {
    file_str
        .split("\r\n")
        .map(|s| string_to_chars(&s.to_uppercase()))
        .collect()
}

fn init_remaining(all_words: &Vec<[char; 5]>, hidden_words_file: &'static str) -> HashSet<usize> {
    let answer_words = file_to_vec(hidden_words_file);
    let mut ret = HashSet::with_capacity(answer_words.len());

    let mut answer_idx = 0;
    for i in 0..all_words.len() {
        if all_words[i] == answer_words[answer_idx] {
            ret.insert(i);
            answer_idx += 1;
            if answer_idx >= answer_words.len() {
                break;
            }
        }
    }
    ret
}

impl Session {
    pub fn new(mode: Mode) -> Self {
        // The best starting word can be found by the program
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
        let all_words_file = include_str!("words/words.txt");
        let all_words = file_to_vec(all_words_file);

        let answer_words_file = include_str!("words/hiddenwords.txt");

        let ret = Self {
            mode,
            remaining_words: init_remaining(&all_words, answer_words_file),
            all_words,
            current_word: default_word_idx(mode),
            gameover: false,
            rounds: 1,
        };
        /*
        // Find first best word manually
        ret.current_word =
            ret.find_best_word(0..ret.all_words.len());
        */
        ret
    }

    pub fn rmv_words(&mut self, code: u16) {
        self.remaining_words.retain(|w| {
            wordle::gen_code(&self.all_words[self.current_word], &self.all_words[*w]) == code
        });
    }

    pub fn new_guess(&mut self, code: u16) {
        if code == 0 {
            // 5 Greens
            self.gameover = true;
            return;
        }

        self.rounds += 1;

        // Remove everything that doesn't match that code with the same word
        self.rmv_words(code);

        if self.remaining_words.is_empty() {
            self.gameover = true;
            return;
        }

        self.current_word = match self.mode {
            Mode::Normal => self.find_best_word(0..self.all_words.len()),
            Mode::HardMode | Mode::WorstGuess => {
                self.find_best_word(self.remaining_words.iter().copied())
            }
        }
    }

    pub fn find_best_word<I>(&self, outer_collection: I) -> usize
    where
        I: IntoIterator<Item = usize>,
    {
        let mut ret_word_idx: usize = 0;
        let mut ret_entropy: f64 = match self.mode {
            Mode::Normal | Mode::HardMode => 0.0,
            Mode::WorstGuess => f64::MAX,
        };

        // For all words in the dictionary, which one minimizes standard dev
        // of possible follow up guesses (and therefore most likely to narrow
        // the list of remaining words the most)

        // survivle / hard mode can only use legal words
        for i in outer_collection {
            let guess_word = &self.all_words[i];
            let mut code_counts: [u16; 243] = [0; 243];

            for idx in &self.remaining_words {
                let possible_word = &self.all_words[*idx];
                let code = wordle::gen_code(possible_word, guess_word);
                code_counts[code as usize] += 1;
            }

            let entropy = entropy(&code_counts, self.remaining_words.len());
            if self.mode != Mode::WorstGuess {
                if entropy > ret_entropy
                    || entropy == ret_entropy && self.remaining_words.contains(&i)
                {
                    ret_word_idx = i;
                    ret_entropy = entropy;
                    println!(
                        "{}: {ret_entropy}",
                        self.all_words[ret_word_idx].iter().collect::<String>()
                    );
                }
            } else if entropy < ret_entropy {
                ret_word_idx = i;
                ret_entropy = entropy;
                println!(
                    "{}: {ret_entropy}",
                    self.all_words[ret_word_idx].iter().collect::<String>()
                );
            }
        }
        ret_word_idx
    }

    pub fn solutions(&self) -> usize {
        self.remaining_words.len()
    }

    pub fn current_word(&self) -> String {
        self.all_words[self.current_word].iter().collect()
    }

    pub fn gameover(&self) -> bool {
        self.gameover
    }

    pub fn mode(&self) -> Mode {
        self.mode
    }
}

fn entropy(matches: &[u16; 243], remaining: usize) -> f64 {
    let mut entropy = 0.0;
    for i in 0..243 {
        let p_i = matches[i] as f64 / remaining as f64;
        if p_i == 0.0 {
            continue;
        }
        entropy += p_i * p_i.log2();
    }
    -entropy
}
