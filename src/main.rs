mod run_wordle;

pub fn main () {
    std::process::exit(match run_wordle::run() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {:?}", err);
            1
        }
    })
}
