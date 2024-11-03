#![no_std]

use soroban_sdk::{auth::{Context, ContractContext}, contract, contracterror, contractimpl, contracttype, panic_with_error, token::TokenClient, vec, Address, BytesN, Env, String, Vec};
use storage::{get_config, Config, _get_config, set_asset_config, set_config, AssetConfig, get_asset_config};



#[contract]
pub struct SmartProtecc;


mod reflector{
    use soroban_sdk::contractimport;

    contractimport!(
        file = "../../reflector-oracle_v4.1.0.wasm"
    );
}

mod soroswap{
    use soroban_sdk::contractimport;

    contractimport!(
        file = "../../soroswap-router_v0.0.1.wasm"
    );
}

#[contracterror]
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u32)]
pub enum Error {
    AlreadyInit = 1,
}


pub trait SmartProteccTrait {
    fn init(e: Env, config: Config);

    fn protecc(e: Env, address: Address, asset: Address); 

    fn add_asset(e: Env, asset: Address, asset_config: AssetConfig);
}

#[contractimpl]
impl SmartProtecc {
    pub fn upgrade(e: Env, new_wasm_hash: BytesN<32>) {
        get_config(&e).admin.require_auth();

        e.deployer().update_current_contract_wasm(new_wasm_hash);
    }
}

#[contractimpl]
impl SmartProteccTrait for SmartProtecc{
    fn init(e: Env, config: Config){
        if let Some(_) = _get_config(&e){
            panic_with_error!(&e, Error::AlreadyInit);
        }


        set_config(&e, &config);
    }

    fn add_asset(e: Env, asset: Address, asset_config: AssetConfig){
        get_config(&e).admin.require_auth();

        set_asset_config(&e, asset, asset_config);
    }

    fn protecc(e: Env, address: Address, asset: Address){
        address.require_auth();

        let config = get_config(&e);
        let asset_config = get_asset_config(&e, asset.clone());

        let reflector = reflector::Client::new(&e, &config.reflector);
        
        let last_price = reflector.lastprice(&reflector::Asset::Stellar(asset.clone())).unwrap();

        if last_price.price < asset_config.liq_price{
            let router = soroswap::Client::new(&e, &address);

            let amount_to_swap = TokenClient::new(&e, &address).balance(&address);
            router.swap_exact_tokens_for_tokens(&amount_to_swap, &0, &vec![&e, asset.clone(), config.safe_asset], &address, &(e.ledger().timestamp() + 10));
        }
    }

}



mod test;
mod storage;
