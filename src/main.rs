use std::io::Read;

fn main() {
    let argv: Vec<String> = std::env::args().collect::<Vec<String>>();
	let argc: usize = argv.len();

	println!("hexreader - v1.6.0");
	println!("Made by Gerworks-HS (itsgerliz)");
	println!("");

	if argc != 2 {
		println!("Usage: {} <file name>", argv[0]);
		std::process::exit(1);
	}

	let file_path: &std::path::Path = std::path::Path::new(&argv[1]);

	let mut file_data: std::fs::File = match std::fs::File::open(file_path) {
		Ok(file_struct) => {file_struct}
		Err(_) => {
			println!("Error trying to access file path ({})", argv[1]);
			std::process::exit(1);
		}
	};

	println!("Reading file <{}>", argv[1]);

	read_hex(&mut file_data);
}

fn read_hex(file: &mut std::fs::File) -> () {
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
						if counter >= 8 {
							println!("");
							print!("{}", format!("{:02X} ", byte_buffer[i]));
							counter = 0;
							counter += 1;
							continue;
						} else {
							print!("{}", format!("{:02X} ", byte_buffer[i]));
							counter += 1;
							continue;
						}
					}
				}
			}
			Err(_) => {
				println!("Error found when trying to read file");
				std::process::exit(1);
			}
		}
	}
}