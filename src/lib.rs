use revm::{
    db::{CacheDB, EmptyDB},
    primitives::{address, Address, Bytes, ExecutionResult, ResultAndState, TransactTo},
    DatabaseCommit, Evm,
};

use modules::tokens::TokenModule;

pub mod modules;

pub const DEFAULT_ADDRESS: Address = address!("0000000000000000000000000000000000000000");

pub struct JoeUniverse {
    pub db: CacheDB<EmptyDB>,
    pub token_module: TokenModule,
}

impl JoeUniverse {
    pub fn new() -> Self {
        let db = CacheDB::new(EmptyDB::default());

        Self {
            db,
            token_module: TokenModule::new(),
        }
    }

    fn call(
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
