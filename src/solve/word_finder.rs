// The instance of the game. The 'player'.

use crate::solve::dictionary;
use crate::solve::wordle;

fn init_remaining(length: u32) -> Vec<usize> {
    let mut ret: Vec<usize> = Vec::new();
    for i in 0..length as usize {
        ret.push(i);
    }
    ret
}

fn avg(matches: &[u16; 243]) -> f32 {
    let mut sum: u16 = 0;
    for i in matches {
        sum += i;
    }
    sum as f32 / 243.0
}

fn standard_dev(matches: &[u16; 243]) -> f32 {
    let avg = avg(matches);

    let mut deviations: [f32; 243] = [0.0; 243];
    for i in 0..deviations.len() {
        let diff: f32 = matches[i] as f32 - avg;
        deviations[i] = diff * diff;
    }

    let mut std_dev: f32 = 0.0;
    for i in deviations {
        std_dev += i;
    }
    std_dev = ((std_dev as f64 / 243.0).sqrt()).ceil() as f32;
    std_dev
}

// The instance of each game
pub struct Finder {
    pub remaining_words : Vec<usize>
}

impl Finder {

    pub fn new (dict: u32) -> Self {
        Finder {
            remaining_words: init_remaining(dict)
        }
    }

    pub fn rmv_words (&mut self, prev_guess: &[char; 5], code: u16, dict: &dictionary::Dictionary) {
        self.remaining_words.retain
        (|w|
            wordle::gen_code(
                &prev_guess,
                dict.get_word(*w)) == code);
    }

    pub fn get_word (&self, dict: &dictionary::Dictionary, normal: bool) -> usize {
        let mut ret_word: usize = 0;
        let mut ret_dev: f32 = match normal {
                true => f32::MAX,
                false => 0.0
            };

        for possible_guess in &self.remaining_words {
            let mut codes_count: [u16; 243] = [0; 243];
            for word in &dict.words {
                let code = wordle::gen_code(word, &dict.words[*possible_guess]);
                let b10_code = wordle::to_b10(code);
                codes_count[b10_code as usize] += 1;
            }
            let std_dev = standard_dev(&codes_count);
            // println!("{}", std_dev);
            if normal {
                if std_dev < ret_dev {
                    ret_word = *possible_guess;
                    ret_dev = std_dev;
                }
            }
            else {
                if std_dev > ret_dev {
                    ret_word = *possible_guess;
                    ret_dev = std_dev;
                }
            }
        }
        ret_word
    }
}

