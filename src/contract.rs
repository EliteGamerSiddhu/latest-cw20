#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_json_binary};
use cw2::set_contract_version;

use crate::msg::MigrateMsg;

use cw20_base::msg::{QueryMsg, ExecuteMsg, InstantiateMsg};
use cw20_base::allowances::*;
use cw20_base::enumerable::*;
use cw20_base::contract::*;
use cw20_base::ContractError;

const CONTRACT_NAME: &str = "crates.io:cw20_contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(instantiate(deps, env, info, msg)?)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg{
        ExecuteMsg::Burn { amount } => execute_burn(deps, env, info, amount),
        ExecuteMsg::BurnFrom { owner, amount } => execute_burn_from(deps, env, info, owner, amount),
        ExecuteMsg::DecreaseAllowance { spender, amount, expires } => execute_decrease_allowance(deps, env, info, spender, amount, expires),
        ExecuteMsg::IncreaseAllowance { spender, amount, expires } => execute_increase_allowance(deps, env, info, spender, amount, expires),
        ExecuteMsg::Mint { recipient, amount } => execute_mint(deps, env, info, recipient, amount),
        ExecuteMsg::Send { contract, amount, msg } => execute_send(deps, env, info, contract, amount, msg),
        ExecuteMsg::SendFrom { owner, contract, amount, msg } => execute_send_from(deps, env, info, owner, contract, amount, msg),
        ExecuteMsg::Transfer { recipient, amount } => execute_transfer(deps, env, info, recipient, amount),
        ExecuteMsg::TransferFrom { owner, recipient, amount } => execute_transfer_from(deps, env, info, owner, recipient, amount),
        ExecuteMsg::UpdateMarketing { project, description, marketing } => execute_update_marketing(deps, env, info, project, description, marketing),
        ExecuteMsg::UpdateMinter { new_minter } => execute_update_minter(deps, env, info, new_minter),
        ExecuteMsg::UploadLogo(logo) => execute_upload_logo(deps, env, info, logo),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::AllAccounts { start_after, limit } => to_json_binary(&query_all_accounts(deps, start_after, limit)?),
        QueryMsg::AllAllowances { owner, start_after, limit } => to_json_binary(&query_owner_allowances(deps, owner, start_after, limit)?),
        QueryMsg::AllSpenderAllowances { spender, start_after, limit } => to_json_binary(&query_spender_allowances(deps, spender, start_after, limit)?),
        QueryMsg::Allowance { owner, spender } => to_json_binary(&query_allowance(deps, owner, spender)?),
        QueryMsg::Balance { address } => to_json_binary(&query_balance(deps, address)?),
        QueryMsg::DownloadLogo {  } => to_json_binary(&query_download_logo(deps)?),
        QueryMsg::MarketingInfo {  } => to_json_binary(&query_marketing_info(deps)?),
        QueryMsg::Minter {  } => to_json_binary(&query_minter(deps)?),
        QueryMsg::TokenInfo {  } => to_json_binary(&query_token_info(deps)?),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}