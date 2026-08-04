use std::fs;
use std::io::{BufRead, BufReader};

/*
 * The data
 */
pub struct Cartridge {
    pub data: Vec<u8>,
}

impl Cartridge {
    pub fn new(path: &str) -> Self {
        // 1. Read file at path into a Vec<u8>
        let file_path = path;
        //
        // 2. return a Cartridge with that data
    }
}
