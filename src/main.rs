#![warn(
	unused,
	future_incompatible,
	clippy::unwrap_used,
	clippy::cargo,
	clippy::pedantic,
	clippy::nursery,
	clippy::shadow_unrelated,
	clippy::string_to_string,
	clippy::decimal_literal_representation,
	clippy::unseparated_literal_suffix,
	clippy::empty_structs_with_brackets,
	clippy::format_push_string,
	clippy::arithmetic_side_effects
)]
//can't `forbid` because of `module.rs`
//I have no idea why
#![deny(clippy::cast_precision_loss, clippy::float_cmp)]
#![forbid(
	unsafe_code,
	clippy::mem_forget,
	clippy::large_include_file,
	clippy::fn_to_numeric_cast_any,
	clippy::float_arithmetic,
	clippy::excessive_precision,
	clippy::lossy_float_literal,
	clippy::float_cmp_const
)]

mod module;
#[allow(clippy::wildcard_imports)]
use crate::module::*;
use std::process::exit;

///program name
const NAME: &str = "colfind";

fn main() {
	let args: Vec<String> = std::env::args().collect();
	if args.len() < 2 || args.iter().any(|a| a == "--help") || args.iter().any(|a| a == "-h") {
		println!(
			"\
			usage: {} [n]\n\
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
		",
			NAME
		);
		return;
	}
	let n = parse(&args[1]).expect("invalid numeral");

	if check(n) {
		println!("counter-example VERIFIED!");
		/*
		despite the absence of an error, this situation is so extraordinary
		that it should trigger something on automated scripts
		*/
		exit(1)
	} else {
		println!("known cycle, regular number");
		exit(0)
	}
}
