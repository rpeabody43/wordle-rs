use std::collections::HashMap;
use regex::Regex;

fn arr_to_rgx (arr: [&str; 5]) -> Regex {
    let mut ret: String = String::new();
    for str in arr {
        ret = format!("{}{}", ret, str);
    }
    Regex::new(&format!("^{}$", ret)).unwrap()
}

pub fn is_valid_code(code: &str) -> bool {
    if code.chars().length != 5 { return false; }
    for i in code.chars() {
        if i == 'O' || i == 'X' || i == '-' { return true; }
    }
    false
}

fn match_hashmap (word: &str, letters: &HashMap<char, (usize, bool)>) -> bool {
    let mut counts: HashMap<char, usize> = HashMap::new();
    let chars: Vec<char> = word.chars().collect();
    for c in &chars {
        counts.insert(*c, counts.get(c).unwrap() + 1);
    }
    for c in &chars {
        if counts.get(c).unwrap() > &letters.get(c).unwrap().0 && letters.get(c).unwrap().1 {
            return false;
        }
    }
    true
}

/// Returns if the given word matches the regex condition formatted as an array, as well as a hashmap
pub fn match_wordle (word: &str, letters: &HashMap<char, (usize, bool)>, exact_arr: [&str; 5]) -> bool {
    // TODO figure out a good structure for this because idk
    let exact = arr_to_rgx(exact_arr);

    exact.is_match(word) && match_hashmap(word, letters)
}