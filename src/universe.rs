use std::convert::TryInto;

use crate::{
    commons::{
        token_mill::{
            constants::{
                TM_DEFAULT_CREATOR_FEE_SHARE, TM_DEFAULT_PROTOCOL_FEE_SHARE,
                TM_DEFAULT_REFERRAL_FEE_SHARE, TM_DEFAULT_STAKING_FEE_SHARE,
            },
            curve::Curve,
        },
        SwapAmountType, SwapType, DEFAULT_ADDRESS,
    },
    engine::EvmEngine,
    modules::{token_mill::TokenMillManager, tokens::TokenCreator},
};

const DEFAULT_QUOTE_TOKEN: &str = "Default Quote Token";
const DEFAULT_BASE_TOKEN: &str = "Default Base Token";

// JoeUniverse is a wrapper around the EvmEngine that provides a simplified API for interacting with the engine modules.
pub struct JoeUniverse {
    engine: EvmEngine,
}

impl JoeUniverse {
    pub fn new() -> Self {
        let mut engine = EvmEngine::new();

        engine
            .deploy_token_mill(TM_DEFAULT_PROTOCOL_FEE_SHARE, TM_DEFAULT_REFERRAL_FEE_SHARE)
            .unwrap();
        engine.create_token(DEFAULT_QUOTE_TOKEN, 9).unwrap();
        engine.add_quote_token(DEFAULT_QUOTE_TOKEN).unwrap();

        Self { engine }
    }

    pub fn create_market(
        &mut self,
        base_token_decimals: u8,
        total_supply: u64,
        price_curve: Curve,
    ) {
        let curve = price_curve.to_evm();

        self.engine
            .create_token_and_market(
                DEFAULT_BASE_TOKEN,
                base_token_decimals,
                DEFAULT_QUOTE_TOKEN,
                total_supply.into(),
                curve.0,
                curve.1,
                TM_DEFAULT_CREATOR_FEE_SHARE,
                TM_DEFAULT_STAKING_FEE_SHARE,
            )
            .unwrap();
    }

    pub fn swap(
        &mut self,
        swap_type: SwapType,
        swap_amount_type: SwapAmountType,
        amount: u64,
    ) -> (u64, u64) {
        let market = *self
            .engine
            .token_mill_module
            .get_market(DEFAULT_BASE_TOKEN)
            .unwrap();

        let base_to_quote = swap_type == SwapType::Sell;

        let delta_amount = match swap_amount_type {
            SwapAmountType::ExactInput => amount as i128,
            SwapAmountType::ExactOutput => -(amount as i128),
        };

        let amount_in = match swap_amount_type {
            SwapAmountType::ExactInput => amount as u128,
            SwapAmountType::ExactOutput => self
                .engine
                .get_amount_in(DEFAULT_BASE_TOKEN, delta_amount, base_to_quote)
                .unwrap(),
        };

        let (token_in, token_out) = if base_to_quote {
            (DEFAULT_BASE_TOKEN, DEFAULT_QUOTE_TOKEN)
        } else {
            (DEFAULT_QUOTE_TOKEN, DEFAULT_BASE_TOKEN)
        };

        let balance_before = self.engine.balance_of(token_out, DEFAULT_ADDRESS).unwrap();

        self.engine
            .transfer(token_in, DEFAULT_ADDRESS, market, amount_in)
            .unwrap();

        self.engine
            .swap(DEFAULT_BASE_TOKEN, delta_amount, base_to_quote)
            .unwrap();

        let balance_after = self.engine.balance_of(token_out, DEFAULT_ADDRESS).unwrap();

        let amount_out = balance_after - balance_before;

        (
            amount_in.try_into().unwrap(),
            amount_out.try_into().unwrap(),
        )
    }

    pub fn claim_fees(&mut self) -> (u64, u64, u64) {
        let creator_fee = self.engine.claim_creator_fees(DEFAULT_BASE_TOKEN).unwrap();

        let referral_fee = self
            .engine
            .claim_referral_fees(DEFAULT_QUOTE_TOKEN)
            .unwrap();

        let protocol_fee = self
            .engine
            .claim_protocol_fees(DEFAULT_QUOTE_TOKEN)
            .unwrap();

        (creator_fee, referral_fee, protocol_fee)
    }

    pub fn deposit(&mut self, amount: u64) {
        self.engine
            .deposit(DEFAULT_BASE_TOKEN, amount.into())
            .unwrap();
    }

    pub fn withdraw(&mut self, amount: u64) {
        self.engine
            .withdraw(DEFAULT_BASE_TOKEN, amount.into())
            .unwrap();
    }

    pub fn claim_staking_rewards(&mut self) -> u64 {
        self.engine
            .claim_staking_rewards(DEFAULT_BASE_TOKEN)
            .unwrap()
    }
}
