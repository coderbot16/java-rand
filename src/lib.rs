//! Implementation of the Java Random Number generator.

use std::num::Wrapping;

/// Modulus
pub const M: Wrapping<i64> = Wrapping((1 << 48) - 1);

/// Multiplier
pub const A: Wrapping<i64> = Wrapping(0x5DEECE66D);

/// Increment
pub const C: Wrapping<i64> = Wrapping(11);

const F32_DIV: f32 = (1u32 << 24) as f32;
const F64_DIV: f64 = (1u64 << 53) as f64;

#[derive(Debug, Clone)]
pub struct Random {
	state: Wrapping<i64>,
	next_gaussian: Option<f64>
}

impl Random {
	pub fn new(seed: u64) -> Self {
		Random {
			state: Wrapping((seed as i64) ^ A.0) & M,
			next_gaussian: None
		}
	}

	/// Sets the seed to `seed`. This is equivalent to `Random::new`
	pub fn set_seed(&mut self, seed: u64) {
		*self = Random::new(seed);
	}

	/// Steps the RNG, returning up to 48 bits.
	///
	/// # Panics
	/// If the amount of requested bits is over 48, this function panics. Use next_i64/next_u64 instead, or multiple calls.
	pub fn next(&mut self, bits: u8) -> u64 {
		if bits > 48 {
			panic!("Too many bits!")
		}

		self.state = (self.state * A + C) & M;

		(self.state.0 as u64) >> (48 - bits)
	}

	/// Fills the byte array with random bytes.
	pub fn next_bytes(&mut self, bytes: &mut [u8]) {
		for chunk in bytes.chunks_mut(4) {
			let mut block = self.next_u32();

			for item in chunk {
				*item = (block & 0xFF) as u8;
				block >>= 8;
			}
		}
	}

	/// Returns a uniformly distributed signed 32-bit integer.
	pub fn next_i32(&mut self) -> i32 {
		self.next(32) as i32
	}

	/// Returns a uniformly distributed unsigned 32-bit integer.
	pub fn next_u32(&mut self) -> u32 {
		self.next(32) as u32
	}

	/// Returns a positive random number in the range [0, max), up to 2^31.
	/// The range of the return value is represented by the value `0 <= value < max`.
	/// A maximum of less than 1 is invalid because then no value would satisfy the range.
	///
	/// # Panics
	/// If `max` is less than 1, the function panics.
	pub fn next_i32_bound(&mut self, max: i32) -> i32 {
		if max <= 0 {
			panic!("Maximum must be > 0")
		}

		if (max as u32).is_power_of_two() {
			let max = max as u64;

			return ((max.wrapping_mul(self.next(31))) >> 31) as i32;
		}

		let mut bits = self.next(31) as i32;
		let mut val = bits % max;

		while bits - val + (max - 1) < 0 {
			bits = self.next(31) as i32;
			val = bits % max;
		}

		val
	}

	/// Returns a positive random number in the range [0, max), up to 2^31.
	/// The range of the return value is represented by the value `0 <= value < max`.
	/// A maximum of 0 is invalid because then no value would satisfy the range.
	/// Maximums of 2^31 or greater are not supported in Java.
	///
	/// # Panics
	/// If `max` reinterpreted as a signed 32-bit integer is less than 1, the function panics.
	pub fn next_u32_bound(&mut self, max: u32) -> u32 {
		self.next_i32_bound(max as i32) as u32
	}

	/// Returns a uniformly distributed signed 64-bit integer.
	pub fn next_i64(&mut self) -> i64 {
		self.next_u64() as i64
	}

	/// Returns a uniformly distributed unsigned 64-bit integer.
	pub fn next_u64(&mut self) -> u64 {
		(self.next(32) << 32).wrapping_add(self.next(32))
	}

	/// Returns a boolean value that has an equal chance of being true or false.
	pub fn next_bool(&mut self) -> bool {
		self.next(1) == 1
	}

	/// Returns a f32 uniformly distributed between 0.0 and 1.0.
	pub fn next_f32(&mut self) -> f32 {
		(self.next(24) as f32) / F32_DIV
	}

