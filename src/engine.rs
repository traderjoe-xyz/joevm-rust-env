use revm::{
    db::{CacheDB, EmptyDB},
    primitives::{Address, Bytes, ExecutionResult, ResultAndState, TransactTo},
    DatabaseCommit, Evm,
};

use super::modules::{token_mill::TokenMillModule, tokens::TokenModule};

pub struct EvmEngine {
    pub db: CacheDB<EmptyDB>,

    pub token_module: TokenModule,
    pub token_mill_module: TokenMillModule,
}

impl Default for EvmEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl EvmEngine {
    pub fn new() -> Self {
        let db = CacheDB::new(EmptyDB::default());

        Self {
            db,
            token_module: TokenModule::new(),
            token_mill_module: TokenMillModule::new(),
        }
    }

    pub fn call(
        &mut self,
        caller: Address,
        transact_to: TransactTo,
        data: Bytes,
        commit: bool,
    ) -> ExecutionResult {
        let ResultAndState {
            state: changes,
            result,
        } = {
            let mut evm = Evm::builder()
                .modify_cfg_env(|cfg| cfg.disable_balance_check = true)
                .with_ref_db(&self.db)
                .modify_tx_env(|tx| {
                    tx.caller = caller;
                    tx.transact_to = transact_to;
                    tx.data = data;
                })
                .build();

            evm.transact().unwrap()
        };

        if commit {
            self.db.commit(changes);
        }

        result
    }
}
