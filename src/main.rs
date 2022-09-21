use std::io::stdin;
use wordle::is_valid_code;

mod solve;
pub(crate) mod wordle;


// 1 : Wordle (Least guesses possible)
// 2 : Survivle (Most guesses possible)
fn set_mode () -> bool {
    println!("1 for Wordle, 2 for Survivle: ");
    let mut mode_str = String::new();
    let valid = false;
    while !valid {
        match stdin().read_line(&mut mode_str) {
            Ok(_) => { }
            Err(_) => {
                println!("something went wrong");
                break;
            }
        }
        mode_str = mode_str.trim().to_string();
        if !(mode_str.eq("1") || mode_str.eq("2")) {
            println!("INVALID");
        }
        else { return mode_str.eq("1"); }
    }
    false
}

fn get_valid_code () -> String {
    let mut code_valid = false;
    let mut code_str: String = String::new();
    while !code_valid {
        code_str = String::new();
        match stdin().read_line(&mut code_str) {
            Ok(_) => {}
            Err(_) => {
                println!("something went wrong");
                break;
            }
        }

        // Cast to uppercase, then take everything but the \n at the end
        code_str = code_str.to_uppercase().trim().to_string();
        if is_valid_code(&code_str) {
            code_valid = true;
        } else {
            println!("invalid code: {}", code_str);
        }
    }
    code_str
}

pub fn main () {
    console_app();
}

pub fn console_app () {
    let mode = set_mode();
    println!("{}",
             match mode {
                 true => "WORDLE:",
                 false => "SURVIVLE:"
             });

    let dict_path = "resources/hiddenwords.txt";

    let mut session: solve::Session = solve::Session::new(mode, dict_path);
    let mut code_str: String;

    while !session.gameover {
        println!("{} ({} solutions)", session.current_word(), session.solutions());
        code_str = get_valid_code();
        println!("{}: {}\n", session.current_word(), code_str);
        session.new_guess(&code_str);
    }
}
