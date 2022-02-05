use std::collections::HashMap;

pub struct Game {
    letters: HashMap<char, (usize, bool)>,
    known_positions: [String; 5],
}

fn construct_letters () -> HashMap<char, (usize, bool)> {
    let mut ret: HashMap<char, (usize, bool)> = HashMap::new();
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    for i in 0..alphabet.len() {
        ret.insert(alphabet.chars().nth(i).unwrap(), (0, false));
    }
    ret
}

impl Game {
    /// # DEFAULT CONSTRUCTOR:
    /// * `letters` -> a HashMap<char, (usize, bool)> with every
    /// letter of the alphabet corresponding to 0 to represent
    /// known amount of each letter, and whether we know if it's the max
    /// * `known_letters` -> an "empty" regex (^.....$)
    pub fn new () -> Self {
        Game {
            letters: construct_letters(),
            known_positions: ["[^g]".to_string(), "[^g]".to_string(), "[^g]".to_string(), "[^g]".to_string(), "[^g]".to_string()],
        }
    }

    /// # Adds a new guess to the turn history
    ///
    /// ## PARAMS
    ///
    /// * `code`: the wordle response in format "O-XOX"
    ///     * O : perfect
    ///     * \- : letter appears somewhere else
    ///     * X : letter not in word
    pub fn update (&mut self, code: &String, word: &String) {
        let codes: Vec<char> = code.chars().collect();
        let chars: Vec<char> = word.chars().collect();
        let mut rgx_update: [String; 5] = self.known_positions.clone();
        let mut hash_update = construct_letters();

        for i in 0..code.len() {
            // If the code is X, cap the max amount of times that letter can appear
            if codes[i] == 'X' {
                let amt = self.letters.get(&chars[i]).unwrap().0;
                self.letters.insert(chars[i], (amt, true));


            } else {
                // If the code is O, set the point in rgx to be that letter
                if codes[i] == 'O' {
                    rgx_update[i] = chars[i].to_string();
                } else if codes[i] == '-' {
                    // Select only the content part of the string
                    // i.e. if rgx_update[i] == [^gACF] add becomes gACF
                    // g is a placeholder character
                    let add: String = rgx_update[i][1..rgx_update[i].len() - 1].to_string();
                    rgx_update[i] = format!("[{}{}]", &add, chars[i]);
                }
                // increment
                let amt: usize = hash_update.get(&chars[i]).unwrap().0;
                hash_update.insert(chars[i], (amt + 1, false));

                // if update value is greater than instance variable
                // set instance variable to update val
                if hash_update.get(&chars[i]).unwrap().0 > self.letters.get(&chars[i]).unwrap().0 {
                    self.letters.insert(chars[i], *hash_update.get(&chars[i]).unwrap());
                }

            }
        }

        // Update rgx
        for i in 0..rgx_update.len() {
            self.known_positions[i] = rgx_update[i].to_string();
        }
    }

    //region GETTERS
    pub fn get_letter_map (&self) -> &HashMap<char, (usize, bool)> {
        &self.letters
    }

    pub fn get_exact_positions (&self) -> &[String; 5] {
        &self.known_positions
    }
    //endregion
}