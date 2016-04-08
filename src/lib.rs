




#![crate_name = "safex"]

extern crate secp256k1;
extern crate rand;
extern crate rustc_serialize;
extern crate crypto;
extern crate byteorder;
#[macro_use(lazy_static)]
extern crate lazy_static;
extern crate bitcoin;
extern crate num;

#[macro_use]
pub mod genesis;