#[macro_use]
extern crate ini;

use std::process::Command;

fn main() {
    let map = ini!("./aromlauncher.ini");
    let execute = map["launch"]["exe"].clone().unwrap();

    Command::new("AlphaROMdiE.exe").arg(execute).spawn().expect("Failed to launch");
}
