// Functions that are just 'rules of the game'

pub fn is_valid_code_str(code: &str) -> bool {
    if code.chars().count() != 5 {
        return false;
    }
    for i in code.chars() {
        if i != 'O' && i != 'X' && i != '-' {
            return false;
        }
    }
    true
}

// Given two words, the answer and a guess, returns a code in the same way as wordle
pub fn gen_code(guess: &[char; 5], answer: &[char; 5]) -> u16 {
    // 0 = O
    // 1 = -
    // 2 = X

    let mut code: [u8; 5] = [0; 5];

    // Count how many times a letter has appeared
    let mut guess_char_count: [u8; 26] = [0; 26];
    let mut answer_char_count: [u8; 26] = [0; 26];

    for i in 0..5 {
        if guess[i] == answer[i] {
            code[i] = 0;
        }

        guess_char_count[guess[i] as usize - 65] += 1;
        answer_char_count[answer[i] as usize - 65] += 1;
    }

    for i in (0..5).rev() {
        let idx = guess[i] as usize - 65;
        if guess[i] != answer[i] {
            if answer_char_count[idx] < guess_char_count[idx] {
                guess_char_count[idx] -= 1;
                code[i] = 2;
            } else {
                code[i] = 1;
            }
        }
    }

    // Convert from a slice to a base 3 number
    let mut ret: u16 = 0;
    for (i, &c) in code.iter().enumerate() {
        let iter = c as u16 * 3_u16.pow(i as u32);
        ret += iter;
    }

    ret
}
