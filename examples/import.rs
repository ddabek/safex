extern crate safex;
extern crate rustc_serialize;


use std::io;
use std::io::Read;
use std::io::{BufRead};


use safex::genesis::key_generation::KeyPair;

fn main() {



		println!("Enter \"1\" for base64 or \"2\" for WIF import");
		let mut input2 = String::new();
    	let stdin2 = io::stdin();
    	stdin2.lock().read_line(&mut input2).unwrap();

    	let trimmed = input2.trim_right_matches("\n");

    	let answer_parse = trimmed.parse().ok().expect("invalid entry");


    	match answer_parse {
    		1 => base64(),
    		2 => base58(),
    		_ => println!("something is wrong idk what"),
    	};
}

fn base64() {
	println!("input your private key");
	let mut input2 = String::new();
    let stdin2 = io::stdin();
    stdin2.lock().read_line(&mut input2).unwrap();

    let trimmed = input2.trim_right_matches("\n");
	let new_keys = KeyPair::keypair_frombase64(trimmed.to_string());
	println!("public key {:?}", KeyPair::address_base58(&new_keys.public));
}

fn base58() {
	println!("input your private key");
	let mut input2 = String::new();
    let stdin2 = io::stdin();
    stdin2.lock().read_line(&mut input2).unwrap();

    let trimmed = input2.trim_right_matches("\n");
	let new_keys = KeyPair::keypair_frombase58wif(trimmed.to_string());
	println!("public key {:?}", KeyPair::address_base58(&new_keys.public));
}