use finschia_std_derive::CosmwasmExt;
/// EventDeactivateContractProposal is the event that is emitted when the
/// contract is deactivate.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/lbm.wasm.v1.")]
pub struct EventDeactivateContractProposal {
    /// contract is the smart contract's address
    #[prost(string, tag = "1")]
    pub contract: ::prost::alloc::string::String,
}
/// EventActivateContractProposal is the event that is emitted when the contract
/// is activates.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/lbm.wasm.v1.")]
pub struct EventActivateContractProposal {
    /// contract is the smart contract's address
    #[prost(string, tag = "1")]
    pub contract: ::prost::alloc::string::String,
}
/// GenesisState - genesis state of x/wasm
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/lbm.wasm.v1.")]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<super::super::super::cosmwasm::wasm::v1::Params>,
    #[prost(message, repeated, tag = "2")]
    pub codes: ::prost::alloc::vec::Vec<super::super::super::cosmwasm::wasm::v1::Code>,
    #[prost(message, repeated, tag = "3")]
    pub contracts: ::prost::alloc::vec::Vec<super::super::super::cosmwasm::wasm::v1::Contract>,
    #[prost(message, repeated, tag = "4")]
    pub sequences: ::prost::alloc::vec::Vec<super::super::super::cosmwasm::wasm::v1::Sequence>,
    #[prost(message, repeated, tag = "5")]
    pub gen_msgs:
        ::prost::alloc::vec::Vec<super::super::super::cosmwasm::wasm::v1::genesis_state::GenMsgs>,
    /// InactiveContractAddresses is a list of contract address that set inactive
    #[prost(string, repeated, tag = "6")]
    pub inactive_contract_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// DeactivateContractProposal gov proposal content type adds a contract to
/// inactive list.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/lbm.wasm.v1.")]
pub struct DeactivateContractProposal {
    /// Title is a short summary
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// Description is a human readable text
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Contract is the smart contract address to deactivate
    #[prost(string, tag = "3")]
    pub contract: ::prost::alloc::string::String,
}
/// ActivateContractProposal gov proposal content type deletes a contract from
/// inactive list.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/lbm.wasm.v1.")]
pub struct ActivateContractProposal {
    /// Title is a short summary
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// Description is a human readable text
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Contract is the smart contract address to activate
    #[prost(string, tag = "3")]
    pub contract: ::prost::alloc::string::String,
}
/// QueryInactiveContractsRequest is the request type for Query/InactiveContract
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/lbm.wasm.v1.")]
pub struct QueryInactiveContractsRequest {
    /// pagination defines an optional pagination for the request
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryInactiveContractsResponse is the response type for the
/// Query/InactiveContract RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/lbm.wasm.v1.")]
pub struct QueryInactiveContractsResponse {
    /// addresses is the inactive address list
    #[prost(string, repeated, tag = "1")]
    pub addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// pagination defines the pagination in the response
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryIsInactiveContractRequest is the request type for
/// Query/IsInactiveContract RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/lbm.wasm.v1.")]
pub struct QueryInactiveContractRequest {
    /// address is the address of the contract
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// QueryInactiveContractsResponse is the response type for the
/// Query/IsInactiveContract RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/lbm.wasm.v1.")]
pub struct QueryInactiveContractResponse {
    /// inactivated is the result if the contract is inactive contract or not
    #[prost(bool, tag = "1")]
    pub inactivated: bool,
}
/// MsgStoreCodeAndInstantiateContract submit Wasm code to the system and
/// instantiate a contract using it.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/lbm.wasm.v1.")]
pub struct MsgStoreCodeAndInstantiateContract {
    /// Sender is the that actor that signed the messages
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// WASMByteCode can be raw or gzip compressed
    #[prost(bytes = "vec", tag = "2")]
    pub wasm_byte_code: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "5")]
    pub instantiate_permission:
        ::core::option::Option<super::super::super::cosmwasm::wasm::v1::AccessConfig>,
    /// Admin is an optional address that can execute migrations
    #[prost(string, tag = "6")]
    pub admin: ::prost::alloc::string::String,
    /// Label is optional metadata to be stored with a contract instance.
    #[prost(string, tag = "7")]
    pub label: ::prost::alloc::string::String,
    /// Msg json encoded message to be passed to the contract on instantiation
    #[prost(bytes = "vec", tag = "8")]
    pub msg: ::prost::alloc::vec::Vec<u8>,
    /// Funds coins that are transferred to the contract on instantiation
    #[prost(message, repeated, tag = "9")]
    pub funds: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgStoreCodeAndInstantiateContractResponse returns store and instantiate
/// result data.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/lbm.wasm.v1.")]
pub struct MsgStoreCodeAndInstantiateContractResponse {
    /// CodeID is the reference to the stored WASM code
    #[prost(uint64, tag = "1")]
    #[serde(alias = "codeID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub code_id: u64,
    /// Address is the bech32 address of the new contract instance
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
    /// Data contains base64-encoded bytes to returned from the contract
    #[prost(bytes = "vec", tag = "3")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
