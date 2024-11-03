#![cfg(test)]

use super::*;
use soroban_sdk::{vec, Env, String};

#[test]
fn test() {
    let e = Env::default();
    
    let smart_wallet1 = e.register_contract(None, smart_wallet::Contract);

    let w_1 = smart_wallet::ContractClient::new(&e, &smart_wallet1);

    // w_1.add_signer(&smart_wallet_interface::types::Signer::Ed25519((), (), (), ()));

}
