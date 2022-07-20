use collatz_finder::*;

fn main() {
	let args: Vec<String> = std::env::args().collect();
	if args.len() < 2 || args.iter().any(|a| a == "--help") || args.iter().any(|a| a == "-h") {
		println!("\
			Usage: collatz_finder [n]\n\
			n must be an integer numeral.\n\
			Add a prefix to specify the base/radix (optional).\n\
			Supported prefixes and their associated radices:\n\
			0u (unary, 1)\n\
			0b (binary, 2)\n\
			0t (ternary, 3)\n\
			0q (quaternary, 4)\n\
			0p (pental/quinary, 5)\n\
			0h (heximal/senary, 6. NOT hexaDECimal)\n\
			0s (septimal/septenary, 7)\n\
			0o (octal, 8. the \"o\" is mandatory)\n\
			0n (nonary, 9)\n\
			0d (decimal, 10. standard de facto)\n\
			0x (hexadecimal, 16)\
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
