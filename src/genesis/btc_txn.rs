//transaction and script usage

use std::default::Default;
use num::FromPrimitive;

use bitcoin::blockdata::opcodes;
use bitcoin::blockdata::script;
use bitcoin::blockdata::transaction::{Transaction, TxOut, TxIn};
use bitcoin::blockdata::block::{Block, BlockHeader};
use bitcoin::network::constants::Network;
use bitcoin::util::misc::hex_bytes;
use bitcoin::util::hash::MerkleRoot;
use bitcoin::util::uint::Uint256;

/// The maximum allowable sequence number
pub static MAX_SEQUENCE: u32 = 0xFFFFFFFF;
/// How many satoshis are in "one bitcoin"
pub static COIN_VALUE: u64 = 100_000_000;
/// How many seconds between blocks we expect on average
pub static TARGET_BLOCK_SPACING: u32 = 600;
/// How many blocks between diffchanges
pub static DIFFCHANGE_INTERVAL: u32 = 2016;
/// How much time on average should occur between diffchanges
pub static DIFFCHANGE_TIMESPAN: u32 = 14 * 24 * 3600;

/// In Bitcoind this is insanely described as ~((u256)0 >> 32)
pub fn max_target(_: Network) -> Uint256 {
    <Uint256 as FromPrimitive>::from_u64(0xFFFF).unwrap() << 208
}

/// The maximum value allowed in an output (useful for sanity checking,
/// since keeping everything below this value should prevent overflows
/// if you are doing anything remotely sane with monetary values).
pub fn max_money(_: Network) -> u64 {
    21_000_000 * COIN_VALUE
}

/// Constructs and returns the coinbase (and only) transaction of the Bitcoin genesis block
fn bitcoin_genesis_tx() -> Transaction {
    // Base
    let mut ret = Transaction {
        version: 1,
        lock_time: 0,
        input: vec![],
        output: vec![]
    };

    // Inputs
    let in_script = script::Builder::new().push_scriptint(486604799)
                                          .push_scriptint(4)
                                          .push_slice(b"The Times 03/Jan/2009 Chancellor on brink of second bailout for banks")
                                          .into_script();
    ret.input.push(TxIn {
        prev_hash: Default::default(),
        prev_index: 0xFFFFFFFF,
        script_sig: in_script,
        sequence: MAX_SEQUENCE
    });

    // Outputs
    let out_script = script::Builder::new()
        .push_slice(&hex_bytes("04678afdb0fe5548271967f1a67130b7105cd6a828e03909a67962e0ea1f61deb649f6bc3f4cef38c4f35504e51ec112de5c384df7ba0b8d578a4c702b6bf11d5f").unwrap())
        .push_opcode(opcodes::All::OP_CHECKSIG)
        .into_script();
    ret.output.push(TxOut {
        value: 50 * COIN_VALUE,
        script_pubkey: out_script
    });

    // end
    ret
}

/// Constructs and returns the genesis block
pub fn genesis_block(network: Network) -> Block {
    match network {
        Network::Bitcoin => {
            let txdata = vec![bitcoin_genesis_tx()];
            Block {
                header: BlockHeader {
                    version: 1,
                    prev_blockhash: Default::default(),
                    merkle_root: txdata.merkle_root(),
                    time: 1231006505,
                    bits: 0x1d00ffff,
                    nonce: 2083236893
                },
                txdata: txdata
            }
        }
        Network::Testnet => {
            let txdata = vec![bitcoin_genesis_tx()];
            Block {
                header: BlockHeader {
                    version: 1,
                    prev_blockhash: Default::default(),
                    merkle_root: txdata.merkle_root(),
                    time: 1296688602,
                    bits: 0x1d00ffff,
                    nonce: 414098458
                },
                txdata: txdata
            }
        }
    }
}
