use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


struct RegisterFile {

	// contains the 16 8-bit registers, V0-VF
	let data_regs = [mut [mut u8, ..7], ..15];

	// address register
	let address_reg = [mut u8 ..15];
}


fn decode() {
}













fn main() {
    let args: Vec<String> = env::args().collect();

    let path = Path::new(&args[0]);
    

    let display = path.display();


    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {} : {}", display, why),
        Ok(file) => file,
    };

	let mut buffer = Vec::new();
	file.read_to_end(&mut buffer);

	for i in 0..63 {
		println!("{:#04x} : {:#04x}", i, buffer[i]);
	}
}
