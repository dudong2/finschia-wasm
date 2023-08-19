use finschia_std_derive::CosmwasmExt;
/// InactiveAddr models the blocked address for the bankplus module
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
#[proto_message(type_url = "/lbm.bankplus.v1.InactiveAddr")]
pub struct InactiveAddr {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
