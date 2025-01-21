const PRICES_LENGTH: usize = 11;
const DEFAULT_SCALE: u64 = 1_000_000; //1e6
const SCALE_EVM: u128 = 1_000_000_000_000_000_000; //1e18
const SCALE_SVM: u128 = 10_000_000_000; // 1e10
const SCALE_MVM: u128 = 10_000_000_000; // 1e10

#[derive(Debug, Copy, Clone)]
pub struct Curve {
    pub bid_prices: [u64; PRICES_LENGTH],
    pub ask_prices: [u64; PRICES_LENGTH],
}

impl Default for Curve {
    fn default() -> Self {
        let mut bid_prices = [0; PRICES_LENGTH];
        let mut ask_prices = [0; PRICES_LENGTH];

        for i in 0..PRICES_LENGTH {
            bid_prices[i] = i as u64 * DEFAULT_SCALE * 9 / 10_000;
            ask_prices[i] = i as u64 * DEFAULT_SCALE / 1_000;
        }

        Self {
            bid_prices,
            ask_prices,
        }
    }
}

impl Curve {
    pub fn to_evm(&self) -> (Vec<u128>, Vec<u128>) {
        (
            self.bid_prices
                .iter()
                .map(|p| *p as u128 * SCALE_EVM / DEFAULT_SCALE as u128)
                .collect(),
            self.ask_prices
                .iter()
                .map(|p| *p as u128 * SCALE_EVM / DEFAULT_SCALE as u128)
                .collect(),
        )
    }

    pub fn to_svm(&self) -> (Vec<u128>, Vec<u128>) {
        (
            self.bid_prices
                .iter()
                .map(|p| *p as u128 * SCALE_SVM / DEFAULT_SCALE as u128)
                .collect(),
            self.ask_prices
                .iter()
                .map(|p| *p as u128 * SCALE_SVM / DEFAULT_SCALE as u128)
                .collect(),
        )
    }

    pub fn to_mvm(&self) -> (Vec<u128>, Vec<u128>) {
        (
            self.bid_prices
                .iter()
                .map(|p| *p as u128 * SCALE_MVM / DEFAULT_SCALE as u128)
                .collect(),
            self.ask_prices
                .iter()
                .map(|p| *p as u128 * SCALE_MVM / DEFAULT_SCALE as u128)
                .collect(),
        )
    }
}
