use std::collections::HashMap;

use alloy::{dyn_abi::DynSolValue, hex, sol, sol_types::SolCall};
use revm::primitives::{Address, ExecutionResult, Output, TransactTo, U256};

use anyhow::{anyhow, Result};

use crate::{JoeUniverse, DEFAULT_ADDRESS};

const ERC20_BYTECODE: [u8; 2796] = hex!("60a06040523480156200001157600080fd5b5060405162000aec38038062000aec83398101604081905262000034916200012a565b828260036200004483826200023e565b5060046200005382826200023e565b50505060ff16608052506200030a9050565b634e487b7160e01b600052604160045260246000fd5b600082601f8301126200008d57600080fd5b81516001600160401b0380821115620000aa57620000aa62000065565b604051601f8301601f19908116603f01168101908282118183101715620000d557620000d562000065565b81604052838152602092508683858801011115620000f257600080fd5b600091505b83821015620001165785820183015181830184015290820190620000f7565b600093810190920192909252949350505050565b6000806000606084860312156200014057600080fd5b83516001600160401b03808211156200015857600080fd5b62000166878388016200007b565b945060208601519150808211156200017d57600080fd5b506200018c868287016200007b565b925050604084015160ff81168114620001a457600080fd5b809150509250925092565b600181811c90821680620001c457607f821691505b602082108103620001e557634e487b7160e01b600052602260045260246000fd5b50919050565b601f8211156200023957600081815260208120601f850160051c81016020861015620002145750805b601f850160051c820191505b81811015620002355782815560010162000220565b5050505b505050565b81516001600160401b038111156200025a576200025a62000065565b62000272816200026b8454620001af565b84620001eb565b602080601f831160018114620002aa5760008415620002915750858301515b600019600386901b1c1916600185901b17855562000235565b600085815260208120601f198616915b82811015620002db57888601518255948401946001909101908401620002ba565b5085821015620002fa5787850151600019600388901b60f8161c191681555b5050505050600190811b01905550565b6080516107c662000326600039600061013001526107c66000f3fe608060405234801561001057600080fd5b50600436106100be5760003560e01c806340c10f191161007657806395d89b411161005b57806395d89b4114610198578063a9059cbb146101a0578063dd62ed3e146101b357600080fd5b806340c10f191461015a57806370a082311461016f57600080fd5b806318160ddd116100a757806318160ddd1461010457806323b872dd14610116578063313ce5671461012957600080fd5b806306fdde03146100c3578063095ea7b3146100e1575b600080fd5b6100cb6101ec565b6040516100d89190610610565b60405180910390f35b6100f46100ef36600461067a565b61027e565b60405190151581526020016100d8565b6002545b6040519081526020016100d8565b6100f46101243660046106a4565b610298565b60405160ff7f00000000000000000000000000000000000000000000000000000000000000001681526020016100d8565b61016d61016836600461067a565b6102bc565b005b61010861017d3660046106e0565b6001600160a01b031660009081526020819052604090205490565b6100cb6102ca565b6100f46101ae36600461067a565b6102d9565b6101086101c1366004610702565b6001600160a01b03918216600090815260016020908152604080832093909416825291909152205490565b6060600380546101fb90610735565b80601f016020809104026020016040519081016040528092919081815260200182805461022790610735565b80156102745780601f1061024957610100808354040283529160200191610274565b820191906000526020600020905b81548152906001019060200180831161025757829003601f168201915b5050505050905090565b60003361028c8185856102e7565b60019150505b92915050565b6000336102a68582856102f9565b6102b185858561037c565b506001949350505050565b6102c682826103db565b5050565b6060600480546101fb90610735565b60003361028c81858561037c565b6102f48383836001610411565b505050565b6001600160a01b038381166000908152600160209081526040808320938616835292905220546000198114610376578181101561036757604051637dc7a0d960e11b81526001600160a01b038416600482015260248101829052604481018390526064015b60405180910390fd5b61037684848484036000610411565b50505050565b6001600160a01b0383166103a657604051634b637e8f60e11b81526000600482015260240161035e565b6001600160a01b0382166103d05760405163ec442f0560e01b81526000600482015260240161035e565b6102f48383836104e6565b6001600160a01b0382166104055760405163ec442f0560e01b81526000600482015260240161035e565b6102c6600083836104e6565b6001600160a01b03841661043b5760405163e602df0560e01b81526000600482015260240161035e565b6001600160a01b03831661046557604051634a1406b160e11b81526000600482015260240161035e565b6001600160a01b038085166000908152600160209081526040808320938716835292905220829055801561037657826001600160a01b0316846001600160a01b03167f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925846040516104d891815260200190565b60405180910390a350505050565b6001600160a01b038316610511578060026000828254610506919061076f565b909155506105839050565b6001600160a01b038316600090815260208190526040902054818110156105645760405163391434e360e21b81526001600160a01b0385166004820152602481018290526044810183905260640161035e565b6001600160a01b03841660009081526020819052604090209082900390555b6001600160a01b03821661059f576002805482900390556105be565b6001600160a01b03821660009081526020819052604090208054820190555b816001600160a01b0316836001600160a01b03167fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef8360405161060391815260200190565b60405180910390a3505050565b600060208083528351808285015260005b8181101561063d57858101830151858201604001528201610621565b506000604082860101526040601f19601f8301168501019250505092915050565b80356001600160a01b038116811461067557600080fd5b919050565b6000806040838503121561068d57600080fd5b6106968361065e565b946020939093013593505050565b6000806000606084860312156106b957600080fd5b6106c28461065e565b92506106d06020850161065e565b9150604084013590509250925092565b6000602082840312156106f257600080fd5b6106fb8261065e565b9392505050565b6000806040838503121561071557600080fd5b61071e8361065e565b915061072c6020840161065e565b90509250929050565b600181811c9082168061074957607f821691505b60208210810361076957634e487b7160e01b600052602260045260246000fd5b50919050565b8082018082111561029257634e487b7160e01b600052601160045260246000fdfea2646970667358221220c57fe874dcb4f45e8502823534f851978dba1f647c0032f5c6889c47245e27db64736f6c63430008140033");

