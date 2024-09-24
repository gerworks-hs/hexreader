use std::io::Read;

fn main() {
	let argv: Vec<String> = std::env::args().collect::<Vec<String>>();
	let argc: usize = argv.len();

	println!("hexreader - v2.0.0");
	println!("Made by Gerworks-HS (itsgerliz)");
	println!("");

	if argc < 2 {
		println!("Usage: {} <file name> ...", argv[0]);
		std::process::exit(1);
	}

	for i in 1..=argc {
		let file_path: &std::path::Path = std::path::Path::new(&argv[i]);

		let mut file_data: std::fs::File = match std::fs::File::open(file_path) {
			Ok(file_struct) => {file_struct}
			Err(_) => {
				println!("Error trying to open file path ({})", argv[i]);
				continue;
			}
		};

		println!("Reading file <{}>", argv[i]);

		match read_hex(&mut file_data) {
			true => {
				println!("I/O Error, aborting...");
				std::process::exit(1);
			}
			false => {()}
		}
	}
	
}

fn read_hex(file: &mut std::fs::File) -> bool {
	let mut byte_buffer: [u8; 4] = [0; 4];
	let mut counter: u8 = 0;
	loop {
		match file.read(&mut byte_buffer) {
			Ok(bytes_read) => {
				if bytes_read == 0 {
					println!("");
					println!("--> EOF reached <--");
					break;
				} else {
					for i in 0..=(bytes_read - 1) {
						print!("{}", format!("{:02X} ", byte_buffer[i]));
						counter += 1;
						counter %= 8;
						if counter == 0 {
							println!("");
						}
						continue;
					}
				}
			}
			Err(_) => {
				println!("Error found when trying to read file");
				return true;
			}
		}
	}
	return false;
}