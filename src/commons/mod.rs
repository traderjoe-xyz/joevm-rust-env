use revm::primitives::{address, Address};

pub mod token_mill;

pub const DEFAULT_ADDRESS: Address = address!("0000000000000000000000000000000000000001");

#[derive(Debug, Clone, PartialEq)]
pub enum SwapType {
    Buy,
    Sell,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SwapAmountType {
    ExactInput,
    ExactOutput,
}