	/// Returns a f64 uniformly distributed between 0.0 and 1.0.
	pub fn next_f64(&mut self) -> f64 {
		let high = (self.next(26) as i64) << 27;
		let low = self.next(27) as i64;

		(high.wrapping_add(low) as f64) / F64_DIV
	}

	/// Returns a pair of gaussian random numbers generated by the Box-Mueller transform.
	fn next_gaussian_pair(&mut self) -> (f64, f64) {
		let mut next_candidate = || {
			let v = (
				2.0 * self.next_f64() - 1.0,
				2.0 * self.next_f64() - 1.0
			);

			(v, v.0*v.0 + v.1*v.1)
		};

		let (mut v, mut s) = next_candidate();

		while s >= 1.0 || s == 0.0 {
			let (vn, sn) = next_candidate();
			v = vn;
			s = sn;
		}

		// TODO: Use StrictMath (software) equivalent.
		let multiplier = ((s.log(::std::f64::consts::E) / s) * -2.0).sqrt();

		(v.0 * multiplier, v.1 * multiplier)
	}

	/// Returns a gaussian-distributed number with a mean of 0.0 and standard deviation of 1.0.
	pub fn next_gaussian(&mut self) -> f64 {
		match self.next_gaussian.take() {
			Some(next) => next,
			None => {
				let (v0, v1) = self.next_gaussian_pair();

				self.next_gaussian = Some(v1);

				v0
			}
		}
	}
}

/*const F32_DIV: f32 = (1u32 << 24) as f32;
const F64_DIV: f64 = (1u64 << 53) as f64;

/// Implementation of a random number generator matching the implementation in Java. Used very commonly in all versions of the Minecraft worldgen.
#[derive(Debug, Clone)]
pub struct Random {
	pub seed: i64
}

impl Random {
	/// Initializes the RNG with a seed. This is NOT the same as creating it raw, as the seed undergoes some transformation first.
	pub fn new(seed: u64) -> Self {
		let seed = seed as i64;
		Random {seed: (seed ^ 0x5DEECE66D) & ((1 << 48) - 1)}
	}

	/// Steps the RNG by one, returning up to 48 bits.
	fn next(&mut self, bits: u8) -> i32 {
		if bits > 48 {
			panic!("Too many bits!")
		}

		self.seed = (self.seed.wrapping_mul(0x5DEECE66D).wrapping_add(0xB)) & ((1 << 48) - 1);
		(self.seed >> (48 - bits)) as i32
	}

	/// Returns an i32 in the range [0, max).
	pub fn next_i32_bound(&mut self, max: i32) -> i32 {
		if max <= 0 {
			panic!("Maximum must be > 0")
		}

		if (max & -max) == max  {// i.e., n is a power of 2
			let max = max as u64;

			return ((max.wrapping_mul(self.next(31) as u64)) >> 31) as i32;
		}

		let mut bits = self.next(31) as i32;
		let mut val = bits % max;

		while bits - val + (max - 1) < 0 {
			bits = self.next(31) as i32;
			val = bits % max;
		}

		val
	}

	pub fn next_u32_bound(&mut self, max: u32) -> u32 {
		self.next_i32_bound(max as i32) as u32
	}

	/// Returns an i64. There are only 2^48 possible results from this function, as JavaRng has a 48-bit state.
	pub fn next_i64(&mut self) -> i64 {
		((self.next(32) as i64) << 32).wrapping_add(self.next(32) as i64)
	}

	pub fn next_u64(&mut self) -> u64 {
		self.next_i64() as u64
	}

	/// Returns a f32 uniformly distributed between 0.0 and 1.0.
	pub fn next_f32(&mut self) -> f32 {
		(self.next(24) as f32) / F32_DIV
	}

	/// Returns a f64 uniformly distributed between 0.0 and 1.0.
	pub fn next_f64(&mut self) -> f64 {
		let high = (self.next(26) as i64) << 27;
		let low = self.next(27) as i64;

		(high.wrapping_add(low) as f64) / F64_DIV
	}
}*/