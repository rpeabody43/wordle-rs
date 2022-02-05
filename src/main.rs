mod solve;

pub fn main () {
    std::process::exit(match solve::run() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {:?}", err);
            1
        }
    })
}
