extern crate safex;

use safex::genesis::key_generation::KeyPair;

fn main() {

	let keys = KeyPair::create().ok().expect("error");
	println!("base64 private key: {:?}", KeyPair::private_key_tobase64(keys.secret));
	println!("base58 private key: {:?}", KeyPair::private_key_base58(keys.secret));
	println!("base58 bitcoin address: {:?}", KeyPair::address_base58(&keys.public));
}