pub struct TokenModule {
    pub tokens: HashMap<String, Address>,
}

impl TokenModule {
    pub fn new() -> Self {
        Self {
            tokens: HashMap::new(),
        }
    }

    pub fn get_token(&self, name: &str) -> Result<&Address> {
        self.tokens.get(name).ok_or(anyhow!("Token not found"))
    }
}

pub trait TokenCreator {
    fn create_token(&mut self, name: &str, decimals: u8) -> Result<Address>;
    fn mint(&mut self, token: &str, to: Address, amount: u128) -> Result<()>;
    fn approve(
        &mut self,
        token: &str,
        owner: Address,
        spender: Address,
        amount: u128,
    ) -> Result<()>;
    fn balance_of(&mut self, token: &str, owner: Address) -> Result<u128>;
    fn transfer_from(
        &mut self,
        token: &str,
        from: Address,
        to: Address,
        amount: u128,
    ) -> Result<()>;
}

impl TokenCreator for JoeUniverse {
    fn create_token(&mut self, name: &str, decimals: u8) -> Result<Address> {
        let mut constructor_data = DynSolValue::Tuple(vec![
            DynSolValue::String(name.to_string()),
            DynSolValue::String("".to_string()),
            decimals.into(),
        ])
        .abi_encode();

        // Ugly, must be a better way to do this
        constructor_data.drain(0..32);

        let data = ERC20_BYTECODE
            .into_iter()
            .chain(constructor_data)
            .collect::<Vec<_>>();

        let result = self.call(DEFAULT_ADDRESS, TransactTo::Create, data.into(), true);

        let contract_address = match result {
            ExecutionResult::Success {
                output: Output::Create(_, address),
                ..
            } => address.ok_or(anyhow!("ERC20 deployment failed: {result:?}"))?,
            result => return Err(anyhow!("ERC20 execution failed: {result:?}")),
        };

        self.token_module
            .tokens
            .insert(name.to_string(), contract_address);

        Ok(contract_address)
    }

