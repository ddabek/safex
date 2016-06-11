//! Definies the methods for forming a cryptocurrency for use in the safex protocol
/*
use xor_name::XorName;
use sodiumoxide::crypto::hash::sha512;

pub struct Coin {
	name: String,
	quantity: u64,
}


impl Coin {
	pub fn new(
		name: String,
		quantity: u64) 
		-> Coin {


		let mut coin = Coin {
			name: name,
			quantity: quantity,
        };
        coin
	}

	pub fn generate_xorname(&self) {
		let name = XorName::new(sha512::hash(&self.name.to_owned().into_bytes()).0);

		println!("{:?}", name);
	}
}

#[test]
fn test() {



}*/