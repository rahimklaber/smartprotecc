#![no_std]
use soroban_sdk::{auth::{Context, ContractContext}, contract, contracterror, contractimpl, panic_with_error, vec, Address, Env, String, Vec};


#[contracterror]
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u32)]
pub enum Error {
    NotAllowed = 1,
}


#[contract]
pub struct ProteccPolicy;

#[contractimpl]
impl ProteccPolicy {
    pub fn policy__(env: Env, _source: Address, contexts: Vec<Context>){
        for context in contexts.iter() {
            if let Context::Contract(ContractContext { fn_name, args, .. }) = context{
                
            }
        }

        panic_with_error!(&env, Error::NotAllowed);
    }
}

mod test;
