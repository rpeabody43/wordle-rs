// Functions that are just 'rules of the game'

pub fn is_valid_code(code: &str) -> bool {
    if code.chars().count() != 5 { return false; }
    for i in code.chars() {
        if i != 'O' && i != 'X' && i != '-' { return false; }
    }
    true
}

pub fn to_b10 (num: u16) -> u16 {
    let mut i = num;
    let mut ret: u16 = 0;
    let mut pass: u8 = 0;
    while i > 0 as u16 {
        ret += (i % 10) * 3_u16.pow(pass as u32);
        i /= 10;
        pass += 1;
    }
    ret

}

// Given two words, the answer and a guess, returns a code in the same way as wordle
pub fn gen_code (guess: &[char; 5], answer: &[char; 5]) -> u16 {
    // 0 = O
    // 1 = -
    // 2 = X

    let mut code: [u8; 5] = [0; 5]; // Will be converted to a String at the end

    // Count how many times a letter has appeared
    let mut guess_char_count: [u8; 26] = [0; 26];
    let mut answer_char_count: [u8; 26] = [0; 26];

    for i in 0..5 as usize {
        if guess[i] == answer[i] {
            code[i] = 0;
        }

        guess_char_count[guess[i] as usize - 65] += 1;
        answer_char_count[answer[i] as usize - 65] += 1;
    }

    for i in (0..5 as usize).rev() {
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
    for i in 0..5 {
        let c = code[i];
        let iter = (c as u16 * 10_u16.pow(i as u32)) as u16;
        ret += iter;
    }

    ret
}
