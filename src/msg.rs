use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Increment {},
}

// Comes from the chain when it fires
#[cw_serde]
pub enum SudoMsg {
    AfterValidatorCreated {
        validator_address: String,
        moniker: String,
        commission: String,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(crate::state::Config)]
    GetConfig {},

    #[returns(crate::state::LastCreatedValidator)]
    GetLastValInfo {},
}

// We define a custom struct for each query response
#[cw_serde]
pub struct GetCountResponse {
    pub count: u32,
}
