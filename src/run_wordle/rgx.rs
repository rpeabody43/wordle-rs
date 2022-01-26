use regex::Regex;

// TODO TESTING SO MUCH TESTING
pub fn test () {
    assert_eq!(prev_good_letters(&Regex::new("^(?!.*[HOE])^(?=.*[RS].*[RS].*)^.*$").unwrap(), &'!'), "HOE");
}

pub fn get_regx (word: &str, code: &str, prev: Regex) -> Regex {
    let chars: Vec<char> = word.to_uppercase().chars().collect();
    let codes: Vec<char> = code.to_uppercase().chars().collect();

    let mut good = prev_letters(&prev, &"=");
    let mut bad = prev_letters(&prev, &"!");
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
                // good = format!("{}{}.*", good, chars[i]);
                exact = format!("{}{}{}]", exact, chars_not_at_idx(&prev)[i], chars[i]);
            }
        }
    }
    goodChars = good.len();
    good = format!(".*[{}]", good);
    good = good.repeat(goodChars);

    let ret = Regex::new(&format!("^(?!.*[{b}])^(?={g}.*)^{e}$", b=bad, g=good, e=exact));
}

fn chars_not_at_idx (rgx: &Regex) -> Vec<String> {
    // ^(?!.*[HOE])^(?=.*[RS].*[RS].*)^S[^n][^n][^n]R$
    let re = rgx.as_str();
    let start = re.find("*)^").unwrap() + 3;
    let end = re.find("$").unwrap();
    // S[^n][^n][^n]R
    let big_str = re[start..end].to_string();

    let mut ret: Vec<String> = Vec::new();
    while i < big_str.len() {
        if big_str[i..].find("[") == 0 {
            ret.push(big_str[i..i + 3].to_string());
            i += 4;
        }
        else {
            ret.push(big_str[i..i + 1].to_string());
            i += 1;
        }
    }

    ret
}

/// rgx is the previous regex, option is either '!' or '='
fn prev_letters (rgx: &Regex, option: &str) -> String {
    let re = rgx.as_str();
    let start = re.find(option).unwrap();
    let end = re[start + 4..].find("]").unwrap();
    re[start..end].to_string()
}