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

// how do we `impl` this for all `BigInt`s and `PrimInt`s?
///remove all trailing-zero bits.
#[allow(clippy::needless_pass_by_value)]
fn trim(n: BigInt) -> BigInt {
	&n >> n.trailing_zeros().unwrap_or(0)
}

///extreme shortcut Collatz function.
///
///same as "standard shortcut" but it `trim`s all trailing zeros.
fn f(n: BigInt) -> BigInt {
	trim(if n.bit(0) { 3 * n + 1 } else { n })
}

pub struct Limit {
	pub(crate) pos: BigInt,
	pub(crate) neg: BigInt,
}

#[allow(clippy::needless_pass_by_value)]
///check a single number against the Collatz algorithm.
///
///returns `false` if it "converges", `true` if it disproves CC
pub fn check(mut n: BigInt, lim: Limit) -> bool {
	n = trim(n);

	if n.is_positive() {
		if n <= lim.pos {
			return false;
		}
		let m = n.clone();
		loop {
			n = f(n);
			if n <= lim.pos {
				return n == m;
			}
		}
	} else {
		if n >= lim.neg {
			return false;
		}
		let m = n.clone();
		loop {
			n = f(n);
			if n >= lim.neg {
				return n == m;
			}
		}
	}
}

///check a range of values `len`.
///
///returns `None` if all ints "converge", `Some` if at least 1 int disproves CC
pub fn search(len: i128, mut lim: Limit) -> Option<BigInt> {
	let mut n;
	if len < 0 {
		for _ in len..0 {
			n = f(lim.neg.clone());
			while n < lim.neg {
				n = f(n);
			}
			if n == lim.neg {
				return Some(lim.neg);
			};
			lim.neg -= 2;
		}
	} else {
		for _ in 0..len {
			n = f(lim.pos.clone());
			while n > lim.pos {
				n = f(n);
			}
			if n == lim.pos {
				return Some(lim.pos);
			};
			lim.pos += 2;
		}
	};
	None
}
