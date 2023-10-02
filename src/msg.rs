use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint64;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Increment {},
}

// create a type named Validator
#[cw_serde]
pub struct Validator {
    pub moniker: String,
    pub validator_address: String,
    pub commission: String,
    pub validator_tokens: String,
    pub bonded_tokens: String,
    pub bond_status: String,
}

// Comes from the chain when it fires
#[cw_serde]
pub enum SudoMsg {
    // Validators
    AfterValidatorCreated {
        moniker: String,
        validator_address: String,
        commission: String,
        validator_tokens: String,
        bonded_tokens: String,
        bond_status: String,
    },
    AfterValidatorRemoved {
        moniker: String,
        validator_address: String,
        commission: String,
        validator_tokens: String,
        bonded_tokens: String,
        bond_status: String,
    },
    BeforeValidatorModified {
        moniker: String,
        validator_address: String,
        commission: String,
        validator_tokens: String,
        bonded_tokens: String,
        bond_status: String,
    },
    AfterValidatorModified {
        moniker: String,
        validator_address: String,
        commission: String,
        validator_tokens: String,
        bonded_tokens: String,
        bond_status: String,
    },
    AfterValidatorBonded {
        moniker: String,
        validator_address: String,
        commission: String,
        validator_tokens: String,
        bonded_tokens: String,
        bond_status: String,
    },
    AfterValidatorBeginUnbonding {
        moniker: String,
        validator_address: String,
        commission: String,
        validator_tokens: String,
        bonded_tokens: String,
        bond_status: String,
    },
    BeforeValidatorSlashed {
        moniker: String,
        validator_address: String,
        slashed_amount: String,
    },

    // Delegations
    BeforeDelegationCreated {
        validator_address: String,
        delegator_address: String,
        shares: String,
    },
    BeforeDelegationSharesModified {
        validator_address: String,
        delegator_address: String,
        shares: String,
    },
    AfterDelegationModified {
        validator_address: String,
        delegator_address: String,
        shares: String,
    },
    BeforeDelegationRemoved {
        validator_address: String,
        delegator_address: String,
        shares: String,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(crate::state::Config)]
    GetConfig {},

    #[returns(crate::state::Validator)]
    LastValChange {},

    #[returns(crate::state::Delegation)]
    LastDelegationChange {},

    #[returns(crate::state::ValidatorSlashed)]
    LastValidatorSlash {},
}

// We define a custom struct for each query response
#[cw_serde]
pub struct GetCountResponse {
    pub count: u32,
}
