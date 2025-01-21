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
    joe_universe: EvmEngine,
}

impl JoeUniverse {
    pub fn new() -> Self {
        let mut joe_universe = EvmEngine::new();

        joe_universe
            .deploy_token_mill(TM_DEFAULT_PROTOCOL_FEE_SHARE, TM_DEFAULT_REFERRAL_FEE_SHARE)
            .unwrap();
        joe_universe.create_token(DEFAULT_QUOTE_TOKEN, 9).unwrap();
        joe_universe.add_quote_token(DEFAULT_QUOTE_TOKEN).unwrap();

        Self { joe_universe }
    }

    pub fn create_market(
        &mut self,
        base_token_decimals: u8,
        total_supply: u64,
        price_curve: Curve,
    ) {
        let curve = price_curve.to_evm();

        self.joe_universe
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
            .joe_universe
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
                .joe_universe
                .get_amount_in(DEFAULT_BASE_TOKEN, delta_amount, base_to_quote)
                .unwrap(),
        };

        let (token_in, token_out) = if base_to_quote {
            (DEFAULT_BASE_TOKEN, DEFAULT_QUOTE_TOKEN)
        } else {
            (DEFAULT_QUOTE_TOKEN, DEFAULT_BASE_TOKEN)
        };

        let balance_before = self
            .joe_universe
            .balance_of(token_out, DEFAULT_ADDRESS)
            .unwrap();

        self.joe_universe
            .transfer(token_in, DEFAULT_ADDRESS, market, amount_in)
            .unwrap();

        self.joe_universe
            .swap(DEFAULT_BASE_TOKEN, delta_amount, base_to_quote)
            .unwrap();

        let balance_after = self
            .joe_universe
            .balance_of(token_out, DEFAULT_ADDRESS)
            .unwrap();

        let amount_out = balance_after - balance_before;

        (
            amount_in.try_into().unwrap(),
            amount_out.try_into().unwrap(),
        )
    }

    pub fn claim_fees(&mut self) -> (u64, u64, u64) {
        let creator_fee = self
            .joe_universe
            .claim_creator_fees(DEFAULT_BASE_TOKEN)
            .unwrap();

        let referral_fee = self
            .joe_universe
            .claim_referral_fees(DEFAULT_QUOTE_TOKEN)
            .unwrap();

        let protocol_fee = self
            .joe_universe
            .claim_protocol_fees(DEFAULT_QUOTE_TOKEN)
            .unwrap();

        (creator_fee, referral_fee, protocol_fee)
    }

    pub fn deposit(&mut self, amount: u64) {
        self.joe_universe
            .deposit(DEFAULT_BASE_TOKEN, amount.into())
            .unwrap();
    }

    pub fn withdraw(&mut self, amount: u64) {
        self.joe_universe
            .withdraw(DEFAULT_BASE_TOKEN, amount.into())
            .unwrap();
    }

    pub fn claim_staking_rewards(&mut self) -> u64 {
        self.joe_universe
            .claim_staking_rewards(DEFAULT_BASE_TOKEN)
            .unwrap()
    }
}
