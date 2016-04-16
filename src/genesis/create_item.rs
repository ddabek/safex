//! Definies the methods for forming a cryptographic item for use in the safex protocol



pub struct Item {
	name: String, //name of the item
	description: String, //text description of item
	quantity: u64, //quantity of item at initialization
	rec_asset: String, //This should be the identity of the target asset recommended in exchange for this item
	rec_price: u64, //recommended quantity for purchase
	contact: String, //a message ID to get in touch with the originator
	keywords: Vec<String>, //vector of strings of keywords
	documents: Vec<String>, //this is a vector of Files found in SAFE Network which could be any kind of files which will correspond to this item
}



