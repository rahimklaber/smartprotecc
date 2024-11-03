use soroban_sdk::{contracttype, Address, Env};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Config,
    Asset(Address)
}

#[contracttype]
pub struct AssetConfig{
    pub liq_price: i128,
    pub pool: Address,
}

fn bump_persistent(e: &Env, key: &DataKey){
    let max = e.storage().max_ttl();
    e.storage()
    .persistent()
    .extend_ttl(key, max / 3, max);
}

pub fn set_asset_config(e: &Env, asset: Address, asset_config: AssetConfig){
    e.storage()
    .persistent()
    .set(&DataKey::Asset(asset), &asset_config);
}

pub fn get_asset_config(e: &Env, asset: Address) -> AssetConfig{
    let key = DataKey::Asset(asset);

   bump_persistent(e, &key);

    e.storage()
    .persistent()
    .get(&key)
    .unwrap()
}

pub fn set_config(e: &Env, config: &Config){
    e.storage()
    .instance()
    .set(&DataKey::Config, config);
}

pub fn get_config(e: &Env) -> Config{
    e.storage()
    .instance()
    .get(&DataKey::Config)
    .unwrap()
}

pub fn _get_config(e: &Env) -> Option<Config>{
    e.storage()
    .instance()
    .get(&DataKey::Config)
}

#[contracttype]
pub struct Config{
    pub admin: Address,
    pub reflector: Address,
    pub router: Address,
    pub safe_asset: Address,
}