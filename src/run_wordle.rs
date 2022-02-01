use std::fs;
use std::io::stdin;
use rand::Rng;

mod game;
mod matcher;

pub fn run () -> Result<(), ()> {
    let tests: Vec<(&str, &str)> = vec![("XXXXX", "KNIFE"),
                                        ("XOXXX", "ROBOT"),
                                        ("XOX-X", "POACH"),
                                        ("OOOXX", "COUCH"),
                                        ("OOOOO", "COULD")];
    // let tests: Vec<(&str, &str)> = vec![("XOX-X", "POACH")];
    game::test(&tests);
    Ok(())
    // TODO figure this out and implement properly
    /*
    let words= file_to_vec("src/words.txt");

    let mut rng = rand::thread_rng();
    let r_num = rng.gen_range(0..words.len());
    let mut best_word: &str = &words[r_num];

    loop {
        println!("{}", best_word);
        let mut next_l = String::new();
        stdin().read_line(&mut next_l);
        if next_l.eq("exit\n") { return Ok(()); }
        print!("{}", next_l);
    }
     */
}

/// Returns a vec, split from file path; panics if file does not exist
fn file_to_vec (f: &str) -> Vec<String> {
    let file = fs::read_to_string(f);
    match file {
        Ok(f) => {
            return f.split("\r\n").
                map(|s| s.to_string()).
                collect();
        }
        Err(e) => { panic!("FnF"); }
    }
}