#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, SudoMsg};
use crate::state::{Config, CONFIG, LAST_CREATED_VALIDATOR, LastCreatedValidator};

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

    CONFIG.save(deps.storage, &Config { val: 0 })?;
    LAST_CREATED_VALIDATOR.save(deps.storage, &LastCreatedValidator {
        val_addr: "".to_string(),
        moniker: "".to_string(),
        commission: "".to_string(),
    })?;

    Ok(Response::new().add_attribute("method", "instantiate"))
}

fn increment(deps: DepsMut) -> Result<(), ContractError> {
    let mut config = CONFIG.load(deps.storage)?;
    config.val += 1;
    CONFIG.save(deps.storage, &config)?;
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

// sudo msg
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(deps: DepsMut, _env: Env, msg: SudoMsg) -> Result<Response, ContractError> {
    match msg {
        SudoMsg::AfterValidatorCreated {validator_address, moniker, commission} => {            
            LAST_CREATED_VALIDATOR.save(deps.storage, &LastCreatedValidator {
                val_addr: validator_address,
                moniker: moniker,
                commission: commission,
            })?;

            increment(deps)?;
            Ok(Response::new())
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetConfig {} => to_binary(&query_config(deps)?),
        QueryMsg::GetLastValInfo {} => to_binary(&query_last_val(deps)?),
    }
}

fn query_config(deps: Deps) -> StdResult<Config> {
    let count = CONFIG.load(deps.storage)?.val;
    Ok(Config { val: count })
}

fn query_last_val(deps: Deps) -> StdResult<LastCreatedValidator> {
    let val: LastCreatedValidator = LAST_CREATED_VALIDATOR.load(deps.storage)?;
    Ok(LastCreatedValidator { 
        commission: val.commission,
        moniker: val.moniker,
        val_addr: val.val_addr,
     })
}
