use cosmwasm_schema::cw_serde;
use cw_storage_plus::Item;

#[cw_serde]
pub struct Config {
    pub val: u32,
}
pub const CONFIG: Item<Config> = Item::new("config");

#[cw_serde]
pub struct LastCreatedValidator {
    pub val_addr: String,
    pub moniker: String,
    pub commission: String,
}
pub const LAST_CREATED_VALIDATOR: Item<LastCreatedValidator> = Item::new("last_val_info");


