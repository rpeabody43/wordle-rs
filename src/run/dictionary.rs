use std::fs;
use rand::thread_rng;
use rand::Rng;

pub struct Dictionary {
    words: Vec<String>
}

/// Returns a vec, split from file path; panics if file does not exist
fn file_to_vec (f: &str) -> Vec<String> {
    let file = fs::read_to_string(f);
    match file {
        Ok(f) => {
            return f.split("\r\n").
                map(|s| s.to_string().to_uppercase()).
                collect();
        }
        Err(e) => { panic!("FnF"); }
    }
}

impl Dictionary {
    pub fn new (f: &str) -> Self {
        Dictionary {
            words: file_to_vec(f)
        }
    }

    pub fn get_word (self, idx: usize) -> String {
        self.words[idx].to_string()
    }

    pub fn get_random_word (self) -> String {
        let mut rng = thread_rng();
        let num: usize = rng.gen_range(0..self.words.len());
        self.words[num].clone()
    }

    pub fn rmv_word (&mut self, word: &str) {
        let idx = self.words.iter().position(|w| w.eq(word)).unwrap();
        self.words.remove(idx);
    }
}

pub fn test () {
    for i in Dictionary::new("src/words.txt").words {
        println!("{}", i);
    }
}
