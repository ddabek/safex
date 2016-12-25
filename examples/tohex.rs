extern crate safex;
extern crate rustc_serialize;


use std::io;
use std::io::Read;
use std::io::{BufRead};
use rustc_serialize::base64::{self, ToBase64, FromBase64, STANDARD};
use rustc_serialize::hex::{ToHex, FromHex};

fn main() {

		println!("Enter the value you want to convert to hex");
		let mut input = String::new();
    	let stdin = io::stdin();
    	stdin.lock().read_line(&mut input).unwrap();

    	let trimmed = input.trim_right_matches("\n");

    	let answer_parse: String = trimmed.parse().unwrap();



    	print!("{:?}", &answer_parse.as_bytes().to_hex());

    }