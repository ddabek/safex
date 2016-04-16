extern crate safex;

use safex::genesis::create_coin::*;


fn main() {
	let our_coin = Coin::new("testcoin".to_string(), 1);

	let our_name = our_coin.generate_xorname();


}