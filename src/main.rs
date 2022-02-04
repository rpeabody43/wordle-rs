mod run;

pub fn main () {
    std::process::exit(match run::run() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {:?}", err);
            1
        }
    })
}
