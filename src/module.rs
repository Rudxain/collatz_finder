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

///shortcut Collatz function
fn f(n: BigInt) -> BigInt {
	(if n.bit(0) { 3 * n + 1 } else { n }) >> 1
}

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
			n = trim(f(n));
			if n <= lim_pos {
				break;
			}
		}
	} else {
		if n >= lim_neg {
			return false;
		}
		loop {
			n = trim(f(n));
			if n >= lim_neg {
				break;
			}
		}
	}
	n == m
}

pub fn parse(raw: &str) -> Option<BigInt> {
	let mut norm = raw.trim().to_lowercase();
	//this needs a `match`
	let radix = if norm.starts_with('0') {
		norm.remove(0);
		if norm.starts_with('u') {
			norm.remove(0);
			1
		} else if norm.starts_with('b') {
			norm.remove(0);
			2
		} else if norm.starts_with('t') {
			norm.remove(0);
			3
		} else if norm.starts_with('q') {
			norm.remove(0);
			4
		} else if norm.starts_with('p') {
			norm.remove(0);
			5
		} else if norm.starts_with('h') {
			norm.remove(0);
			6
		} else if norm.starts_with('s') {
			norm.remove(0);
			7
		} else if norm.starts_with('o') {
			norm.remove(0);
			8
		} else if norm.starts_with('n') {
			norm.remove(0);
			9
		} else if norm.starts_with('d') {
			norm.remove(0);
			10
		} else if norm.starts_with('x') {
			norm.remove(0);
			0x10
		} else {
			10
		}
	} else {
		10
	};

	let norm = norm;

	if radix == 1 {
		Some(BigInt::from(norm.len()))
	} else if norm.is_empty() {
		Some(BigInt::from(0))
	} else {
		BigInt::parse_bytes(norm.as_bytes(), radix)
	}
}
