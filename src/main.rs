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

fn main() {
	///program name
	const NAME: &str = "colfind";

	let args: Vec<String> = std::env::args().skip(1).collect();
	if args.is_empty() || args[0] == "help" {
		println!(
			"checks or searches for counterexamples to the Collatz conjecture\n\
			\n\
			usage: {} [SUBCOMMAND] [n]\n\
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
			0x (hexadecimal, 16)",
			NAME
		);
	}

	let subcmd = &args[0];
	assert!(
		!(subcmd != "check" && subcmd != "search"),
		"unrecognized subcommand"
	);

	let n = parse(&args[1]).expect("numeral argument cannot be parsed to integer");

	println!(
		"{}",
		if check(n) {
			"counter-example VERIFIED!"
		} else {
			"known cycle, regular number"
		}
	);
}
