use fancy_regex::Regex;

// TODO TESTING SO MUCH TESTING
pub fn test () {
    println!("{}", prev_letters(&Regex::new("^(?!.*[bHOE])^(?=.*[gRS].*[gRS].*)^.*$").unwrap(), 'g'));
}

pub fn get_regx (word: &str, code: &str, prev: Regex) -> Regex {
    let chars: Vec<char> = word.to_uppercase().chars().collect();
    let codes: Vec<char> = code.to_uppercase().chars().collect();

    let mut good = prev_letters(&prev, 'g');
    let mut bad = prev_letters(&prev, 'b');
    let mut exact: String = String::new();

    // TODO does this algorithm work
    for i in 0..5 {
        if codes[i] == 'X' {
            bad = format!("{}{}", bad, chars[i]);
        } else {
            if codes[i] == 'O' {
                exact = format!("{}{}", exact, chars[i]);
            } else if codes[i] == '-' {
                // TODO turn this into .*[RS].*[RS] kinda thing
                let good_tmp = format!("{}{}",
                                       prev_letters(&prev, 'g'), chars[i]);
                good = format!("[g{}].*", good_tmp);
                good = good.repeat(good_tmp.len());
                exact = format!("{}{}{}]", exact, chars_not_at_idx(&prev)[i], chars[i]);
            }
        }
    }
    let good_chars = good.len();
    good = format!(".*[{}]", good);
    good = good.repeat(good_chars);

    let ret = Regex::new(&format!("^(?!.*[{b}])^(?=.*{g}.*)^{e}$", b=bad, g=good, e=exact));
    ret.unwrap()
}

fn chars_not_at_idx(rgx: &Regex) -> Vec<String> {
    // ^(?!.*[HOE])^(?=.*[RS].*[RS].*)^S[^n][^n][^n]R$
    let re = rgx.as_str();
    let start = re.find("*)^").unwrap() + 3;
    let end = re.find("$").unwrap();
    // S[^n][^n][^n]R
    let big_str = re[start..end].to_string();

    let mut ret: Vec<String> = Vec::new();
    let mut i = 0;
    while i < big_str.len() {
        if big_str[i..].find("[") == Some(0) {
            ret.push(big_str[i..i + 3].to_string());
            i += 4;
        } else {
            ret.push(big_str[i..i + 1].to_string());
            i += 1;
        }
    }

    ret
}

fn index_of(str: &str, substring: &str) -> isize {
    for i in 0..str.len() - substring.len() {
        if str[i..i + substring.len()].eq(substring) {
            return i as isize;
        }
    }
    -1
}

fn prev_letters(rgx: &Regex, good_or_bad: char) -> String {
    let re = rgx.as_str();
    let temp_start = index_of(&re, &format!("[{}", good_or_bad));
    let start = index_of(&re, &format!("[{}", good_or_bad)) as usize;
    let end = (index_of(&re[start + 1..], "]") + start as isize + 1) as usize;
    re[start + 2..end].to_string()
}
