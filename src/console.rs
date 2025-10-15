use std::io::stdin;
use std::time::Instant;

use crate::game::{Mode, Session, is_valid_code_str};

pub fn console_app() {
    let mode = set_mode().unwrap();
    println!(
        "{}",
        match mode {
            Mode::Normal => "WORDLE:",
            Mode::HardMode => "WORDLE (Hard Mode):",
            Mode::WorstGuess => "SURVIVLE:",
        }
    );

    let mut session: Session = Session::new(mode);
    let mut code_str: String;

    while !session.gameover() {
        println!(
            "{} ({} solutions)",
            session.current_word(),
            session.solutions(),
        );
        code_str = get_valid_code();
        println!("{}: {}\n", session.current_word(), code_str);
        let code: u16 = str_to_code(&code_str);

        let now = Instant::now();
        session.new_guess(code);
        let guess_time = now.elapsed();
        println!("{:2?} elapsed", guess_time);
        println!();
    }
}

// 1 : Wordle (Least guesses possible)
// 2 : Survivle (Most guesses possible)
fn set_mode() -> Result<Mode, String> {
    println!("1 for Normal Wordle, 2 for Hard Mode, 3 for Survivle: ");
    let mut buffer = String::new();
    loop {
        // Loops forever
        match stdin().read_line(&mut buffer) {
            Ok(_) => {}
            Err(_) => {
                println!("something went wrong");
                return Err(String::from("something went wrong"));
            }
        }
        let mode_str = buffer.trim();
        match mode_str {
            "1" => return Ok(Mode::Normal),
            "2" => return Ok(Mode::HardMode),
            "3" => return Ok(Mode::WorstGuess),
            _ => println!("INVALID"),
        }
    }
}

fn get_valid_code() -> String {
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
        if is_valid_code_str(&code_str) {
            code_valid = true;
        } else {
            println!("invalid code: {}", code_str);
        }
    }
    code_str
}

fn str_to_code(code_str: &String) -> u16 {
    // Convert from a human readable string to a base 3 number
    // The b3 number is reversed
    // i.e. XX-OX becomes 20122
    let mut code: u16 = 0;
    for i in 0..code_str.len() {
        let c = code_str.chars().nth(i).unwrap();
        code += match c {
            'O' => 0,
            '-' => 1,
            'X' => 2,
            _ => {
                // Already checked for validity above, so this will never run
                // Borrow checker throws a compile error if there's no _ pattern
                panic!("this can't happen lol");
            }
        } * 3_u32.pow(i as u32) as u16;
    }
    code
}
