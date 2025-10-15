#![allow(dead_code)]
#![allow(unused_imports)]

mod console;
mod game;
mod webapp;

use console::console_app;
use webapp::web_app;

fn main() {
    // console_app();
    web_app();
}
