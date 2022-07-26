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
	clippy::format_push_string
)]
//can't `forbid` because of `module.rs` and `clap`
#![deny(
	clippy::cast_precision_loss,
	clippy::float_cmp,
	clippy::mem_forget,
	clippy::large_include_file,
	clippy::fn_to_numeric_cast_any,
	clippy::float_arithmetic,
	clippy::excessive_precision,
	clippy::lossy_float_literal,
	clippy::float_cmp_const
)]
#![forbid(unsafe_code)]

mod module;
#[allow(clippy::wildcard_imports)]
use crate::module::*;

use num_bigint::BigInt;
use num_traits::ToPrimitive;

use clap::Parser;

#[derive(clap::Subcommand)]
enum Action {
	///verify a single integer
	Check,
	///check a range of values
	Search,
}
#[derive(Parser)]
#[clap(version, about = "searches counterexamples to the Collatz conjecture")]
//WHY IS IT BACKWARDS?
struct Cli {
	#[clap(subcommand)]
	action: Action,
	///number to check, or range size
	#[clap(value_parser)]
	n: BigInt,
}

fn main() {
	let cli = Cli::parse();

	let lim = Limit {
		pos: BigInt::from((1_i128 << 68) | 1),
		neg: BigInt::from((-1_i64 << 33) | 1),
	};

	match &cli.action {
		Action::Check => {
			println!(
				"{}",
				if check(cli.n, lim) {
					"counter-example VERIFIED!"
				} else {
					"known cycle, regular number"
				}
			);
		}
		Action::Search => match search(cli.n.to_i128().expect("expected decimal numeral"), lim) {
			Some(n) => println!("found counter-example!\n{}", n),
			None => println!("not found yet"),
		},
	}
}
