use finschia_std_derive::CosmwasmExt;
/// CreateValidatorAuthorization allows the grantee to create a new validator.
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
#[proto_message(type_url = "/lbm.stakingplus.v1.CreateValidatorAuthorization")]
pub struct CreateValidatorAuthorization {
    /// redundant, but good for the query.
    #[prost(string, tag = "1")]
    pub validator_address: ::prost::alloc::string::String,
}
