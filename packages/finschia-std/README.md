# finschia-std

Finschia's proto-generated types and helpers for interacting with the appchain. Compatible with CosmWasm contract.

## CosmWasm stargate message and stargate query

You can find all types and querier generated from finschia's protobuf in their respective module in `finschia_std`.

### Publishing Finschia's message from CosmWasm Contract

```rust
use cosmwasm_std::{CosmosMsg, Response, Env};
use finschia_std::types::lbm::collection::v1::MsgIssueNft;

# type ContractError = cosmwasm_std::StdError;
// ..

pub fn try_issue_nft(
    deps: DepsMut,
    name: String,
    meta: String,
    owner: String,
) -> Result<Response, ContractError> {
    let contract_id = CONTRACT_ID.load(deps.storage)?;

    // construct message and convert them into cosmos message
    // (notice `CosmosMsg` type and `.into()`)
    let msg_issue_nft: CosmosMsg = MsgIssueNft {
        contract_id,
        name,
        meta,
        owner,
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "try_issue_nft")
        .add_submessage(SubMsg::reply_on_success(msg_issue_nft, ISSUE_NFT_REPLY_ID)))
}
```

## Querying Finschia's module

Each module has its own querier that derived from protobuf service definition that can be found [here](https://github.com/Finschia/finschia-sdk/tree/v0.48.0-rc1/proto/lbm).

```rust
use cosmwasm_std::{Deps, Env, StdResult};
use finschia_std::types::finschia::collection::v1::{CollectionQuerier, QueryNftSupplyResponse};

// ..

fn query_nft_supply(deps: Deps, token_type: String) -> StdResult<QueryNftSupplyResponse> {
    let contract_id = CONTRACT_ID.load(deps.storage)?;

    // create `CollectionQuerier`
    let cq = CollectionQuerier::new(&deps.querier);

    // `CollectionQuerier` has all the fns for querying the module
    let res = cq.nft_supply(contract_id, token_type)?;
    Ok(QueryNftSupplyResponse { supply: res.supply })
}
```
