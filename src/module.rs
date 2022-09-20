#![warn(
	unused,
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
#![deny(clippy::unwrap_used)]
#![forbid(
	unsafe_code,
	clippy::exit,
	clippy::mem_forget,
	clippy::large_include_file,
	clippy::fn_to_numeric_cast_any,
	clippy::cast_precision_loss,
	clippy::float_arithmetic,
	clippy::excessive_precision,
	clippy::lossy_float_literal,
	clippy::float_cmp,
	clippy::float_cmp_const
)]

use num_bigint::BigInt;
use num_traits::Signed;

///remove all trailing zeros from a `BigInt`
fn trim(n: BigInt) -> BigInt {
	match n.trailing_zeros() {
		Some(z) => n >> z,
		None => n,
	}
}

///extreme shortcut Collatz function
///
///same as "standard shortcut" but it trims all trailing zeros
fn f(n: BigInt) -> BigInt {
	trim(if n.bit(0) { 3 * n + 1 } else { n })
}

///check a single number against the Collatz algorithm
pub fn check(mut n: BigInt) -> bool {
	n = trim(n);
	let m = n.clone();

	let lim_pos = BigInt::from((1_i128 << 68) | 1);
	let lim_neg = BigInt::from((-1_i64 << 33) | 1);

	if n.is_positive() {
		if n <= lim_pos {
			return false;
		}
		loop {
			n = f(n);
			if n <= lim_pos {
				break;
			}
		}
	} else {
		if n >= lim_neg {
			return false;
		}
		loop {
			n = f(n);
			if n >= lim_neg {
				break;
			}
		}
	}
	n == m
}
