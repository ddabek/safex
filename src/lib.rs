




#![crate_name = "safex"]

extern crate secp256k1;
extern crate rand;
extern crate rustc_serialize;
extern crate crypto;
extern crate byteorder;
#[macro_use(lazy_static)]
extern crate lazy_static;
extern crate bitcoin;


//SAFE Related
//extern crate xor_name;
//extern crate sodiumoxide;

#[macro_use]
pub mod genesis;
pub mod contracts;