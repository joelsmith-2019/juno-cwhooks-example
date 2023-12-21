#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, SudoMsg};
use crate::state::{
    Config, Delegation, Validator, ValidatorSlashed, CONFIG, LAST_DELEGATION_CHANGE,
    LAST_VALIDATOR, VALIDATOR_SLASHED,
};

const CONTRACT_NAME: &str = "crates.io:cw-ibc-example";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    CONFIG.save(deps.storage, &Config { calls: 0 })?;

    LAST_VALIDATOR.save(
        deps.storage,
        &Validator {
            validator_address: "".to_string(),
            moniker: "".to_string(),
            commission: "".to_string(),
            bond_status: "".to_string(),
            bonded_tokens: "".to_string(),
            validator_tokens: "".to_string(),
        },
    )?;

    VALIDATOR_SLASHED.save(
        deps.storage,
        &ValidatorSlashed {
            moniker: "".to_string(),
            validator_address: "".to_string(),
            slashed_amount: "".to_string(),
        },
    )?;

    LAST_DELEGATION_CHANGE.save(
        deps.storage,
        &Delegation {
            validator_address: "".to_string(),
            delegator_address: "".to_string(),
            shares: "".to_string(),
        },
    )?;

    Ok(Response::new().add_attribute("method", "instantiate"))
}

fn increment(deps: DepsMut) -> Result<(), ContractError> {
    let mut config = CONFIG.load(deps.storage)?;
    for _ in 0..500 {   
        config.calls += 1;
        CONFIG.save(deps.storage, &config)?;
    }
    Ok(())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Increment {} => {
            increment(deps)?;
            Ok(Response::new())
        }
    }
}

fn last_validator(
    deps: DepsMut,
    moniker: String,
    validator_address: String,
    commission: String,
    validator_tokens: String,
    bonded_tokens: String,
    bond_status: String,
) -> Result<Response, ContractError> {
    LAST_VALIDATOR.save(
        deps.storage,
        &Validator {
            validator_address,
            moniker,
            commission,
            bond_status,
            bonded_tokens,
            validator_tokens,
        },
    )?;
    increment(deps)?;
    Ok(Response::new())
}

fn last_delegation(
    deps: DepsMut,
    validator_address: String,
    delegator_address: String,
    shares: String,
) -> Result<Response, ContractError> {
    LAST_DELEGATION_CHANGE.save(
        deps.storage,
        &Delegation {
            validator_address,
            delegator_address,
            shares,
        },
    )?;
    increment(deps)?;
    Ok(Response::new())
}

// sudo msg
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(deps: DepsMut, _env: Env, msg: SudoMsg) -> Result<Response, ContractError> {
    match msg {
        SudoMsg::AfterValidatorCreated {
            moniker,
            validator_address,
            commission,
            validator_tokens,
            bonded_tokens,
            bond_status,
        } => last_validator(
            deps,
            moniker,
            validator_address,
            commission,
            validator_tokens,
            bonded_tokens,
            bond_status,
        ),
        SudoMsg::AfterDelegationModified {
            validator_address,
            delegator_address,
            shares,
        } => last_delegation(deps, validator_address, delegator_address, shares),
        SudoMsg::BeforeValidatorSlashed {
            moniker,
            validator_address,
            slashed_amount,
        } => {
            VALIDATOR_SLASHED.save(
                deps.storage,
                &ValidatorSlashed {
                    moniker,
                    validator_address,
                    slashed_amount,
                },
            )?;
            Ok(Response::new())
        }
        SudoMsg::AfterValidatorRemoved {
            moniker,
            validator_address,
            commission,
            validator_tokens,
            bonded_tokens,
            bond_status,
        } => last_validator(
            deps,
            moniker,
            validator_address,
            commission,
            validator_tokens,
            bonded_tokens,
            bond_status,
        ),
        SudoMsg::BeforeValidatorModified {
            moniker,
            validator_address,
            commission,
            validator_tokens,
            bonded_tokens,
            bond_status,
        } => last_validator(
            deps,
            moniker,
            validator_address,
            commission,
            validator_tokens,
            bonded_tokens,
            bond_status,
        ),
        SudoMsg::AfterValidatorModified {
            moniker,
            validator_address,
            commission,
            validator_tokens,
            bonded_tokens,
            bond_status,
        } => last_validator(
            deps,
            moniker,
            validator_address,
            commission,
            validator_tokens,
            bonded_tokens,
            bond_status,
        ),
        SudoMsg::AfterValidatorBonded {
            moniker,
            validator_address,
            commission,
            validator_tokens,
            bonded_tokens,
            bond_status,
        } => last_validator(
            deps,
            moniker,
            validator_address,
            commission,
            validator_tokens,
            bonded_tokens,
            bond_status,
        ),
        SudoMsg::AfterValidatorBeginUnbonding {
            moniker,
            validator_address,
            commission,
            validator_tokens,
            bonded_tokens,
            bond_status,            
        } => last_validator(
            deps,
            moniker,
            validator_address,
            commission,
            validator_tokens,
            bonded_tokens,
            bond_status,
        ),
        SudoMsg::BeforeDelegationCreated {
            validator_address,
            delegator_address,
            shares,
        } => last_delegation(deps, validator_address, delegator_address, shares),
        SudoMsg::BeforeDelegationSharesModified {
            validator_address,
            delegator_address,
            shares,
        } => last_delegation(deps, validator_address, delegator_address, shares),
        SudoMsg::BeforeDelegationRemoved {
            validator_address,
            delegator_address,
            shares,
        } => last_delegation(deps, validator_address, delegator_address, shares),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetConfig {} => {
            let count = CONFIG.load(deps.storage)?.calls;
            to_binary(&Config { calls: count })
        }
        QueryMsg::LastValChange {} => {
            let val: Validator = LAST_VALIDATOR.load(deps.storage)?;
            to_binary(&val)
        }

        QueryMsg::LastValidatorSlash {} => {
            let val: ValidatorSlashed = VALIDATOR_SLASHED.load(deps.storage)?;
            to_binary(&val)
        }
        QueryMsg::LastDelegationChange {} => {
            let val: Delegation = LAST_DELEGATION_CHANGE.load(deps.storage)?;
            to_binary(&val)
        }
    }
}
