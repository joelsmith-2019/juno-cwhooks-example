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
    // "after_validator_created":{"moniker":"test123","validator_address":"junovaloper1efd63aw40lxf3n4mhf7dzhjkr453axurnh5ze0","commission":"0.050000000000000000","validator_tokens":"0","bonded_tokens":"0","bond_status":"BOND_STATUS_UNBONDED"}}
    AfterValidatorCreated {
        moniker: String,
        validator_address: String,
        commission: String,
        validator_tokens: String,
        bonded_tokens: String,
        bond_status: String,
    },
    // BeforeValidatorCreated
    // AfterValidatorModified

    // {"after_delegation_modified":{"validator_address":"junovaloper1efd63aw40lxf3n4mhf7dzhjkr453axurnh5ze0","delegator_address":"juno1efd63aw40lxf3n4mhf7dzhjkr453axurv2zdzk","shares":"1.000000000000000000"}}
    AfterDelegationModified {
        validator_address: String,
        delegator_address: String,
        shares: String,
    },
    
    BeforeValidatorSlashed {
        moniker: String,
        validator_address: String,
        slashed_amount: String,
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
