use std::io::Read;

fn main() {
    let argv: Vec<String> = std::env::args().collect::<Vec<String>>();
	let argc: usize = argv.len();

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

	read_hex(&mut file_data);
}

fn read_hex(file: &mut std::fs::File) -> () {
	// The function uses an array of 4 bytes (32 bits) as the data chunk size
	// If the file is 40 bits long then 2 chunks of data (iterations) must be processed
	// fill method is used to avoid repeating data if the next chunk does not fill up the buffer
	// The smaller the buffer is, the slower hexreader becomes
	// Since fill method is used, if the next chunk does not fill up buffer, zeroes will be printed
	// This can lead to confusion when reading a file that in fact contains zeroes
	// This could be easily fixed decreasing data chunk size, but performance would be decreased
	// A fix will come out for this
	let mut byte_buffer: [u8; 4] = [0; 4];
	loop {
		byte_buffer.fill(0);
		match file.read(&mut byte_buffer) {
			Ok(bytes_read) => {
				if bytes_read == 0 {
					println!(" --> EOF reached <-- ");
					break;
				} else {
					for i in byte_buffer {
						print!("{}", format!("{:02X} ", i));
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