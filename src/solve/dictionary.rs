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
        Err(..) => { panic!("FnF"); }
    }
}

impl Dictionary {
    pub fn new (f: &str) -> Self {
        Dictionary {
            words: file_to_vec(f)
        }
    }

    pub fn get_word (&self, idx: usize) -> &String {
        &self.words[idx]
    }

    pub fn get_random_word (&self) -> &String {
        let mut rng = thread_rng();
        let num: usize = rng.gen_range(0..self.words.len());
        &self.words[num]
    }

    pub fn rmv_word (&mut self, word: &str) {
        let idx = self.words.iter().position(|w| w.eq(word)).unwrap();
        self.words.remove(idx);
    }


    pub fn index_of (&self, str: &str) -> Option<usize> {
        let mut begin: usize = 0;
        let mut end: usize = self.words.len() - 1;
        let mut pointer: usize = self.words.len() / 2;

        while !self.words[pointer].eq(str) && begin <= end {
            pointer = (begin + end) / 2;

            // < in this context means lower in the alphabet
            if self.words[pointer] < str.to_string() {
                begin = pointer + 1;
            }
            else {
                end = pointer - 1;
            }
        }

        match self.words[pointer].eq(str) {
            true => {
                Some(pointer)
            }
            false => {
                None
            }
        }
    }
}
