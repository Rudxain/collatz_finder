use num_bigint::BigInt;
use num_traits::{Zero, Signed};

//for best performance on any CPU, adapt to its WS
const WORD_SIZE: usize = std::mem::size_of::<usize>();

//remove trailing 0s FAST
fn trim(mut n: BigInt, mask: &BigInt) -> BigInt {
	if !(n.is_zero()) {
		//word iteration (batch processing)
		while (&n & mask).is_zero() {
			n >>= WORD_SIZE
		}
		//bit iter (slow and granular)
		while !(n.bit(0)) {
			n >>= 1
		}
	}
	n
}

fn f(n: BigInt) -> BigInt {
	(if n.bit(0) { 3 * n + 1 } else { n } ) >> 1
}

pub fn check(mut n: BigInt) -> bool {
	let lim_pos = BigInt::from((1i128 << 68) | 1);
	let lim_neg = BigInt::from(-(1i128 << 32) | 1);

	let mask = BigInt::from(!0usize);
	n = trim(n, &mask);
	let m = n.clone();

	if n.is_negative() {
		if n >= lim_neg {
			return false;
		}
		while {
			//do while loop
			n = trim(f(n), &mask);
			n < lim_neg
		} {}
	} else {
		if n <= lim_pos {
			return false;
		}
		while {
			n = trim(f(n), &mask);
			n > lim_pos
		} {}
	}
	n == m
}

pub fn parse(raw: &str) -> BigInt {
	let mut norm = raw.trim().to_lowercase();
	//I think I need `match`
	let radix = if norm.starts_with("0") {
		norm.remove(0);
		if norm.starts_with("u") {
			norm.remove(0);
			1
		} else if norm.starts_with("b") {
			norm.remove(0);
			2
		} else if norm.starts_with("t") {
			norm.remove(0);
			3
		} else if norm.starts_with("q") {
			norm.remove(0);
			4
		} else if norm.starts_with("p") {
			norm.remove(0);
			5
		} else if norm.starts_with("h") {
			norm.remove(0);
			6
		} else if norm.starts_with("s") {
			norm.remove(0);
			7
		} else if norm.starts_with("o") {
			norm.remove(0);
			8
		} else if norm.starts_with("n") {
			norm.remove(0);
			9
		} else if norm.starts_with("d") {
			norm.remove(0);
			10
		} else if norm.starts_with("x") {
			norm.remove(0);
			16
		} else {
			10
		}
	} else {
		10
	};

	if radix == 1 {
		BigInt::from(norm.len())
	} else {
		if norm == "" {
			BigInt::from(0)
		} else {
			BigInt::parse_bytes(norm.as_bytes(), radix).unwrap()
		}
	}
}
