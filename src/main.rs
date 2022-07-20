use collatz_finder::*;

fn main() {
	let args: Vec<String> = std::env::args().collect();
	if args.len() < 2 || args.iter().any(|a| a == "--help") || args.iter().any(|a| a == "-h") {
		println!("\
			usage: collatz [n]\n\
			n must be an integer decimal (B10) numeral\n\
			use 0x prefix to input hexadecimal (B16), or 0t for ternary (B3)\
		");
		return;
	}
	let n = parse(&args[1]);
	println!(
		"{}",
		if check(n) {
			"counter-example VERIFIED!"
		} else {
			"known cycle, regular number"
		}
	);
}
