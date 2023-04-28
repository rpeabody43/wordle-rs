// Standard functions for the dictionary of words
#[derive(Clone, PartialEq, Eq,)]
pub struct Dictionary {
    pub words: Vec<[char; 5]>,
}

// The < operator works with strings but for a char array I have to do this
fn is_lower_alphabet (word1: &[char; 5], word2: &[char; 5]) -> bool {
    for i in 0..5 as usize {
        match word1[i] == word2[i] {
            true => { continue; }
            false => { return word1[i] < word2[i]; }
        }
    }
    false
}

// Char arrays are more efficient to index into
fn string_to_chars (word: &String) -> [char; 5]{
    let mut ret: [char; 5] = [' '; 5];
    let chars: Vec<char> = word.chars().collect();
    for i in 0..chars.len() as usize { ret[i] = chars[i];
    }
    ret
}

// Returns a vector, split from file path; panics if file does not exist
fn file_to_vec () -> Vec<[char; 5]> {
    let file = include_str!("hiddenwords.txt");
    file.split("\r\n").
    map(|s| string_to_chars(&s.to_uppercase())).
    collect()
}

impl Dictionary {
    pub fn new () -> Self {
        Dictionary {
            words: file_to_vec(),
        }
    }

    pub fn get_word (&self, idx: usize) -> &[char; 5] {
        &self.words[idx]
    }

    pub fn index_of (&self, str: &str) -> Option<usize> {
        let target: [char; 5] = string_to_chars(&String::from(str));

        let mut begin: usize = 0;
        let mut end: usize = self.words.len() + 1;
        let mut pointer: usize = self.words.len() / 2;

        while !self.words[pointer].eq(&target) && begin <= end {
            // println!("{}", pointer);
            pointer = begin + (end - begin) / 2;
            if is_lower_alphabet(&target, &self.words[pointer]) {
                end = pointer - 1;
            } else {
                begin = pointer + 1;
            }
        }

        match self.words[pointer].eq(&target) {
            true => {
                Some(pointer)
            }
            false => {
                None
            }
        }
    }
}
