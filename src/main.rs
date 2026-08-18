// Let the module be known
mod bus;
mod cartridge;
mod cpu;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: metal-boy <rom file>");
        std::process::exit(1);
    }

    let rom_path = &args[1];
    println!("Loading ROM: {}", rom_path);
    println!("Extra random space");
}
