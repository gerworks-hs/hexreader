fn main() {
    let argv: Vec<String> = std::env::args().collect::<Vec<String>>();
	let argc: usize = argv.len();

	if argc != 2 {
		println!("Usage: {} <file name>", argv[0]);
		std::process::exit(1);
	}

	let file_path: &std::path::Path = std::path::Path::new(&argv[1]);

	let file_data: std::fs::File = match std::fs::File::open(file_path) {
		Ok(file_struct) => {file_struct}
		Err(_) => {
			println!("Error trying to access file path ({})", argv[1]);
			std::process::exit(1);
		}
	};
}