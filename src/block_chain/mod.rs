use std::sync::{Mutex};

use chain::Chain;

pub mod tx;
pub mod utxo;
pub mod block;
pub mod func;
pub mod chain;
pub mod utxos;

use once_cell::sync::Lazy;

pub static BLOCK_CHAIN: Lazy<Mutex<Chain>> = Lazy::new(|| {
    Mutex::new(Chain::new())
});

