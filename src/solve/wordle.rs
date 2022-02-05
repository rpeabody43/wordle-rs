use std::collections::HashMap;
use regex::Regex;

pub fn is_valid_code(code: &str) -> bool {
    if code.chars().count() != 5 { return false; }
    for i in code.chars() {
        if i != 'O' && i != 'X' && i != '-' { return false; }
    }
    true
}

fn arr_to_rgx (arr: &[String; 5]) -> Regex {
    let mut ret: String = String::new();
    for str in arr {
        ret = format!("{}{}", ret, str);
    }
    Regex::new(&format!("^{}$", ret)).unwrap()
}

fn match_hashmap (word: &String, letters: &HashMap<char, (usize, bool)>) -> bool {
    let mut counts: HashMap<char, usize> = HashMap::new();
    let chars: Vec<char> = word.chars().collect();
    let alphabet: Vec<char> = "ABCEDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    for c in &alphabet {
        counts.insert(*c, 0);
    }
    for c in &chars {
        counts.insert(*c, counts.get(c).unwrap() + 1);
    }
    // Allowed amounts are equal to if hard capped, or greater than if not
    for c in &alphabet {
        // If the amount of characters exceeds hard cap return false
        if counts.get(c).unwrap() > &letters.get(c).unwrap().0 && letters.get(c).unwrap().1 { return false; }
        // If the amount of characters is less than the required amounts
        if counts.get(c).unwrap() < &letters.get(c).unwrap().0 { return false; }
    }
    true
}

/// Returns true if the given word matches the regex condition formatted as an array, as well as a hashmap
pub fn match_wordle (word: &String, letters: &HashMap<char, (usize, bool)>, exact_arr: &[String; 5]) -> bool {
    let exact = arr_to_rgx(exact_arr);

    exact.is_match(word) && match_hashmap(word, letters)
}