    fn mint(&mut self, token: &str, to: Address, amount: u128) -> Result<()> {
        let token_address = self.token_module.get_token(token)?;

        sol! {
            function mint(address recipient, uint256 amount) external;
        }

        let data: Vec<u8> = mintCall {
            recipient: to,
            amount: U256::from(amount),
        }
        .abi_encode();

        let result = self.call(
            DEFAULT_ADDRESS,
            TransactTo::Call(*token_address),
            data.into(),
            true,
        );

        match result {
            ExecutionResult::Success { .. } => Ok(()),
            result => Err(anyhow!("Mint failed: {result:?}")),
        }
    }

    fn approve(
        &mut self,
        token: &str,
        owner: Address,
        spender: Address,
        amount: u128,
    ) -> Result<()> {
        let token_address = self.token_module.get_token(token)?;

        sol! {
            function approve(address spender, uint256 amount) external;
        }

        let data: Vec<u8> = approveCall {
            spender,
            amount: U256::from(amount),
        }
        .abi_encode();

        let result = self.call(owner, TransactTo::Call(*token_address), data.into(), true);

        match result {
            ExecutionResult::Success { .. } => Ok(()),
            result => Err(anyhow!("Approval failed: {result:?}")),
        }
    }

    fn transfer_from(
        &mut self,
        token: &str,
        from: Address,
        to: Address,
        amount: u128,
    ) -> Result<()> {
        let token_address = self.token_module.get_token(token)?;

        sol! {
            function transferFrom(address from, address to, uint256 amount) external;
        }

        let data: Vec<u8> = transferFromCall {
            from,
            to,
            amount: U256::from(amount),
        }
        .abi_encode();

        let result = self.call(to, TransactTo::Call(*token_address), data.into(), true);

        match result {
            ExecutionResult::Success { .. } => Ok(()),
            result => Err(anyhow!("TransferFrom failed: {result:?}")),
        }
    }

    fn balance_of(&mut self, token: &str, owner: Address) -> Result<u128> {
        let token_address = self.token_module.get_token(token)?;

        sol! {
            function balanceOf(address owner) external view returns (uint256);
        }

        let data: Vec<u8> = balanceOfCall { owner }.abi_encode();

        let result = self.call(
            DEFAULT_ADDRESS,
            TransactTo::Call(*token_address),
            data.into(),
            false,
        );

        match result {
            ExecutionResult::Success {
                output: Output::Call(data),
                ..
            } => Ok(u128::try_from(
                balanceOfCall::abi_decode_returns(&data, false)?._0,
            )?),
            result => Err(anyhow!("BalanceOf failed: {result:?}")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use revm::primitives::address;

    const ALICE: Address = address!("0000000000000000000000000000000000000001");
    const BOB: Address = address!("0000000000000000000000000000000000000002");

    #[test]
    fn test_create_token() {
        let mut joe_universe = JoeUniverse::new();
        let token_address = joe_universe.create_token("TestToken", 18).unwrap();

        assert_eq!(joe_universe.token_module.tokens.len(), 1);
        assert_eq!(
            joe_universe.token_module.tokens.get("TestToken").unwrap(),
            &token_address
        );
    }

    #[test]
    fn test_mint() {
        let mut joe_universe = JoeUniverse::new();

        joe_universe.create_token("TestToken", 18).unwrap();

        joe_universe.mint("TestToken", ALICE, 100).unwrap();

        let balance = joe_universe.balance_of("TestToken", ALICE).unwrap();

        assert_eq!(balance, 100);
    }

    #[test]
    fn test_approve_and_transfer_from() {
        let mut joe_universe = JoeUniverse::new();

        joe_universe.create_token("TestToken", 18).unwrap();

        joe_universe.mint("TestToken", ALICE, 100).unwrap();

        joe_universe.approve("TestToken", ALICE, BOB, 100).unwrap();

        joe_universe
            .transfer_from("TestToken", ALICE, BOB, 50)
            .unwrap();

        let alice_balance = joe_universe.balance_of("TestToken", ALICE).unwrap();
        assert_eq!(alice_balance, 50);

        let bob_balance = joe_universe.balance_of("TestToken", BOB).unwrap();
        assert_eq!(bob_balance, 50);
    }
}
