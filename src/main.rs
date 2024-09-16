fn main() {
    let argv: Vec<String> = std::env::args().collect::<Vec<String>>();
	let argc: usize = argv.len();

	if argc != 2 {
		println!("Usage: {} <file name>", argv[0]);
		std::process::exit(1);
	}
}