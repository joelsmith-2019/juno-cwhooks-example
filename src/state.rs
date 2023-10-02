use cosmwasm_schema::cw_serde;
use cw_storage_plus::Item;

#[cw_serde]
pub struct Config {
    pub calls: u32,
}
pub const CONFIG: Item<Config> = Item::new("config");

// x/cw-hooks/keeper/staking_hook_types.go
#[cw_serde]
pub struct Validator {
    pub moniker: String,
    pub validator_address: String,
    pub commission: String,
    pub validator_tokens: String,
    pub bonded_tokens: String,
    pub bond_status: String,
}

#[cw_serde]
pub struct Delegation {
    pub validator_address: String,
    pub delegator_address: String,
    pub shares: String,
}

#[cw_serde]
pub struct ValidatorSlashed {
    pub moniker: String,
    pub validator_address: String,
    pub slashed_amount: String,
}

pub const LAST_VALIDATOR: Item<Validator> = Item::new("lcv");

// any actions which uses delegations uses this
pub const LAST_DELEGATION_CHANGE: Item<Delegation> = Item::new("ldc");

pub const VALIDATOR_SLASHED: Item<ValidatorSlashed> = Item::new("vs");
