use finschia_std_derive::CosmwasmExt;
/// Params defines the parameters for the token module.
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
#[proto_message(type_url = "/lbm.token.v1.Params")]
pub struct Params {}
/// Contract defines token information.
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
#[proto_message(type_url = "/lbm.token.v1.Contract")]
pub struct Contract {
    /// id defines the unique identifier of the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "ID")]
    pub id: ::prost::alloc::string::String,
    /// name defines the human-readable name of the contract. mandatory (not ERC20 compliant).
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// symbol is an abbreviated name for contract. mandatory (not ERC20 compliant).
    #[prost(string, tag = "3")]
    pub symbol: ::prost::alloc::string::String,
    /// an uri for the image of the contract stored off chain.
    #[prost(string, tag = "4")]
    pub uri: ::prost::alloc::string::String,
    /// meta is a brief description of contract.
    #[prost(string, tag = "5")]
    pub meta: ::prost::alloc::string::String,
    /// decimals is the number of decimals which one must divide the amount by to get its user representation.
    #[prost(int32, tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub decimals: i32,
    /// mintable represents whether the token is allowed to mint or burn.
    #[prost(bool, tag = "7")]
    pub mintable: bool,
}
/// Attribute defines a key and value of the attribute.
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
#[proto_message(type_url = "/lbm.token.v1.Attribute")]
pub struct Attribute {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
/// Authorization defines an authorization given to the operator on tokens of the holder.
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
#[proto_message(type_url = "/lbm.token.v1.Authorization")]
pub struct Authorization {
    /// address of the token holder which approves the authorization.
    #[prost(string, tag = "1")]
    pub holder: ::prost::alloc::string::String,
    /// address of the operator which the authorization is granted to.
    #[prost(string, tag = "2")]
    pub operator: ::prost::alloc::string::String,
}
/// Grant defines permission given to a grantee.
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
#[proto_message(type_url = "/lbm.token.v1.Grant")]
pub struct Grant {
    /// address of the grantee.
    #[prost(string, tag = "1")]
    pub grantee: ::prost::alloc::string::String,
    /// permission on the contract.
    #[prost(enumeration = "Permission", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub permission: i32,
}
/// Permission enumerates the valid permissions on a contract.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum Permission {
    /// unspecified defines the default permission which is invalid.
    Unspecified = 0,
    /// PERMISSION_MODIFY defines a permission to modify a contract.
    Modify = 1,
    /// PERMISSION_MINT defines a permission to mint tokens of a contract.
    Mint = 2,
    /// PERMISSION_BURN defines a permission to burn tokens of a contract.
    Burn = 3,
}
impl Permission {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Permission::Unspecified => "PERMISSION_UNSPECIFIED",
            Permission::Modify => "PERMISSION_MODIFY",
            Permission::Mint => "PERMISSION_MINT",
            Permission::Burn => "PERMISSION_BURN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PERMISSION_UNSPECIFIED" => Some(Self::Unspecified),
            "PERMISSION_MODIFY" => Some(Self::Modify),
            "PERMISSION_MINT" => Some(Self::Mint),
            "PERMISSION_BURN" => Some(Self::Burn),
            _ => None,
        }
    }
}
/// Deprecated: use Permission
///
/// LegacyPermission enumerates the valid permissions on a contract.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum LegacyPermission {
    /// unspecified defines the default permission which is invalid.
    Unspecified = 0,
    /// modify defines a permission to modify a contract.
    Modify = 1,
    /// mint defines a permission to mint tokens of a contract.
    Mint = 2,
    /// burn defines a permission to burn tokens of a contract.
    Burn = 3,
}
impl LegacyPermission {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LegacyPermission::Unspecified => "LEGACY_PERMISSION_UNSPECIFIED",
            LegacyPermission::Modify => "LEGACY_PERMISSION_MODIFY",
            LegacyPermission::Mint => "LEGACY_PERMISSION_MINT",
            LegacyPermission::Burn => "LEGACY_PERMISSION_BURN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LEGACY_PERMISSION_UNSPECIFIED" => Some(Self::Unspecified),
            "LEGACY_PERMISSION_MODIFY" => Some(Self::Modify),
            "LEGACY_PERMISSION_MINT" => Some(Self::Mint),
            "LEGACY_PERMISSION_BURN" => Some(Self::Burn),
            _ => None,
        }
    }
}
/// EventSent is emitted when tokens are transferred.
///
/// Since: 0.46.0 (finschia)
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
#[proto_message(type_url = "/lbm.token.v1.EventSent")]
pub struct EventSent {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// address which triggered the send.
    #[prost(string, tag = "2")]
    pub operator: ::prost::alloc::string::String,
    /// holder whose tokens were sent.
    #[prost(string, tag = "3")]
    pub from: ::prost::alloc::string::String,
    /// recipient of the tokens
    #[prost(string, tag = "4")]
    pub to: ::prost::alloc::string::String,
    /// number of tokens sent.
    #[prost(string, tag = "5")]
    pub amount: ::prost::alloc::string::String,
}
/// EventAuthorizedOperator is emitted when a holder authorizes an operator to manipulate its tokens.
///
/// Since: 0.46.0 (finschia)
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
#[proto_message(type_url = "/lbm.token.v1.EventAuthorizedOperator")]
pub struct EventAuthorizedOperator {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// address of a holder which authorized the `operator` address as an operator.
    #[prost(string, tag = "2")]
    pub holder: ::prost::alloc::string::String,
    /// address which became an operator of `holder`.
    #[prost(string, tag = "3")]
    pub operator: ::prost::alloc::string::String,
}
/// EventRevokedOperator is emitted when an authorization is revoked.
///
/// Since: 0.46.0 (finschia)
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
#[proto_message(type_url = "/lbm.token.v1.EventRevokedOperator")]
pub struct EventRevokedOperator {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// address of a holder which revoked the `operator` address as an operator.
    #[prost(string, tag = "2")]
    pub holder: ::prost::alloc::string::String,
    /// address which was revoked as an operator of `holder`.
    #[prost(string, tag = "3")]
    pub operator: ::prost::alloc::string::String,
}
/// EventIssued is emitted when a new contract is created.
///
/// Since: 0.46.0 (finschia)
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
#[proto_message(type_url = "/lbm.token.v1.EventIssued")]
pub struct EventIssued {
    /// address which created the contract.
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    /// contract id associated with the contract.
    #[prost(string, tag = "2")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// name defines the human-readable name of the contract.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// symbol is an abbreviated name for contract.
    #[prost(string, tag = "4")]
    pub symbol: ::prost::alloc::string::String,
    /// uri is an uri for the resource of the contract stored off chain.
    #[prost(string, tag = "5")]
    pub uri: ::prost::alloc::string::String,
    /// meta is a brief description of contract.
    #[prost(string, tag = "6")]
    pub meta: ::prost::alloc::string::String,
    /// decimals is the number of decimals which one must divide the amount by to get its user representation.
    #[prost(int32, tag = "7")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub decimals: i32,
    /// mintable represents whether the token is allowed to mint.
    #[prost(bool, tag = "8")]
    pub mintable: bool,
}
/// EventGranted is emitted when a granter grants its permission to a grantee.
///
/// Info: `granter` would be empty if the permission is granted by an issuance.
///
/// Since: 0.46.0 (finschia)
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
#[proto_message(type_url = "/lbm.token.v1.EventGranted")]
pub struct EventGranted {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// address which granted the permission to `grantee`.
    /// it would be empty where the event is triggered by the issuance.
    #[prost(string, tag = "2")]
    pub granter: ::prost::alloc::string::String,
    /// address of the grantee.
    #[prost(string, tag = "3")]
    pub grantee: ::prost::alloc::string::String,
    /// permission on the contract.
    #[prost(enumeration = "Permission", tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub permission: i32,
}
/// EventRenounced is emitted when a grantee renounces its permission.
///
/// Since: 0.46.0 (finschia)
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
#[proto_message(type_url = "/lbm.token.v1.EventRenounced")]
pub struct EventRenounced {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// address of the grantee which abandons its grant.
    #[prost(string, tag = "2")]
    pub grantee: ::prost::alloc::string::String,
    /// permission on the contract.
    #[prost(enumeration = "Permission", tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub permission: i32,
}
/// EventMinted is emitted when tokens are minted.
///
/// Since: 0.46.0 (finschia)
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
#[proto_message(type_url = "/lbm.token.v1.EventMinted")]
pub struct EventMinted {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// address which triggered the mint.
    #[prost(string, tag = "2")]
    pub operator: ::prost::alloc::string::String,
    /// recipient of the tokens.
    #[prost(string, tag = "3")]
    pub to: ::prost::alloc::string::String,
    /// number of tokens minted.
    #[prost(string, tag = "4")]
    pub amount: ::prost::alloc::string::String,
}
/// EventBurned is emitted when tokens are burnt.
///
/// Since: 0.46.0 (finschia)
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
#[proto_message(type_url = "/lbm.token.v1.EventBurned")]
pub struct EventBurned {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// address which triggered the burn.
    #[prost(string, tag = "2")]
    pub operator: ::prost::alloc::string::String,
    /// holder whose tokens were burned.
    #[prost(string, tag = "3")]
    pub from: ::prost::alloc::string::String,
    /// number of tokens burned.
    #[prost(string, tag = "4")]
    pub amount: ::prost::alloc::string::String,
}
/// EventModified is emitted when the information of a contract is modified.
///
/// Since: 0.46.0 (finschia)
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
#[proto_message(type_url = "/lbm.token.v1.EventModified")]
pub struct EventModified {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// address which triggered the modify.
    #[prost(string, tag = "2")]
    pub operator: ::prost::alloc::string::String,
    /// changes on the metadata of the class.
    /// possible attribute keys are same as those of MsgModify.
    /// deprecated "img_uri" has been replaced by "uri" in the events.
    #[prost(message, repeated, tag = "3")]
    pub changes: ::prost::alloc::vec::Vec<Attribute>,
}
/// AttributeKey enumerates the valid attribute keys on x/token.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum AttributeKey {
    Unspecified = 0,
    Name = 1,
    Meta = 3,
    /// deprecated: use ATTRIBUTE_KEY_URI
    ImgUri = 8,
    Uri = 15,
}
impl AttributeKey {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AttributeKey::Unspecified => "ATTRIBUTE_KEY_UNSPECIFIED",
            AttributeKey::Name => "ATTRIBUTE_KEY_NAME",
            AttributeKey::Meta => "ATTRIBUTE_KEY_META",
            AttributeKey::ImgUri => "ATTRIBUTE_KEY_IMG_URI",
            AttributeKey::Uri => "ATTRIBUTE_KEY_URI",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ATTRIBUTE_KEY_UNSPECIFIED" => Some(Self::Unspecified),
            "ATTRIBUTE_KEY_NAME" => Some(Self::Name),
            "ATTRIBUTE_KEY_META" => Some(Self::Meta),
            "ATTRIBUTE_KEY_IMG_URI" => Some(Self::ImgUri),
            "ATTRIBUTE_KEY_URI" => Some(Self::Uri),
            _ => None,
        }
    }
}
/// GenesisState defines the token module's genesis state.
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
#[proto_message(type_url = "/lbm.token.v1.GenesisState")]
pub struct GenesisState {
    /// params defines all the paramaters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// class_state is the class keeper's genesis state.
    #[prost(message, optional, tag = "2")]
    pub class_state: ::core::option::Option<ClassGenesisState>,
    /// balances is an array containing the balances of all the accounts.
    #[prost(message, repeated, tag = "3")]
    pub balances: ::prost::alloc::vec::Vec<ContractBalances>,
    /// classes defines the metadata of the differents tokens.
    #[prost(message, repeated, tag = "4")]
    pub classes: ::prost::alloc::vec::Vec<Contract>,
    /// grants defines the grant information.
    #[prost(message, repeated, tag = "5")]
    pub grants: ::prost::alloc::vec::Vec<ContractGrants>,
    /// authorizations defines the approve information.
    #[prost(message, repeated, tag = "6")]
    pub authorizations: ::prost::alloc::vec::Vec<ContractAuthorizations>,
    /// supplies represents the total supplies of tokens.
    #[prost(message, repeated, tag = "7")]
    pub supplies: ::prost::alloc::vec::Vec<ContractCoin>,
    /// mints represents the total mints of tokens.
    #[prost(message, repeated, tag = "8")]
    pub mints: ::prost::alloc::vec::Vec<ContractCoin>,
    /// burns represents the total burns of tokens.
    #[prost(message, repeated, tag = "9")]
    pub burns: ::prost::alloc::vec::Vec<ContractCoin>,
}
/// ClassGenesisState defines the classs keeper's genesis state.
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
#[proto_message(type_url = "/lbm.token.v1.ClassGenesisState")]
pub struct ClassGenesisState {
    /// nonce is the next class nonce to issue.
    #[prost(string, tag = "1")]
    pub nonce: ::prost::alloc::string::String,
    /// ids represents the issued ids.
    #[prost(string, repeated, tag = "2")]
    pub ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// ContractBalances defines balances belong to a contract.
/// genesis state.
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
#[proto_message(type_url = "/lbm.token.v1.ContractBalances")]
pub struct ContractBalances {
    /// contract id associated with the token class.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// balances of the contract.
    #[prost(message, repeated, tag = "2")]
    pub balances: ::prost::alloc::vec::Vec<Balance>,
}
/// Balance defines a balance of an address.
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
#[proto_message(type_url = "/lbm.token.v1.Balance")]
pub struct Balance {
    /// address of the holder.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// amount of the balance.
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
}
/// ContractAuthorizations defines authorizations belong to a contract.
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
#[proto_message(type_url = "/lbm.token.v1.ContractAuthorizations")]
pub struct ContractAuthorizations {
    /// contract id associated with the token class.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// authorizations of the contract.
    #[prost(message, repeated, tag = "2")]
    pub authorizations: ::prost::alloc::vec::Vec<Authorization>,
}
/// ContractGrant defines grants belong to a contract.
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
#[proto_message(type_url = "/lbm.token.v1.ContractGrants")]
pub struct ContractGrants {
    /// contract id associated with the token class.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// grants of the contract.
    #[prost(message, repeated, tag = "2")]
    pub grants: ::prost::alloc::vec::Vec<Grant>,
}
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
#[proto_message(type_url = "/lbm.token.v1.ContractCoin")]
pub struct ContractCoin {
    /// contract id associated with the token class.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// amount of the token.
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
}
/// QueryBalanceRequest is the request type for the Query/Balance RPC method
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
#[proto_message(type_url = "/lbm.token.v1.QueryBalanceRequest")]
#[proto_query(
    path = "/lbm.token.v1.Query/Balance",
    response_type = QueryBalanceResponse
)]
pub struct QueryBalanceRequest {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// address is the address to query balance for.
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
}
/// QueryBalanceResponse is the response type for the Query/Balance RPC method
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
#[proto_message(type_url = "/lbm.token.v1.QueryBalanceResponse")]
pub struct QueryBalanceResponse {
    /// the balance of the tokens.
    #[prost(string, tag = "1")]
    pub amount: ::prost::alloc::string::String,
}
/// QuerySupplyRequest is the request type for the Query/Supply RPC method
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
#[proto_message(type_url = "/lbm.token.v1.QuerySupplyRequest")]
#[proto_query(path = "/lbm.token.v1.Query/Supply", response_type = QuerySupplyResponse)]
pub struct QuerySupplyRequest {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
}
/// QuerySupplyResponse is the response type for the Query/Supply RPC method
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
#[proto_message(type_url = "/lbm.token.v1.QuerySupplyResponse")]
pub struct QuerySupplyResponse {
    /// the supply of the tokens.
    #[prost(string, tag = "1")]
    pub amount: ::prost::alloc::string::String,
}
/// QueryMintedRequest is the request type for the Query/Minted RPC method
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
#[proto_message(type_url = "/lbm.token.v1.QueryMintedRequest")]
#[proto_query(path = "/lbm.token.v1.Query/Minted", response_type = QueryMintedResponse)]
pub struct QueryMintedRequest {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
}
/// QueryMintedResponse is the response type for the Query/Minted RPC method
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
#[proto_message(type_url = "/lbm.token.v1.QueryMintedResponse")]
pub struct QueryMintedResponse {
    /// the amount of the minted tokens.
    #[prost(string, tag = "1")]
    pub amount: ::prost::alloc::string::String,
}
/// QueryBurntRequest is the request type for the Query/Burnt RPC method
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
#[proto_message(type_url = "/lbm.token.v1.QueryBurntRequest")]
#[proto_query(path = "/lbm.token.v1.Query/Burnt", response_type = QueryBurntResponse)]
pub struct QueryBurntRequest {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
}
/// QueryBurntResponse is the response type for the Query/Burnt RPC method
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
#[proto_message(type_url = "/lbm.token.v1.QueryBurntResponse")]
pub struct QueryBurntResponse {
    /// the amount of the burnt tokens.
    #[prost(string, tag = "1")]
    pub amount: ::prost::alloc::string::String,
}
/// QueryContractRequest is the request type for the Query/Contract RPC method
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
#[proto_message(type_url = "/lbm.token.v1.QueryContractRequest")]
#[proto_query(
    path = "/lbm.token.v1.Query/Contract",
    response_type = QueryContractResponse
)]
pub struct QueryContractRequest {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
}
/// QueryContractResponse is the response type for the Query/Contract RPC method
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
#[proto_message(type_url = "/lbm.token.v1.QueryContractResponse")]
pub struct QueryContractResponse {
    #[prost(message, optional, tag = "1")]
    pub contract: ::core::option::Option<Contract>,
}
/// QueryGranteeGrantsRequest is the request type for the Query/GranteeGrants RPC method
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
#[proto_message(type_url = "/lbm.token.v1.QueryGranteeGrantsRequest")]
#[proto_query(
    path = "/lbm.token.v1.Query/GranteeGrants",
    response_type = QueryGranteeGrantsResponse
)]
pub struct QueryGranteeGrantsRequest {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// grantee which has permissions on the contract.
    #[prost(string, tag = "2")]
    pub grantee: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "3")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryGranteeGrantsResponse is the response type for the Query/GranteeGrants RPC method
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
#[proto_message(type_url = "/lbm.token.v1.QueryGranteeGrantsResponse")]
pub struct QueryGranteeGrantsResponse {
    /// all the grants on the grantee.
    #[prost(message, repeated, tag = "1")]
    pub grants: ::prost::alloc::vec::Vec<Grant>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryIsOperatorForRequest is the request type for the Query/IsOperatorFor RPC method
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
#[proto_message(type_url = "/lbm.token.v1.QueryIsOperatorForRequest")]
#[proto_query(
    path = "/lbm.token.v1.Query/IsOperatorFor",
    response_type = QueryIsOperatorForResponse
)]
pub struct QueryIsOperatorForRequest {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// address of the operator which the authorization is granted to.
    #[prost(string, tag = "2")]
    pub operator: ::prost::alloc::string::String,
    /// address of the holder of the authorization.
    #[prost(string, tag = "3")]
    pub holder: ::prost::alloc::string::String,
}
/// QueryIsOperatorForResponse is the response type for the Query/IsOperatorFor RPC method
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
#[proto_message(type_url = "/lbm.token.v1.QueryIsOperatorForResponse")]
pub struct QueryIsOperatorForResponse {
    #[prost(bool, tag = "1")]
    pub authorized: bool,
}
/// QueryHoldersByOperatorRequest is the request type for the Query/HoldersByOperator RPC method
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
#[proto_message(type_url = "/lbm.token.v1.QueryHoldersByOperatorRequest")]
#[proto_query(
    path = "/lbm.token.v1.Query/HoldersByOperator",
    response_type = QueryHoldersByOperatorResponse
)]
pub struct QueryHoldersByOperatorRequest {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// address of the operator which the authorization is granted to.
    #[prost(string, tag = "2")]
    pub operator: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "3")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryHoldersByOperatorResponse is the response type for the Query/HoldersByOperator RPC method
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
#[proto_message(type_url = "/lbm.token.v1.QueryHoldersByOperatorResponse")]
pub struct QueryHoldersByOperatorResponse {
    /// holder addresses
    #[prost(string, repeated, tag = "1")]
    pub holders: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// MsgSend defines the Msg/Send request type.
///
/// Signer: `from`
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
#[proto_message(type_url = "/lbm.token.v1.MsgSend")]
pub struct MsgSend {
    /// contract id associated with the token class.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// holder whose tokens are being sent.
    #[prost(string, tag = "2")]
    pub from: ::prost::alloc::string::String,
    /// recipient of the tokens.
    #[prost(string, tag = "3")]
    pub to: ::prost::alloc::string::String,
    /// number of tokens to send.
    #[prost(string, tag = "4")]
    pub amount: ::prost::alloc::string::String,
}
/// MsgSendResponse defines the Msg/Send response type.
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
#[proto_message(type_url = "/lbm.token.v1.MsgSendResponse")]
pub struct MsgSendResponse {}
/// MsgOperatorSend defines the Msg/OperatorSend request type.
///
/// Signer: `operator`
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
#[proto_message(type_url = "/lbm.token.v1.MsgOperatorSend")]
pub struct MsgOperatorSend {
    /// contract id associated with the token class.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// the address of the operator.
    #[prost(string, tag = "2")]
    pub operator: ::prost::alloc::string::String,
    /// the address which the transfer is from.
    #[prost(string, tag = "3")]
    pub from: ::prost::alloc::string::String,
    /// the address which the transfer is to.
    #[prost(string, tag = "4")]
    pub to: ::prost::alloc::string::String,
    /// the amount of the transfer.
    #[prost(string, tag = "5")]
    pub amount: ::prost::alloc::string::String,
}
/// MsgOperatorSendResponse defines the Msg/OperatorSend response type.
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
#[proto_message(type_url = "/lbm.token.v1.MsgOperatorSendResponse")]
pub struct MsgOperatorSendResponse {}
/// MsgRevokeOperator defines the Msg/RevokeOperator request type.
///
/// Signer: `holder`
///
/// Since: 0.46.0 (finschia)
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
#[proto_message(type_url = "/lbm.token.v1.MsgRevokeOperator")]
pub struct MsgRevokeOperator {
    /// contract id associated with the token class.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// address of a holder which revokes the `operator` address as an operator.
    #[prost(string, tag = "2")]
    pub holder: ::prost::alloc::string::String,
    /// address to rescind as an operator for `holder`.
    #[prost(string, tag = "3")]
    pub operator: ::prost::alloc::string::String,
}
/// MsgRevokeOperatorResponse defines the Msg/RevokeOperator response type.
///
/// Since: 0.46.0 (finschia)
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
#[proto_message(type_url = "/lbm.token.v1.MsgRevokeOperatorResponse")]
pub struct MsgRevokeOperatorResponse {}
/// MsgAuthorizeOperator defines the Msg/AuthorizeOperator request type.
///
/// Signer: `holder`
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
#[proto_message(type_url = "/lbm.token.v1.MsgAuthorizeOperator")]
pub struct MsgAuthorizeOperator {
    /// contract id associated with the token class.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// address of the token holder which approves the authorization.
    #[prost(string, tag = "2")]
    pub holder: ::prost::alloc::string::String,
    /// address of the operator which the authorization is granted to.
    #[prost(string, tag = "3")]
    pub operator: ::prost::alloc::string::String,
}
/// MsgAuthorizeOperatorResponse defines the Msg/AuthorizeOperator response type.
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
#[proto_message(type_url = "/lbm.token.v1.MsgAuthorizeOperatorResponse")]
pub struct MsgAuthorizeOperatorResponse {}
/// MsgIssue defines the Msg/Issue request type.
///
/// Signer: `owner`
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
#[proto_message(type_url = "/lbm.token.v1.MsgIssue")]
pub struct MsgIssue {
    /// name defines the human-readable name of the token class. mandatory (not ERC20 compliant).
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// symbol is an abbreviated name for token class. mandatory (not ERC20 compliant).
    #[prost(string, tag = "2")]
    pub symbol: ::prost::alloc::string::String,
    /// uri for the image of the token class stored off chain.
    #[prost(string, tag = "3")]
    pub uri: ::prost::alloc::string::String,
    /// meta is a brief description of token class.
    #[prost(string, tag = "4")]
    pub meta: ::prost::alloc::string::String,
    /// decimals is the number of decimals which one must divide the amount by to get its user representation.
    #[prost(int32, tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub decimals: i32,
    /// mintable represents whether the token is allowed to mint.
    #[prost(bool, tag = "6")]
    pub mintable: bool,
    /// the address which all permissions on the token class will be granted to (not a permanent property).
    #[prost(string, tag = "7")]
    pub owner: ::prost::alloc::string::String,
    /// the address to send the minted token to. mandatory.
    #[prost(string, tag = "8")]
    pub to: ::prost::alloc::string::String,
    /// amount of tokens to mint on issuance. mandatory.
    #[prost(string, tag = "9")]
    pub amount: ::prost::alloc::string::String,
}
/// MsgIssueResponse defines the Msg/Issue response type.
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
#[proto_message(type_url = "/lbm.token.v1.MsgIssueResponse")]
pub struct MsgIssueResponse {
    /// id of the new contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
}
/// MsgGrantPermission defines the Msg/GrantPermission request type.
///
/// Signer: `granter`
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
#[proto_message(type_url = "/lbm.token.v1.MsgGrantPermission")]
pub struct MsgGrantPermission {
    /// contract id associated with the token class.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// address of the granter which must have the permission to give.
    #[prost(string, tag = "2")]
    pub from: ::prost::alloc::string::String,
    /// address of the grantee.
    #[prost(string, tag = "3")]
    pub to: ::prost::alloc::string::String,
    /// permission on the token class.
    #[prost(string, tag = "4")]
    pub permission: ::prost::alloc::string::String,
}
/// MsgGrantPermissionResponse defines the Msg/GrantPermission response type.
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
#[proto_message(type_url = "/lbm.token.v1.MsgGrantPermissionResponse")]
pub struct MsgGrantPermissionResponse {}
/// MsgRevokePermission defines the Msg/RevokePermission request type.
///
/// Signer: `grantee`
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
#[proto_message(type_url = "/lbm.token.v1.MsgRevokePermission")]
pub struct MsgRevokePermission {
    /// contract id associated with the token class.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// address of the grantee which abandons the permission.
    #[prost(string, tag = "2")]
    pub from: ::prost::alloc::string::String,
    /// permission on the token class.
    #[prost(string, tag = "3")]
    pub permission: ::prost::alloc::string::String,
}
/// MsgRevokePermissionResponse defines the Msg/RevokePermission response type.
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
#[proto_message(type_url = "/lbm.token.v1.MsgRevokePermissionResponse")]
pub struct MsgRevokePermissionResponse {}
/// MsgMint defines the Msg/Mint request type.
///
/// Signer: `from`
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
#[proto_message(type_url = "/lbm.token.v1.MsgMint")]
pub struct MsgMint {
    /// contract id associated with the token class.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// address which triggers the mint.
    #[prost(string, tag = "2")]
    pub from: ::prost::alloc::string::String,
    /// recipient of the tokens.
    #[prost(string, tag = "3")]
    pub to: ::prost::alloc::string::String,
    /// number of tokens to mint.
    #[prost(string, tag = "4")]
    pub amount: ::prost::alloc::string::String,
}
/// MsgMintResponse defines the Msg/Mint response type.
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
#[proto_message(type_url = "/lbm.token.v1.MsgMintResponse")]
pub struct MsgMintResponse {}
/// MsgBurn defines the Msg/Burn request type.
///
/// Signer: `from`
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
#[proto_message(type_url = "/lbm.token.v1.MsgBurn")]
pub struct MsgBurn {
    /// contract id associated with the token class.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// address whose tokens are being burned.
    #[prost(string, tag = "2")]
    pub from: ::prost::alloc::string::String,
    /// number of tokens to burn.
    #[prost(string, tag = "3")]
    pub amount: ::prost::alloc::string::String,
}
/// MsgBurnResponse defines the Msg/Burn response type.
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
#[proto_message(type_url = "/lbm.token.v1.MsgBurnResponse")]
pub struct MsgBurnResponse {}
/// MsgOperatorBurn defines the Msg/OperatorBurn request type.
///
/// Signer: `operator`
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
#[proto_message(type_url = "/lbm.token.v1.MsgOperatorBurn")]
pub struct MsgOperatorBurn {
    /// contract id associated with the token class.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// address which triggers the burn.
    #[prost(string, tag = "2")]
    pub operator: ::prost::alloc::string::String,
    /// address which the tokens will be burnt from.
    #[prost(string, tag = "3")]
    pub from: ::prost::alloc::string::String,
    /// the amount of the burn.
    #[prost(string, tag = "4")]
    pub amount: ::prost::alloc::string::String,
}
/// MsgOperatorBurnResponse defines the Msg/OperatorBurn response type.
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
#[proto_message(type_url = "/lbm.token.v1.MsgOperatorBurnResponse")]
pub struct MsgOperatorBurnResponse {}
/// MsgModify defines the Msg/Modify request type.
///
/// Signer: `owner`
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
#[proto_message(type_url = "/lbm.token.v1.MsgModify")]
pub struct MsgModify {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// the address of the grantee which must have modify permission.
    #[prost(string, tag = "2")]
    pub owner: ::prost::alloc::string::String,
    /// changes to apply.
    /// possible attribute keys are: name, uri, img_uri (deprecated), meta
    #[prost(message, repeated, tag = "3")]
    pub changes: ::prost::alloc::vec::Vec<Attribute>,
}
/// MsgModifyResponse defines the Msg/Modify response type.
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
#[proto_message(type_url = "/lbm.token.v1.MsgModifyResponse")]
pub struct MsgModifyResponse {}
pub struct TokenQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> TokenQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn balance(
        &self,
        contract_id: ::prost::alloc::string::String,
        address: ::prost::alloc::string::String,
    ) -> Result<QueryBalanceResponse, cosmwasm_std::StdError> {
        QueryBalanceRequest {
            contract_id,
            address,
        }
        .query(self.querier)
    }
    pub fn supply(
        &self,
        contract_id: ::prost::alloc::string::String,
    ) -> Result<QuerySupplyResponse, cosmwasm_std::StdError> {
        QuerySupplyRequest { contract_id }.query(self.querier)
    }
    pub fn minted(
        &self,
        contract_id: ::prost::alloc::string::String,
    ) -> Result<QueryMintedResponse, cosmwasm_std::StdError> {
        QueryMintedRequest { contract_id }.query(self.querier)
    }
    pub fn burnt(
        &self,
        contract_id: ::prost::alloc::string::String,
    ) -> Result<QueryBurntResponse, cosmwasm_std::StdError> {
        QueryBurntRequest { contract_id }.query(self.querier)
    }
    pub fn contract(
        &self,
        contract_id: ::prost::alloc::string::String,
    ) -> Result<QueryContractResponse, cosmwasm_std::StdError> {
        QueryContractRequest { contract_id }.query(self.querier)
    }
    pub fn grantee_grants(
        &self,
        contract_id: ::prost::alloc::string::String,
        grantee: ::prost::alloc::string::String,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryGranteeGrantsResponse, cosmwasm_std::StdError> {
        QueryGranteeGrantsRequest {
            contract_id,
            grantee,
            pagination,
        }
        .query(self.querier)
    }
    pub fn is_operator_for(
        &self,
        contract_id: ::prost::alloc::string::String,
        operator: ::prost::alloc::string::String,
        holder: ::prost::alloc::string::String,
    ) -> Result<QueryIsOperatorForResponse, cosmwasm_std::StdError> {
        QueryIsOperatorForRequest {
            contract_id,
            operator,
            holder,
        }
        .query(self.querier)
    }
    pub fn holders_by_operator(
        &self,
        contract_id: ::prost::alloc::string::String,
        operator: ::prost::alloc::string::String,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryHoldersByOperatorResponse, cosmwasm_std::StdError> {
        QueryHoldersByOperatorRequest {
            contract_id,
            operator,
            pagination,
        }
        .query(self.querier)
    }
}
