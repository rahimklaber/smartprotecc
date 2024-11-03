#![no_std]
use soroban_sdk::{auth::{Context, ContractContext}, contract, contracterror, contractimpl, panic_with_error, symbol_short, vec, xdr::ScVal, Address, Env, String, Val, Vec};


#[contracterror]
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u32)]
pub enum Error {
    NotAllowed = 1,
    AlreadyInit = 2,
}


#[contract]
pub struct ProteccPolicy;


fn _get_protecc_address(e: &Env) -> Option<Address>{
    e.storage()
    .instance()
    .get(&Val::VOID)
}

fn get_protecc_address(e: &Env) -> Address{
    _get_protecc_address(e).unwrap()
}

fn set_protecc_address(e: &Env, address: &Address){
    e.storage()
    .instance()
    .set(&Val::VOID, address)
}


#[contractimpl]
impl ProteccPolicy {
    pub fn init(e: Env, protecc: Address){
        if let Some(_) =_get_protecc_address(&e){
            panic_with_error!(&e, Error::AlreadyInit);
        }

        set_protecc_address(&e, &protecc);
    }

    pub fn policy__(e: Env, _source: Address, contexts: Vec<Context>){
        let protecc = get_protecc_address(&e);

        for context in contexts.iter() {
            if let Context::Contract(ContractContext { fn_name, contract, .. }) = context{
                if contract == protecc && fn_name == symbol_short!("protecc"){
                    return;
                }
            }
        }

        panic_with_error!(&e, Error::NotAllowed);
    }
}

mod test;
