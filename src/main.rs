#![allow(dead_code)]

mod game;
mod console;
mod webapp;

use console::console_app;
use webapp::web_app;

fn main () {
    // console_app();
    web_app();
}
