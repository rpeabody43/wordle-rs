mod solve;

pub fn main () {
    // Entrypoint of the program, just runs the solver
    std::process::exit(match solve::run() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {:?}", err);
            1
        }
    })
}
