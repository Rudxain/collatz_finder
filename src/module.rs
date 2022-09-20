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

pub struct Limit {
	pos: BigInt,
	neg: BigInt,
}

///check a single number against the Collatz algorithm
pub fn check(mut n: BigInt, lim: Limit) -> bool {
	n = trim(n);
	let m = n.clone();

	if n.is_positive() {
		if n <= lim.pos {
			return false;
		}
		loop {
			n = f(n);
			if n <= lim.pos {
				break;
			}
		}
	} else {
		if n >= lim.neg {
			return false;
		}
		loop {
			n = f(n);
			if n >= lim.neg {
				break;
			}
		}
	}
	n == m
}

pub fn search(len: i128, mut lim: Limit) -> Option<BigInt> {
	let mut n;
	if len < 0 {
		for _ in len..0 {
			n = f(lim.neg);
			while n < lim.neg {
				n = f(n)
			}
			if n == lim.neg {
				return Some(lim.neg);
			};
			lim.neg -= 2;
		}
	} else {
		for _ in 0..len {
			n = f(lim.pos);
			while n > lim.pos {
				n = f(n)
			}
			if n == lim.pos {
				return Some(lim.pos);
			};
			lim.pos += 2;
		}
	};
	None
}
