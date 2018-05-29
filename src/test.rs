use Random;
use test_data::*;

#[test]
fn test_nextbytes() {
	let mut random = Random::new(RAND_NEXTBYTES_SEED);

	let mut bytes = [0; 128];
	random.next_bytes(&mut bytes);

	assert_eq!(&bytes as &[u8], &RAND_NEXTBYTES as &[u8]);
}

#[test]
fn test_next32() {
	let mut random = Random::new(RAND_NEXT32_SEED);

	for (index, &elem) in RAND_NEXT32.iter().enumerate() {
		let gen = random.next_u32();

		if gen != elem {
			panic!("mismatch at index {}: expected {}, got {}", index, elem, gen);
		}
	}
}

#[test]
fn test_next64() {
	let mut random = Random::new(RAND_NEXT64_SEED);

	for (index, &elem) in RAND_NEXT64.iter().enumerate() {
		let gen = random.next_u64();

		if gen != elem {
			panic!("mismatch at index {}: expected {}, got {}", index, elem, gen);
		}
	}
}

#[test]
fn test_next32_bound_65536() {
	let mut random = Random::new(RAND_NEXT32_BOUND_65536_SEED);

	for (index, &elem) in RAND_NEXT32_BOUND_65536.iter().enumerate() {
		let gen = random.next_u32_bound(65536);

		if gen != elem {
			panic!("mismatch at index {}: expected {}, got {}", index, elem, gen);
		}
	}
}

#[test]
fn test_next32_bound_999999999() {
	let mut random = Random::new(RAND_NEXT32_BOUND_999999999_SEED);

	for (index, &elem) in RAND_NEXT32_BOUND_999999999.iter().enumerate() {
		let gen = random.next_u32_bound(999999999);

		if gen != elem {
			panic!("mismatch at index {}: expected {}, got {}", index, elem, gen);
		}
	}
}

#[test]
fn test_nextbool() {
	let mut random = Random::new(RAND_NEXTBOOL_SEED);

	for (index, &elem) in RAND_NEXTBOOL.iter().enumerate() {
		let gen = random.next_bool();

		if gen != elem {
			panic!("mismatch at index {}: expected {}, got {}", index, elem, gen);
		}
	}
}

#[test]
fn test_nextf32() {
	let mut random = Random::new(RAND_NEXTF32_SEED);

	for (index, &elem) in RAND_NEXTF32.iter().enumerate() {
		let gen = random.next_f32().to_bits();

		if gen != elem {
			panic!("mismatch at index {}: expected {}, got {}", index, elem, gen);
		}
	}
}

#[test]
fn test_nextf64() {
	let mut random = Random::new(RAND_NEXTF64_SEED);

	for (index, &elem) in RAND_NEXTF64.iter().enumerate() {
		let gen = random.next_f64().to_bits();

		if gen != elem {
			panic!("mismatch at index {}: expected {}, got {}", index, elem, gen);
		}
	}
}

#[test]
fn test_nextgaussian() {
	let mut random = Random::new(RAND_NEXTGAUSSIAN_SEED);

	for (index, &elem) in RAND_NEXTGAUSSIAN.iter().enumerate() {
		let gen = random.next_gaussian().to_bits();

		if gen != elem {
			panic!("mismatch at index {}: expected {}, got {}", index, elem, gen);
		}
	}
}