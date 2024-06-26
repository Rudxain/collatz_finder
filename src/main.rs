#![warn(
	clippy::pedantic,
	clippy::nursery,
	clippy::unwrap_used,
	clippy::shadow_unrelated,
	clippy::string_to_string,
	clippy::format_push_string,
	clippy::decimal_literal_representation,
	clippy::unseparated_literal_suffix,
	clippy::empty_structs_with_brackets
)]
#![forbid(
	unsafe_code,
	clippy::mem_forget,
	clippy::cast_precision_loss,
	clippy::lossy_float_literal,
	clippy::float_arithmetic,
	clippy::float_cmp,
	clippy::float_cmp_const,
	clippy::large_include_file,
	clippy::fn_to_numeric_cast_any,
	clippy::excessive_precision
)]

use core::str::FromStr;
use num_bigint::BigInt;
use std::process::ExitCode;

mod util;
#[allow(clippy::wildcard_imports)]
use util::*;

enum SubCmd {
	Help,
	Check,
	Search,
}

impl FromStr for SubCmd {
	type Err = ();

	fn from_str(input: &str) -> Result<Self, Self::Err> {
		match input {
			"help" | "HELP" | "man" | "/?" | "❔" | "❓" | "ℹ️" | "ℹ" => Ok(Self::Help),
			"check" | "ck" => Ok(Self::Check),
			"search" | "s" => Ok(Self::Search),
			_ => Err(()),
		}
	}
}

#[allow(clippy::too_many_lines)]
fn main() -> ExitCode {
	use std::env::args;
	const NAME: &str = "colfind";

	if args().count() < 2 {
		eprintln!("No arguments provided. Run `{NAME} help` for more info");
		return ExitCode::SUCCESS;
	}

	let arg1: String = args().skip(1).take(1).collect();

	// currently known bounds in 2022
	let lim = Limit {
		pos: BigInt::from((1_i128 << 68) | 1),
		neg: BigInt::from((-1_i64 << 33) | 1),
	};

	if let Ok(subcmd) = SubCmd::from_str(arg1.as_str()) {
		match subcmd {
			SubCmd::Help => {
				println!(
					"\
						Searches counterexamples to the Collatz conjecture.\n\
						Usage: {NAME} <subcommand> [n]\n\
						subcmds: check (verify a single integer), search (check a range of values).\n\
						n is the number to check, or range size.\
					"
				);
				return ExitCode::SUCCESS;
			}
			SubCmd::Check => {
				match BigInt::from_str(args().skip(2).take(1).collect::<String>().as_str()) {
					Ok(n) => {
						println!(
							"{}",
							if check(&n, &lim) {
								"counter-example VERIFIED!"
							} else {
								"known cycle, regular number"
							}
						);
						return ExitCode::SUCCESS;
					}
					Err(e) => {
						eprintln!("{e}");
						return ExitCode::FAILURE;
					}
				}
			}
			SubCmd::Search => {
				match i128::from_str(args().skip(2).take(1).collect::<String>().as_str()) {
					Ok(n) => {
						match search(&Range::Int(n), lim) {
							Some(n) => println!("found counter-example!\n{n}"),
							None => println!("not found yet"),
						}
						return ExitCode::SUCCESS;
					}
					Err(e) => {
						eprintln!("{e}");
						return ExitCode::FAILURE;
					}
				}
			}
		}
	}

	eprintln!("Unrecognized sub-command:\n{arg1}\nRun `{NAME} help` to get list of valid ones");
	ExitCode::FAILURE
}
