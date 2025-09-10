
<div align="center">
# lfsr-fibo

	![Crates.io Total Downloads](https://img.shields.io/crates/d/lfsr-fibo)
	![Crates.io Version](https://img.shields.io/crates/v/lfsr-fibo)

Efficient pure Rust implementation of LFSR in fibonacci representation.

Please see installation details and doc on [crates.io](https://crates.io/crates/lfsr-fibo). 

</div>
## Usage

This is an example of a basic usage. Let say that you want to generate 256 bits from the LFSR represented by the polynomial `x^11 + x^6 + x^3 + 1` with an initial state of `10101101110`, then you can use this crate in the folowing way :

```rust
use std::collections::VecDeque;
use lfsr_fibo::Lfsr;

fn main() {
	let mut lfsr = Lfsr::new(vec![11, 6, 3]);
	lfsr.set_key(VecDeque::from([1, 0, 1, 0, 1, 1, 0, 1, 1, 1, 0]));

	let mut generated: Vec<u8> = vec![];

	for _i in 0..256 {
		generated.push(lfsr.clock());
	}
	println!("Generated : {:?}", generated);
}