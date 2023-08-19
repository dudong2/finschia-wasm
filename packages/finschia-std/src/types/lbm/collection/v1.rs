use finschia_std_derive::CosmwasmExt;
/// Params defines the parameters for the collection module.
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
#[proto_message(type_url = "/lbm.collection.v1.Params")]
pub struct Params {
    #[prost(uint32, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub depth_limit: u32,
    #[prost(uint32, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub width_limit: u32,
}
/// Contract defines the information of the contract for the collection.
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
#[proto_message(type_url = "/lbm.collection.v1.Contract")]
pub struct Contract {
    /// contract_id defines the unique identifier of the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "ID")]
    pub id: ::prost::alloc::string::String,
    /// name defines the human-readable name of the contract.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// meta is a brief description of the contract.
    #[prost(string, tag = "3")]
    pub meta: ::prost::alloc::string::String,
    /// uri for the contract image stored off chain.
    #[prost(string, tag = "4")]
    pub uri: ::prost::alloc::string::String,
}
/// FTClass defines the class of fungible token.
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
#[proto_message(type_url = "/lbm.collection.v1.FTClass")]
pub struct FtClass {
    /// id defines the unique identifier of the token class.
    /// Note: size of the class id is 8 in length.
    /// Note: token id of the fungible token would be `id` + `00000000`.
    #[prost(string, tag = "1")]
    #[serde(alias = "ID")]
    pub id: ::prost::alloc::string::String,
    /// name defines the human-readable name of the token class.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// meta is a brief description of the token class.
    #[prost(string, tag = "3")]
    pub meta: ::prost::alloc::string::String,
    /// decimals is the number of decimals which one must divide the amount by to get its user representation.
    #[prost(int32, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub decimals: i32,
    /// mintable represents whether the token class is allowed to mint or burn its tokens.
    #[prost(bool, tag = "5")]
    pub mintable: bool,
}
/// NFTClass defines the class of non-fungible token.
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
#[proto_message(type_url = "/lbm.collection.v1.NFTClass")]
pub struct NftClass {
    /// id defines the unique identifier of the token class.
    /// Note: size of the class id is 8 in length.
    #[prost(string, tag = "1")]
    #[serde(alias = "ID")]
    pub id: ::prost::alloc::string::String,
    /// name defines the human-readable name of the token class.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// meta is a brief description of the token class.
    #[prost(string, tag = "3")]
    pub meta: ::prost::alloc::string::String,
}
/// NFT defines the information of non-fungible token.
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
#[proto_message(type_url = "/lbm.collection.v1.NFT")]
pub struct Nft {
    /// token id defines the unique identifier of the token.
    #[prost(string, tag = "1")]
    #[serde(alias = "tokenID")]
    pub token_id: ::prost::alloc::string::String,
    /// name defines the human-readable name of the token.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// meta is a brief description of the token.
    #[prost(string, tag = "3")]
    pub meta: ::prost::alloc::string::String,
}
/// Deprecated: use NFT
///
/// OwnerNFT defines the information of non-fungible token.
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
#[proto_message(type_url = "/lbm.collection.v1.OwnerNFT")]
pub struct OwnerNft {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// id defines the unique identifier of the token.
    #[prost(string, tag = "2")]
    #[serde(alias = "tokenID")]
    pub token_id: ::prost::alloc::string::String,
    /// name defines the human-readable name of the token.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// meta is a brief description of the token.
    #[prost(string, tag = "4")]
    pub meta: ::prost::alloc::string::String,
    /// owner of the token.
    #[prost(string, tag = "5")]
    pub owner: ::prost::alloc::string::String,
}
/// Deprecated: use FTClass
///
/// FT defines the information of fungible token.
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
#[proto_message(type_url = "/lbm.collection.v1.FT")]
pub struct Ft {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// token id defines the unique identifier of the fungible token.
    #[prost(string, tag = "2")]
    #[serde(alias = "tokenID")]
    pub token_id: ::prost::alloc::string::String,
    /// name defines the human-readable name of the fungible token.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// meta is a brief description of the fungible token.
    #[prost(string, tag = "4")]
    pub meta: ::prost::alloc::string::String,
    /// decimals is the number of decimals which one must divide the amount by to get its user representation.
    #[prost(int32, tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub decimals: i32,
    /// mintable represents whether the fungible token is allowed to be minted or burnt.
    #[prost(bool, tag = "6")]
    pub mintable: bool,
}
/// Deprecated: use TokenClass
///
/// TokenType defines the information of token type.
/// It represents a NFTClass whose class_id is token_type.
///
/// Note: There is no TokenType instance for FTClass.
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
#[proto_message(type_url = "/lbm.collection.v1.TokenType")]
pub struct TokenType {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// token type defines the unique identifier of the token type.
    /// the format of the value is identical to that of class_id.
    #[prost(string, tag = "2")]
    pub token_type: ::prost::alloc::string::String,
    /// name defines the human-readable name of the token type.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// meta is a brief description of the token type.
    #[prost(string, tag = "4")]
    pub meta: ::prost::alloc::string::String,
}
/// Coin defines a token with a token id and an amount.
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
#[proto_message(type_url = "/lbm.collection.v1.Coin")]
pub struct Coin {
    /// token id associated with the token.
    #[prost(string, tag = "1")]
    #[serde(alias = "tokenID")]
    pub token_id: ::prost::alloc::string::String,
    /// amount of the token.
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
}
/// Grant defines permission given to a grantee.
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
#[proto_message(type_url = "/lbm.collection.v1.Grant")]
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
/// Authorization defines an authorization given to the operator on tokens of the holder.
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
#[proto_message(type_url = "/lbm.collection.v1.Authorization")]
pub struct Authorization {
    /// address of the holder which authorizes the manipulation of its tokens.
    #[prost(string, tag = "1")]
    pub holder: ::prost::alloc::string::String,
    /// address of the operator which the authorization is granted to.
    #[prost(string, tag = "2")]
    pub operator: ::prost::alloc::string::String,
}
/// Attribute defines a key and value of the attribute.
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
#[proto_message(type_url = "/lbm.collection.v1.Attribute")]
pub struct Attribute {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
/// Permission enumerates the valid permissions on a contract.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum Permission {
    /// unspecified defines the default permission which is invalid.
    Unspecified = 0,
    /// PERMISSION_ISSUE defines a permission to create a token class.
    Issue = 1,
    /// PERMISSION_MODIFY defines a permission to modify a contract.
    Modify = 2,
    /// PERMISSION_MINT defines a permission to mint tokens of a contract.
    Mint = 3,
    /// PERMISSION_BURN defines a permission to burn tokens of a contract.
    Burn = 4,
}
impl Permission {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Permission::Unspecified => "PERMISSION_UNSPECIFIED",
            Permission::Issue => "PERMISSION_ISSUE",
            Permission::Modify => "PERMISSION_MODIFY",
            Permission::Mint => "PERMISSION_MINT",
            Permission::Burn => "PERMISSION_BURN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PERMISSION_UNSPECIFIED" => Some(Self::Unspecified),
            "PERMISSION_ISSUE" => Some(Self::Issue),
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
    /// issue defines a permission to create a token class.
    Issue = 1,
    /// modify defines a permission to modify a contract.
    Modify = 2,
    /// mint defines a permission to mint tokens of a contract.
    Mint = 3,
    /// burn defines a permission to burn tokens of a contract.
    Burn = 4,
}
impl LegacyPermission {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LegacyPermission::Unspecified => "LEGACY_PERMISSION_UNSPECIFIED",
            LegacyPermission::Issue => "LEGACY_PERMISSION_ISSUE",
            LegacyPermission::Modify => "LEGACY_PERMISSION_MODIFY",
            LegacyPermission::Mint => "LEGACY_PERMISSION_MINT",
            LegacyPermission::Burn => "LEGACY_PERMISSION_BURN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LEGACY_PERMISSION_UNSPECIFIED" => Some(Self::Unspecified),
            "LEGACY_PERMISSION_ISSUE" => Some(Self::Issue),
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
#[proto_message(type_url = "/lbm.collection.v1.EventSent")]
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
    /// recipient of the tokens.
    #[prost(string, tag = "4")]
    pub to: ::prost::alloc::string::String,
    /// amount of tokens sent.
    #[prost(message, repeated, tag = "5")]
    pub amount: ::prost::alloc::vec::Vec<Coin>,
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
#[proto_message(type_url = "/lbm.collection.v1.EventAuthorizedOperator")]
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
#[proto_message(type_url = "/lbm.collection.v1.EventRevokedOperator")]
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
/// EventCreatedContract is emitted when a new contract is created.
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
#[proto_message(type_url = "/lbm.collection.v1.EventCreatedContract")]
pub struct EventCreatedContract {
    /// address which created the contract.
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    /// contract id associated with the contract.
    #[prost(string, tag = "2")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// name of the contract.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// metadata of the contract.
    #[prost(string, tag = "4")]
    pub meta: ::prost::alloc::string::String,
    /// uri for the contract image stored off chain.
    #[prost(string, tag = "5")]
    pub uri: ::prost::alloc::string::String,
}
/// EventCreatedFTClass is emitted when a new fungible token class is created.
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
#[proto_message(type_url = "/lbm.collection.v1.EventCreatedFTClass")]
pub struct EventCreatedFtClass {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// address which triggered the create.
    #[prost(string, tag = "2")]
    pub operator: ::prost::alloc::string::String,
    /// token id associated with the token class.
    #[prost(string, tag = "3")]
    #[serde(alias = "tokenID")]
    pub token_id: ::prost::alloc::string::String,
    /// name of the token class.
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
    /// metadata of the token class.
    #[prost(string, tag = "5")]
    pub meta: ::prost::alloc::string::String,
    /// decimals of the token class.
    #[prost(int32, tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub decimals: i32,
    /// mintable represents whether the token class is allowed to mint or burn its tokens.
    #[prost(bool, tag = "7")]
    pub mintable: bool,
}
/// EventCreatedNFTClass is emitted when a new non-fungible token class is created.
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
#[proto_message(type_url = "/lbm.collection.v1.EventCreatedNFTClass")]
pub struct EventCreatedNftClass {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// address which triggered the create.
    #[prost(string, tag = "2")]
    pub operator: ::prost::alloc::string::String,
    /// token type associated with the token class.
    /// refer to TokenType for the definition.
    #[prost(string, tag = "3")]
    pub token_type: ::prost::alloc::string::String,
    /// name of the token class.
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
    /// metadata of the token class.
    #[prost(string, tag = "5")]
    pub meta: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/lbm.collection.v1.EventGranted")]
pub struct EventGranted {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// address of the granter which grants the permission.
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
/// EventRenounced is emitted when a grantee renounced its permission.
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
#[proto_message(type_url = "/lbm.collection.v1.EventRenounced")]
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
/// EventMintedFT is emitted when fungible tokens are minted.
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
#[proto_message(type_url = "/lbm.collection.v1.EventMintedFT")]
pub struct EventMintedFt {
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
    /// amount of tokens minted.
    #[prost(message, repeated, tag = "4")]
    pub amount: ::prost::alloc::vec::Vec<Coin>,
}
/// EventMintedNFT is emitted when non-fungible tokens are minted.
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
#[proto_message(type_url = "/lbm.collection.v1.EventMintedNFT")]
pub struct EventMintedNft {
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
    /// tokens minted.
    #[prost(message, repeated, tag = "4")]
    pub tokens: ::prost::alloc::vec::Vec<Nft>,
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
#[proto_message(type_url = "/lbm.collection.v1.EventBurned")]
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
    /// amount of tokens burned.
    #[prost(message, repeated, tag = "4")]
    pub amount: ::prost::alloc::vec::Vec<Coin>,
}
/// EventModifiedContract is emitted when the information of a contract is modified.
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
#[proto_message(type_url = "/lbm.collection.v1.EventModifiedContract")]
pub struct EventModifiedContract {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// address which triggered the modify.
    #[prost(string, tag = "2")]
    pub operator: ::prost::alloc::string::String,
    /// changes of the attributes applied.
    /// possible attribute keys are same as those of MsgModify.
    /// deprecated "base_img_uri" has been replaced by "uri" in the events.
    #[prost(message, repeated, tag = "3")]
    pub changes: ::prost::alloc::vec::Vec<Attribute>,
}
/// EventModifiedTokenClass is emitted when the information of a token class is modified.
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
#[proto_message(type_url = "/lbm.collection.v1.EventModifiedTokenClass")]
pub struct EventModifiedTokenClass {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// address which triggered the modify.
    #[prost(string, tag = "2")]
    pub operator: ::prost::alloc::string::String,
    /// token type associated with the token class.
    /// refer to TokenType for the definition.
    #[prost(string, tag = "3")]
    pub token_type: ::prost::alloc::string::String,
    /// changes of the attributes applied.
    /// possible attribute keys are same as those of MsgModify.
    #[prost(message, repeated, tag = "4")]
    pub changes: ::prost::alloc::vec::Vec<Attribute>,
    /// type name of the token class.
    #[prost(string, tag = "5")]
    pub type_name: ::prost::alloc::string::String,
}
/// EventModifiedNFT is emitted when the information of a non-fungible token is modified.
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
#[proto_message(type_url = "/lbm.collection.v1.EventModifiedNFT")]
pub struct EventModifiedNft {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// address which triggered the modify.
    #[prost(string, tag = "2")]
    pub operator: ::prost::alloc::string::String,
    /// token id associated with the non-fungible token.
    #[prost(string, tag = "3")]
    #[serde(alias = "tokenID")]
    pub token_id: ::prost::alloc::string::String,
    /// changes of the attributes applied.
    /// possible attribute keys are same as those of MsgModify.
    #[prost(message, repeated, tag = "4")]
    pub changes: ::prost::alloc::vec::Vec<Attribute>,
}
/// EventAttached is emitted when a token is attached to another.
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
#[proto_message(type_url = "/lbm.collection.v1.EventAttached")]
pub struct EventAttached {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// address which triggered the attach.
    #[prost(string, tag = "2")]
    pub operator: ::prost::alloc::string::String,
    /// address which holds the tokens.
    #[prost(string, tag = "3")]
    pub holder: ::prost::alloc::string::String,
    /// subject of the attach.
    #[prost(string, tag = "4")]
    pub subject: ::prost::alloc::string::String,
    /// target of the attach.
    #[prost(string, tag = "5")]
    pub target: ::prost::alloc::string::String,
}
/// EventDetached is emitted when a token is detached from its parent.
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
#[proto_message(type_url = "/lbm.collection.v1.EventDetached")]
pub struct EventDetached {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// address which triggered the detach.
    #[prost(string, tag = "2")]
    pub operator: ::prost::alloc::string::String,
    /// address which holds the token.
    #[prost(string, tag = "3")]
    pub holder: ::prost::alloc::string::String,
    /// token being detached.
    #[prost(string, tag = "4")]
    pub subject: ::prost::alloc::string::String,
    /// parent token before the detach.
    #[prost(string, tag = "5")]
    pub previous_parent: ::prost::alloc::string::String,
}
/// EventOwnerChanged is emitted when the owner of token is changed by operation applied to its ancestor.
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
#[proto_message(type_url = "/lbm.collection.v1.EventOwnerChanged")]
pub struct EventOwnerChanged {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// token id associated with the token.
    #[prost(string, tag = "2")]
    #[serde(alias = "tokenID")]
    pub token_id: ::prost::alloc::string::String,
    /// address of the previous owner before the change.
    #[prost(string, tag = "3")]
    pub from: ::prost::alloc::string::String,
    /// address of the new owner.
    #[prost(string, tag = "4")]
    pub to: ::prost::alloc::string::String,
}
/// EventRootChanged is emitted when the root of token is changed by operation applied to its ancestor.
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
#[proto_message(type_url = "/lbm.collection.v1.EventRootChanged")]
pub struct EventRootChanged {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// token id associated with the token.
    #[prost(string, tag = "2")]
    #[serde(alias = "tokenID")]
    pub token_id: ::prost::alloc::string::String,
    /// token id of the previous root before the change.
    #[prost(string, tag = "3")]
    pub from: ::prost::alloc::string::String,
    /// token id of the new root.
    #[prost(string, tag = "4")]
    pub to: ::prost::alloc::string::String,
}
/// AttributeKey enumerates the valid attribute keys on x/collection.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum AttributeKey {
    Unspecified = 0,
    Name = 1,
    Meta = 2,
    /// deprecated: use ATTRIBUTE_KEY_URI
    BaseImgUri = 8,
    Uri = 20,
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
            AttributeKey::BaseImgUri => "ATTRIBUTE_KEY_BASE_IMG_URI",
            AttributeKey::Uri => "ATTRIBUTE_KEY_URI",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ATTRIBUTE_KEY_UNSPECIFIED" => Some(Self::Unspecified),
            "ATTRIBUTE_KEY_NAME" => Some(Self::Name),
            "ATTRIBUTE_KEY_META" => Some(Self::Meta),
            "ATTRIBUTE_KEY_BASE_IMG_URI" => Some(Self::BaseImgUri),
            "ATTRIBUTE_KEY_URI" => Some(Self::Uri),
            _ => None,
        }
    }
}
/// GenesisState defines the collection module's genesis state.
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
#[proto_message(type_url = "/lbm.collection.v1.GenesisState")]
pub struct GenesisState {
    /// params defines all the paramaters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// contracts defines the metadata of the contracts.
    #[prost(message, repeated, tag = "2")]
    pub contracts: ::prost::alloc::vec::Vec<Contract>,
    /// next ids for token classes.
    #[prost(message, repeated, tag = "3")]
    #[serde(alias = "next_classIDs")]
    pub next_class_ids: ::prost::alloc::vec::Vec<NextClassIDs>,
    /// classes defines the metadata of the tokens.
    #[prost(message, repeated, tag = "4")]
    pub classes: ::prost::alloc::vec::Vec<ContractClasses>,
    /// next ids for (non-fungible) tokens.
    #[prost(message, repeated, tag = "5")]
    #[serde(alias = "next_tokenIDs")]
    pub next_token_ids: ::prost::alloc::vec::Vec<ContractNextTokenIDs>,
    /// balances is an array containing the balances of all the accounts.
    #[prost(message, repeated, tag = "6")]
    pub balances: ::prost::alloc::vec::Vec<ContractBalances>,
    /// nfts is an array containing the nfts.
    #[prost(message, repeated, tag = "7")]
    pub nfts: ::prost::alloc::vec::Vec<ContractNfTs>,
    /// parents represents the parents of (non-fungible) tokens.
    #[prost(message, repeated, tag = "8")]
    pub parents: ::prost::alloc::vec::Vec<ContractTokenRelations>,
    /// grants defines the grant information.
    #[prost(message, repeated, tag = "9")]
    pub grants: ::prost::alloc::vec::Vec<ContractGrants>,
    /// authorizations defines the approve information.
    #[prost(message, repeated, tag = "10")]
    pub authorizations: ::prost::alloc::vec::Vec<ContractAuthorizations>,
    /// supplies represents the total supplies of tokens.
    #[prost(message, repeated, tag = "11")]
    pub supplies: ::prost::alloc::vec::Vec<ContractStatistics>,
    /// burnts represents the total amount of burnt tokens.
    #[prost(message, repeated, tag = "12")]
    pub burnts: ::prost::alloc::vec::Vec<ContractStatistics>,
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
#[proto_message(type_url = "/lbm.collection.v1.ContractBalances")]
pub struct ContractBalances {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// balances
    #[prost(message, repeated, tag = "2")]
    pub balances: ::prost::alloc::vec::Vec<Balance>,
}
/// ContractStatistics defines statistics belong to a contract.
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
#[proto_message(type_url = "/lbm.collection.v1.ContractStatistics")]
pub struct ContractStatistics {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// statistics
    #[prost(message, repeated, tag = "2")]
    pub statistics: ::prost::alloc::vec::Vec<ClassStatistics>,
}
/// ClassStatistics defines statistics belong to a token class.
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
#[proto_message(type_url = "/lbm.collection.v1.ClassStatistics")]
pub struct ClassStatistics {
    /// class id associated with the token class.
    #[prost(string, tag = "1")]
    #[serde(alias = "classID")]
    pub class_id: ::prost::alloc::string::String,
    /// statistics
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/lbm.collection.v1.Balance")]
pub struct Balance {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub amount: ::prost::alloc::vec::Vec<Coin>,
}
/// ContractClasses defines token classes belong to a contract.
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
#[proto_message(type_url = "/lbm.collection.v1.ContractClasses")]
pub struct ContractClasses {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// classes
    #[prost(message, repeated, tag = "2")]
    pub classes: ::prost::alloc::vec::Vec<crate::shim::Any>,
}
/// ContractNFTs defines token classes belong to a contract.
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
#[proto_message(type_url = "/lbm.collection.v1.ContractNFTs")]
pub struct ContractNfTs {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// nfts
    #[prost(message, repeated, tag = "2")]
    pub nfts: ::prost::alloc::vec::Vec<Nft>,
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
#[proto_message(type_url = "/lbm.collection.v1.ContractAuthorizations")]
pub struct ContractAuthorizations {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// authorizations
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
#[proto_message(type_url = "/lbm.collection.v1.ContractGrants")]
pub struct ContractGrants {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// grants
    #[prost(message, repeated, tag = "2")]
    pub grants: ::prost::alloc::vec::Vec<Grant>,
}
/// NextClassIDs defines the next class ids of the contract.
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
#[proto_message(type_url = "/lbm.collection.v1.NextClassIDs")]
pub struct NextClassIDs {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// id for the fungible tokens.
    #[prost(string, tag = "2")]
    pub fungible: ::prost::alloc::string::String,
    /// id for the non-fungible tokens.
    #[prost(string, tag = "3")]
    pub non_fungible: ::prost::alloc::string::String,
}
/// ContractNextTokenIDs defines the next token ids belong to a contract.
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
#[proto_message(type_url = "/lbm.collection.v1.ContractNextTokenIDs")]
pub struct ContractNextTokenIDs {
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    #[serde(alias = "tokenIDs")]
    pub token_ids: ::prost::alloc::vec::Vec<NextTokenId>,
}
/// NextTokenID defines the next (non-fungible) token id of the token class.
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
#[proto_message(type_url = "/lbm.collection.v1.NextTokenID")]
pub struct NextTokenId {
    /// class id associated with the token class.
    #[prost(string, tag = "1")]
    #[serde(alias = "classID")]
    pub class_id: ::prost::alloc::string::String,
    /// id for the token.
    #[prost(string, tag = "2")]
    #[serde(alias = "ID")]
    pub id: ::prost::alloc::string::String,
}
/// ContractTokenRelations defines token relations belong to a contract.
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
#[proto_message(type_url = "/lbm.collection.v1.ContractTokenRelations")]
pub struct ContractTokenRelations {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// relations
    #[prost(message, repeated, tag = "2")]
    pub relations: ::prost::alloc::vec::Vec<TokenRelation>,
}
/// TokenRelation defines relations between two tokens.
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
#[proto_message(type_url = "/lbm.collection.v1.TokenRelation")]
pub struct TokenRelation {
    /// self
    #[prost(string, tag = "1")]
    pub self_: ::prost::alloc::string::String,
    /// other
    #[prost(string, tag = "2")]
    pub other: ::prost::alloc::string::String,
}
/// QueryBalanceRequest is the request type for the Query/Balance RPC method.
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
#[proto_message(type_url = "/lbm.collection.v1.QueryBalanceRequest")]
#[proto_query(
    path = "/lbm.collection.v1.Query/Balance",
    response_type = QueryBalanceResponse
)]
pub struct QueryBalanceRequest {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// address is the address to query the balance for.
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
    /// token id associated with the token.
    #[prost(string, tag = "3")]
    #[serde(alias = "tokenID")]
    pub token_id: ::prost::alloc::string::String,
}
/// QueryBalanceResponse is the response type for the Query/Balance RPC method.
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
#[proto_message(type_url = "/lbm.collection.v1.QueryBalanceResponse")]
pub struct QueryBalanceResponse {
    /// balance is the balance of the token.
    #[prost(message, optional, tag = "1")]
    pub balance: ::core::option::Option<Coin>,
}
/// QueryAllBalancesRequest is the request type for the Query/AllBalances RPC method.
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
#[proto_message(type_url = "/lbm.collection.v1.QueryAllBalancesRequest")]
#[proto_query(
    path = "/lbm.collection.v1.Query/AllBalances",
    response_type = QueryAllBalancesResponse
)]
pub struct QueryAllBalancesRequest {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// address is the address to query the balances for.
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "3")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryAllBalancesResponse is the response type for the Query/AllBalances RPC method.
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
#[proto_message(type_url = "/lbm.collection.v1.QueryAllBalancesResponse")]
pub struct QueryAllBalancesResponse {
    /// balances is the balalces of all the tokens.
    #[prost(message, repeated, tag = "1")]
    pub balances: ::prost::alloc::vec::Vec<Coin>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryFTSupplyRequest is the request type for the Query/FTSupply RPC method.
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
#[proto_message(type_url = "/lbm.collection.v1.QueryFTSupplyRequest")]
#[proto_query(
    path = "/lbm.collection.v1.Query/FTSupply",
    response_type = QueryFtSupplyResponse
)]
pub struct QueryFtSupplyRequest {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// token id associated with the fungible token.
    #[prost(string, tag = "2")]
    #[serde(alias = "tokenID")]
    pub token_id: ::prost::alloc::string::String,
}
/// QueryFTSupplyResponse is the response type for the Query/FTSupply RPC method.
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
#[proto_message(type_url = "/lbm.collection.v1.QueryFTSupplyResponse")]
pub struct QueryFtSupplyResponse {
    /// supply is the supply of the tokens.
    #[prost(string, tag = "1")]
    pub supply: ::prost::alloc::string::String,
}
/// QueryFTMintedRequest is the request type for the Query/FTMinted RPC method.
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
#[proto_message(type_url = "/lbm.collection.v1.QueryFTMintedRequest")]
#[proto_query(
    path = "/lbm.collection.v1.Query/FTMinted",
    response_type = QueryFtMintedResponse
)]
pub struct QueryFtMintedRequest {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// token id associated with the fungible token.
    #[prost(string, tag = "2")]
    #[serde(alias = "tokenID")]
    pub token_id: ::prost::alloc::string::String,
}
/// QueryFTMintedResponse is the response type for the Query/FTMinted RPC method.
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
#[proto_message(type_url = "/lbm.collection.v1.QueryFTMintedResponse")]
pub struct QueryFtMintedResponse {
    /// minted is the amount of the minted tokens.
    #[prost(string, tag = "1")]
    pub minted: ::prost::alloc::string::String,
}
/// QueryFTBurntRequest is the request type for the Query/FTBurnt RPC method.
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
#[proto_message(type_url = "/lbm.collection.v1.QueryFTBurntRequest")]
#[proto_query(
    path = "/lbm.collection.v1.Query/FTBurnt",
    response_type = QueryFtBurntResponse
)]
pub struct QueryFtBurntRequest {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// token id associated with the fungible token.
    #[prost(string, tag = "2")]
    #[serde(alias = "tokenID")]
    pub token_id: ::prost::alloc::string::String,
}
/// QueryFTBurntResponse is the response type for the Query/FTBurnt RPC method.
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
#[proto_message(type_url = "/lbm.collection.v1.QueryFTBurntResponse")]
pub struct QueryFtBurntResponse {
    /// burnt is the amount of the burnt tokens.
    #[prost(string, tag = "1")]
    pub burnt: ::prost::alloc::string::String,
}
/// QueryNFTSupplyRequest is the request type for the Query/NFTSupply RPC method.
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
#[proto_message(type_url = "/lbm.collection.v1.QueryNFTSupplyRequest")]
#[proto_query(
    path = "/lbm.collection.v1.Query/NFTSupply",
    response_type = QueryNftSupplyResponse
)]
pub struct QueryNftSupplyRequest {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// token type associated with the token type.
    /// refer to TokenType for the definition.
    #[prost(string, tag = "2")]
    pub token_type: ::prost::alloc::string::String,
}
/// QueryNFTSupplyResponse is the response type for the Query/NFTSupply RPC method.
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
#[proto_message(type_url = "/lbm.collection.v1.QueryNFTSupplyResponse")]
pub struct QueryNftSupplyResponse {
    /// supply is the supply of the non-fungible token.
    #[prost(string, tag = "1")]
    pub supply: ::prost::alloc::string::String,
}
/// QueryNFTMintedRequest is the request type for the Query/NFTMinted RPC method.
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
#[proto_message(type_url = "/lbm.collection.v1.QueryNFTMintedRequest")]
#[proto_query(
    path = "/lbm.collection.v1.Query/NFTMinted",
    response_type = QueryNftMintedResponse
)]
pub struct QueryNftMintedRequest {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// token type associated with the token type.
    /// refer to TokenType for the definition.
    #[prost(string, tag = "2")]
    pub token_type: ::prost::alloc::string::String,
}
/// QueryNFTMintedResponse is the response type for the Query/NFTMinted RPC method.
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
#[proto_message(type_url = "/lbm.collection.v1.QueryNFTMintedResponse")]
pub struct QueryNftMintedResponse {
    /// minted is the amount of minted tokens.
    #[prost(string, tag = "1")]
    pub minted: ::prost::alloc::string::String,
}
/// QueryNFTBurntRequest is the request type for the Query/NFTBurnt RPC method.
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
#[proto_message(type_url = "/lbm.collection.v1.QueryNFTBurntRequest")]
#[proto_query(
    path = "/lbm.collection.v1.Query/NFTBurnt",
    response_type = QueryNftBurntResponse
)]
pub struct QueryNftBurntRequest {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// token type associated with the token type.
    /// refer to TokenType for the definition.
    #[prost(string, tag = "2")]
    pub token_type: ::prost::alloc::string::String,
}
/// QueryNFTBurntResponse is the response type for the Query/NFTBurnt RPC method.
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
#[proto_message(type_url = "/lbm.collection.v1.QueryNFTBurntResponse")]
pub struct QueryNftBurntResponse {
    /// burnt is the amount of the burnt tokens.
    #[prost(string, tag = "1")]
    pub burnt: ::prost::alloc::string::String,
}
/// QueryContractRequest is the request type for the Query/Contract RPC method.
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
#[proto_message(type_url = "/lbm.collection.v1.QueryContractRequest")]
#[proto_query(
    path = "/lbm.collection.v1.Query/Contract",
    response_type = QueryContractResponse
)]
pub struct QueryContractRequest {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
}
/// QueryContractResponse is the response type for the Query/Contract RPC method.
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
#[proto_message(type_url = "/lbm.collection.v1.QueryContractResponse")]
pub struct QueryContractResponse {
    /// contract is the information of the contract.
    #[prost(message, optional, tag = "1")]
    pub contract: ::core::option::Option<Contract>,
}
/// QueryTokenClassTypeNameRequest is the request type for the Query/TokenClassTypeName RPC method.
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
#[proto_message(type_url = "/lbm.collection.v1.QueryTokenClassTypeNameRequest")]
#[proto_query(
    path = "/lbm.collection.v1.Query/TokenClassTypeName",
    response_type = QueryTokenClassTypeNameResponse
)]
pub struct QueryTokenClassTypeNameRequest {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// class id associated with the token class.
    #[prost(string, tag = "2")]
    #[serde(alias = "classID")]
    pub class_id: ::prost::alloc::string::String,
}
/// QueryTokenClassTypeNameResponse is the response type for the Query/TokenClassTypeName RPC method.
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
#[proto_message(type_url = "/lbm.collection.v1.QueryTokenClassTypeNameResponse")]
pub struct QueryTokenClassTypeNameResponse {
    /// type name of the token class.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// QueryTokenTypeRequest is the request type for the Query/TokenType RPC method.
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
#[proto_message(type_url = "/lbm.collection.v1.QueryTokenTypeRequest")]
#[proto_query(
    path = "/lbm.collection.v1.Query/TokenType",
    response_type = QueryTokenTypeResponse
)]
pub struct QueryTokenTypeRequest {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// token type associated with the token type.
    /// refer to TokenType for the definition.
    #[prost(string, tag = "2")]
    pub token_type: ::prost::alloc::string::String,
}
/// QueryTokenTypeResponse is the response type for the Query/TokenType RPC method.
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
#[proto_message(type_url = "/lbm.collection.v1.QueryTokenTypeResponse")]
pub struct QueryTokenTypeResponse {
    /// token type is the information of the token type.
    #[prost(message, optional, tag = "1")]
    pub token_type: ::core::option::Option<TokenType>,
}
/// QueryTokenRequest is the request type for the Query/Token RPC method.
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
#[proto_message(type_url = "/lbm.collection.v1.QueryTokenRequest")]
#[proto_query(
    path = "/lbm.collection.v1.Query/Token",
    response_type = QueryTokenResponse
)]
pub struct QueryTokenRequest {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// token id associated with the fungible token.
    #[prost(string, tag = "2")]
    #[serde(alias = "tokenID")]
    pub token_id: ::prost::alloc::string::String,
}
/// QueryTokenResponse is the response type for the Query/Token RPC method.
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
#[proto_message(type_url = "/lbm.collection.v1.QueryTokenResponse")]
pub struct QueryTokenResponse {
    /// information of the token.
    #[prost(message, optional, tag = "1")]
    pub token: ::core::option::Option<crate::shim::Any>,
}
/// QueryRootRequest is the request type for the Query/Root RPC method.
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
#[proto_message(type_url = "/lbm.collection.v1.QueryRootRequest")]
#[proto_query(path = "/lbm.collection.v1.Query/Root", response_type = QueryRootResponse)]
pub struct QueryRootRequest {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// token id associated with the non-fungible token.
    #[prost(string, tag = "2")]
    #[serde(alias = "tokenID")]
    pub token_id: ::prost::alloc::string::String,
}
/// QueryRootResponse is the response type for the Query/Root RPC method.
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
#[proto_message(type_url = "/lbm.collection.v1.QueryRootResponse")]
pub struct QueryRootResponse {
    /// root is the information of the root token.
    /// it would return itself if it's the root token.
    #[prost(message, optional, tag = "1")]
    pub root: ::core::option::Option<Nft>,
}
/// QueryHasParentRequest is the request type for the Query/HasParent RPC method.
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
#[proto_message(type_url = "/lbm.collection.v1.QueryHasParentRequest")]
#[proto_query(
    path = "/lbm.collection.v1.Query/HasParent",
    response_type = QueryHasParentResponse
)]
pub struct QueryHasParentRequest {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// token id associated wit the non-fungible token.
    #[prost(string, tag = "2")]
    #[serde(alias = "tokenID")]
    pub token_id: ::prost::alloc::string::String,
}
/// QueryHasParentResponse is the response type for the Query/HasParent RPC method.
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
#[proto_message(type_url = "/lbm.collection.v1.QueryHasParentResponse")]
pub struct QueryHasParentResponse {
    /// whether the token has its parent.
    #[prost(bool, tag = "1")]
    pub has_parent: bool,
}
/// QueryParentRequest is the request type for the Query/Parent RPC method.
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
#[proto_message(type_url = "/lbm.collection.v1.QueryParentRequest")]
#[proto_query(
    path = "/lbm.collection.v1.Query/Parent",
    response_type = QueryParentResponse
)]
pub struct QueryParentRequest {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// token id associated wit the non-fungible token.
    #[prost(string, tag = "2")]
    #[serde(alias = "tokenID")]
    pub token_id: ::prost::alloc::string::String,
}
/// QueryParentResponse is the response type for the Query/Parent RPC method.
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
#[proto_message(type_url = "/lbm.collection.v1.QueryParentResponse")]
pub struct QueryParentResponse {
    /// parent is the information of the parent token.
    #[prost(message, optional, tag = "1")]
    pub parent: ::core::option::Option<Nft>,
}
/// QueryChildrenRequest is the request type for the Query/Children RPC method.
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
#[proto_message(type_url = "/lbm.collection.v1.QueryChildrenRequest")]
#[proto_query(
    path = "/lbm.collection.v1.Query/Children",
    response_type = QueryChildrenResponse
)]
pub struct QueryChildrenRequest {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// token id associated with the non-fungible token.
    #[prost(string, tag = "2")]
    #[serde(alias = "tokenID")]
    pub token_id: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "3")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryChildrenResponse is the response type for the Query/Children RPC method.
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
#[proto_message(type_url = "/lbm.collection.v1.QueryChildrenResponse")]
pub struct QueryChildrenResponse {
    /// children is the information of the child tokens.
    #[prost(message, repeated, tag = "1")]
    pub children: ::prost::alloc::vec::Vec<Nft>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryGranteeGrantsRequest is the request type for the Query/GranteeGrants RPC method.
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
#[proto_message(type_url = "/lbm.collection.v1.QueryGranteeGrantsRequest")]
#[proto_query(
    path = "/lbm.collection.v1.Query/GranteeGrants",
    response_type = QueryGranteeGrantsResponse
)]
pub struct QueryGranteeGrantsRequest {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// the address of the grantee.
    #[prost(string, tag = "2")]
    pub grantee: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "3")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryGranteeGrantsResponse is the response type for the Query/GranteeGrants RPC method.
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
#[proto_message(type_url = "/lbm.collection.v1.QueryGranteeGrantsResponse")]
pub struct QueryGranteeGrantsResponse {
    #[prost(message, repeated, tag = "1")]
    pub grants: ::prost::alloc::vec::Vec<Grant>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryIsOperatorForRequest is the request type for the Query/IsOperatorFor RPC method.
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
#[proto_message(type_url = "/lbm.collection.v1.QueryIsOperatorForRequest")]
#[proto_query(
    path = "/lbm.collection.v1.Query/IsOperatorFor",
    response_type = QueryIsOperatorForResponse
)]
pub struct QueryIsOperatorForRequest {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// the address of the operator.
    #[prost(string, tag = "2")]
    pub operator: ::prost::alloc::string::String,
    /// the address of the token holder.
    #[prost(string, tag = "3")]
    pub holder: ::prost::alloc::string::String,
}
/// QueryIsOperatorForResponse is the response type for the Query/IsOperatorFor RPC method.
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
#[proto_message(type_url = "/lbm.collection.v1.QueryIsOperatorForResponse")]
pub struct QueryIsOperatorForResponse {
    #[prost(bool, tag = "1")]
    pub authorized: bool,
}
/// QueryHoldersByOperatorRequest is the request type for the Query/HoldersByOperator RPC method.
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
#[proto_message(type_url = "/lbm.collection.v1.QueryHoldersByOperatorRequest")]
#[proto_query(
    path = "/lbm.collection.v1.Query/HoldersByOperator",
    response_type = QueryHoldersByOperatorResponse
)]
pub struct QueryHoldersByOperatorRequest {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// address of the operator.
    #[prost(string, tag = "2")]
    pub operator: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "3")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryHoldersByOperatorResponse is the response type for the Query/HoldersByOperator RPC method.
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
#[proto_message(type_url = "/lbm.collection.v1.QueryHoldersByOperatorResponse")]
pub struct QueryHoldersByOperatorResponse {
    #[prost(string, repeated, tag = "1")]
    pub holders: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// MsgSendFT is the Msg/SendFT request type.
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
#[proto_message(type_url = "/lbm.collection.v1.MsgSendFT")]
pub struct MsgSendFt {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// the address which the transfer is from.
    #[prost(string, tag = "2")]
    pub from: ::prost::alloc::string::String,
    /// the address which the transfer is to.
    #[prost(string, tag = "3")]
    pub to: ::prost::alloc::string::String,
    /// the amount of the transfer.
    /// Note: amount may be empty.
    #[prost(message, repeated, tag = "4")]
    pub amount: ::prost::alloc::vec::Vec<Coin>,
}
/// MsgSendFTResponse is the Msg/SendFT response type.
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
#[proto_message(type_url = "/lbm.collection.v1.MsgSendFTResponse")]
pub struct MsgSendFtResponse {}
/// MsgOperatorSendFT is the Msg/OperatorSendFT request type.
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
#[proto_message(type_url = "/lbm.collection.v1.MsgOperatorSendFT")]
pub struct MsgOperatorSendFt {
    /// contract id associated with the contract.
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
    /// Note: amount may be empty.
    #[prost(message, repeated, tag = "5")]
    pub amount: ::prost::alloc::vec::Vec<Coin>,
}
/// MsgOperatorSendFTResponse is the Msg/OperatorSendFT response type.
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
#[proto_message(type_url = "/lbm.collection.v1.MsgOperatorSendFTResponse")]
pub struct MsgOperatorSendFtResponse {}
/// MsgSendNFT is the Msg/SendNFT request type.
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
#[proto_message(type_url = "/lbm.collection.v1.MsgSendNFT")]
pub struct MsgSendNft {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// the address which the transfer is from.
    #[prost(string, tag = "2")]
    pub from: ::prost::alloc::string::String,
    /// the address which the transfer is to.
    #[prost(string, tag = "3")]
    pub to: ::prost::alloc::string::String,
    /// the token ids to transfer.
    #[prost(string, repeated, tag = "4")]
    #[serde(alias = "tokenIDs")]
    pub token_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgSendNFTResponse is the Msg/SendNFT response type.
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
#[proto_message(type_url = "/lbm.collection.v1.MsgSendNFTResponse")]
pub struct MsgSendNftResponse {}
/// MsgOperatorSendNFT is the Msg/OperatorSendNFT request type.
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
#[proto_message(type_url = "/lbm.collection.v1.MsgOperatorSendNFT")]
pub struct MsgOperatorSendNft {
    /// contract id associated with the contract.
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
    /// the token ids to transfer.
    #[prost(string, repeated, tag = "5")]
    #[serde(alias = "tokenIDs")]
    pub token_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgOperatorSendNFTResponse is the Msg/OperatorSendNFT response type.
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
#[proto_message(type_url = "/lbm.collection.v1.MsgOperatorSendNFTResponse")]
pub struct MsgOperatorSendNftResponse {}
/// MsgAuthorizeOperator is the Msg/AuthorizeOperator request type.
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
#[proto_message(type_url = "/lbm.collection.v1.MsgAuthorizeOperator")]
pub struct MsgAuthorizeOperator {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// address of the holder who allows the manipulation of its token.
    #[prost(string, tag = "2")]
    pub holder: ::prost::alloc::string::String,
    /// address which the manipulation is allowed to.
    #[prost(string, tag = "3")]
    pub operator: ::prost::alloc::string::String,
}
/// MsgAuthorizeOperatorResponse is the Msg/AuthorizeOperator response type.
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
#[proto_message(type_url = "/lbm.collection.v1.MsgAuthorizeOperatorResponse")]
pub struct MsgAuthorizeOperatorResponse {}
/// MsgRevokeOperator is the Msg/RevokeOperator request type.
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
#[proto_message(type_url = "/lbm.collection.v1.MsgRevokeOperator")]
pub struct MsgRevokeOperator {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// address of the holder who allows the manipulation of its token.
    #[prost(string, tag = "2")]
    pub holder: ::prost::alloc::string::String,
    /// address which the manipulation is allowed to.
    #[prost(string, tag = "3")]
    pub operator: ::prost::alloc::string::String,
}
/// MsgRevokeOperatorResponse is the Msg/RevokeOperator response type.
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
#[proto_message(type_url = "/lbm.collection.v1.MsgRevokeOperatorResponse")]
pub struct MsgRevokeOperatorResponse {}
/// MsgCreateContract is the Msg/CreateContract request type.
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
#[proto_message(type_url = "/lbm.collection.v1.MsgCreateContract")]
pub struct MsgCreateContract {
    /// address which all the permissions on the contract will be granted to (not a permanent property).
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    /// name defines the human-readable name of the contract.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// uri for the contract image stored off chain.
    #[prost(string, tag = "3")]
    pub uri: ::prost::alloc::string::String,
    /// meta is a brief description of the contract.
    #[prost(string, tag = "4")]
    pub meta: ::prost::alloc::string::String,
}
/// MsgCreateContractResponse is the Msg/CreateContract response type.
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
#[proto_message(type_url = "/lbm.collection.v1.MsgCreateContractResponse")]
pub struct MsgCreateContractResponse {
    /// id of the new contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
}
/// MsgIssueFT is the Msg/IssueFT request type.
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
#[proto_message(type_url = "/lbm.collection.v1.MsgIssueFT")]
pub struct MsgIssueFt {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// name defines the human-readable name of the token type.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// meta is a brief description of the token type.
    #[prost(string, tag = "3")]
    pub meta: ::prost::alloc::string::String,
    /// decimals is the number of decimals which one must divide the amount by to get its user representation.
    #[prost(int32, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub decimals: i32,
    /// mintable represents whether the token is allowed to be minted or burnt.
    #[prost(bool, tag = "5")]
    pub mintable: bool,
    /// the address of the grantee which must have the permission to issue a token.
    #[prost(string, tag = "6")]
    pub owner: ::prost::alloc::string::String,
    /// the address to send the minted tokens to. mandatory.
    #[prost(string, tag = "7")]
    pub to: ::prost::alloc::string::String,
    /// the amount of tokens to mint on the issuance.
    /// Note: if you provide negative amount, a panic may result.
    /// Note: amount may be zero.
    #[prost(string, tag = "8")]
    pub amount: ::prost::alloc::string::String,
}
/// MsgIssueFTResponse is the Msg/IssueFT response type.
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
#[proto_message(type_url = "/lbm.collection.v1.MsgIssueFTResponse")]
pub struct MsgIssueFtResponse {
    /// id of the token.
    #[prost(string, tag = "1")]
    #[serde(alias = "tokenID")]
    pub token_id: ::prost::alloc::string::String,
}
/// MsgIssueNFT is the Msg/IssueNFT request type.
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
#[proto_message(type_url = "/lbm.collection.v1.MsgIssueNFT")]
pub struct MsgIssueNft {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// name defines the human-readable name of the token type.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// meta is a brief description of the token type.
    #[prost(string, tag = "3")]
    pub meta: ::prost::alloc::string::String,
    /// the address of the grantee which must have the permission to issue a token.
    #[prost(string, tag = "4")]
    pub owner: ::prost::alloc::string::String,
}
/// MsgIssueNFTResponse is the Msg/IssueNFT response type.
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
#[proto_message(type_url = "/lbm.collection.v1.MsgIssueNFTResponse")]
pub struct MsgIssueNftResponse {
    /// id of the new token type.
    /// refer to TokenType for the definition.
    #[prost(string, tag = "1")]
    pub token_type: ::prost::alloc::string::String,
}
/// MsgMintFT is the Msg/MintFT request type.
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
#[proto_message(type_url = "/lbm.collection.v1.MsgMintFT")]
pub struct MsgMintFt {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// address of the grantee which has the permission for the mint.
    #[prost(string, tag = "2")]
    pub from: ::prost::alloc::string::String,
    /// address which the minted tokens will be sent to.
    #[prost(string, tag = "3")]
    pub to: ::prost::alloc::string::String,
    /// the amount of the mint.
    /// Note: amount may be empty.
    #[prost(message, repeated, tag = "4")]
    pub amount: ::prost::alloc::vec::Vec<Coin>,
}
/// MsgMintFTResponse is the Msg/MintFT response type.
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
#[proto_message(type_url = "/lbm.collection.v1.MsgMintFTResponse")]
pub struct MsgMintFtResponse {}
/// MsgMintNFT is the Msg/MintNFT request type.
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
#[proto_message(type_url = "/lbm.collection.v1.MsgMintNFT")]
pub struct MsgMintNft {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// address of the grantee which has the permission for the mint.
    #[prost(string, tag = "2")]
    pub from: ::prost::alloc::string::String,
    /// address which the minted token will be sent to.
    #[prost(string, tag = "3")]
    pub to: ::prost::alloc::string::String,
    /// parameters for the minted tokens.
    #[prost(message, repeated, tag = "4")]
    pub params: ::prost::alloc::vec::Vec<MintNftParam>,
}
/// MsgMintNFTResponse is the Msg/MintNFT response type.
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
#[proto_message(type_url = "/lbm.collection.v1.MsgMintNFTResponse")]
pub struct MsgMintNftResponse {
    /// ids of the new non-fungible tokens.
    #[prost(string, repeated, tag = "1")]
    #[serde(alias = "tokenIDs")]
    pub token_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MintNFTParam defines a parameter for minting nft.
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
#[proto_message(type_url = "/lbm.collection.v1.MintNFTParam")]
pub struct MintNftParam {
    /// token type or class id of the nft.
    /// Note: it cannot start with zero.
    /// refer to TokenType for the definition.
    #[prost(string, tag = "1")]
    pub token_type: ::prost::alloc::string::String,
    /// name defines the human-readable name of the nft (mandatory).
    /// Note: it has an app-specific limit in length.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// meta is a brief description of the nft.
    /// Note: it has an app-specific limit in length.
    #[prost(string, tag = "3")]
    pub meta: ::prost::alloc::string::String,
}
/// MsgBurnFT is the Msg/BurnFT request type.
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
#[proto_message(type_url = "/lbm.collection.v1.MsgBurnFT")]
pub struct MsgBurnFt {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// address which the tokens will be burnt from.
    /// Note: it must have the permission for the burn.
    #[prost(string, tag = "2")]
    pub from: ::prost::alloc::string::String,
    /// the amount of the burn.
    /// Note: amount may be empty.
    #[prost(message, repeated, tag = "3")]
    pub amount: ::prost::alloc::vec::Vec<Coin>,
}
/// MsgBurnFTResponse is the Msg/BurnFT response type.
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
#[proto_message(type_url = "/lbm.collection.v1.MsgBurnFTResponse")]
pub struct MsgBurnFtResponse {}
/// MsgOperatorBurnFT is the Msg/OperatorBurnFT request type.
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
#[proto_message(type_url = "/lbm.collection.v1.MsgOperatorBurnFT")]
pub struct MsgOperatorBurnFt {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// address which triggers the burn.
    /// Note: it must have the permission for the burn.
    /// Note: it must have been authorized by from.
    #[prost(string, tag = "2")]
    pub operator: ::prost::alloc::string::String,
    /// address which the tokens will be burnt from.
    #[prost(string, tag = "3")]
    pub from: ::prost::alloc::string::String,
    /// the amount of the burn.
    /// Note: amount may be empty.
    #[prost(message, repeated, tag = "4")]
    pub amount: ::prost::alloc::vec::Vec<Coin>,
}
/// MsgOperatorBurnFTResponse is the Msg/OperatorBurnFT response type.
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
#[proto_message(type_url = "/lbm.collection.v1.MsgOperatorBurnFTResponse")]
pub struct MsgOperatorBurnFtResponse {}
/// MsgBurnNFT is the Msg/BurnNFT request type.
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
#[proto_message(type_url = "/lbm.collection.v1.MsgBurnNFT")]
pub struct MsgBurnNft {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// address which the tokens will be burnt from.
    /// Note: it must have the permission for the burn.
    #[prost(string, tag = "2")]
    pub from: ::prost::alloc::string::String,
    /// the token ids to burn.
    /// Note: id cannot start with zero.
    #[prost(string, repeated, tag = "3")]
    #[serde(alias = "tokenIDs")]
    pub token_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgBurnNFTResponse is the Msg/BurnNFT response type.
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
#[proto_message(type_url = "/lbm.collection.v1.MsgBurnNFTResponse")]
pub struct MsgBurnNftResponse {}
/// MsgOperatorBurnNFT is the Msg/OperatorBurnNFT request type.
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
#[proto_message(type_url = "/lbm.collection.v1.MsgOperatorBurnNFT")]
pub struct MsgOperatorBurnNft {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// address which triggers the burn.
    /// Note: it must have the permission for the burn.
    /// Note: it must have been authorized by from.
    #[prost(string, tag = "2")]
    pub operator: ::prost::alloc::string::String,
    /// address which the tokens will be burnt from.
    #[prost(string, tag = "3")]
    pub from: ::prost::alloc::string::String,
    /// the token ids to burn.
    /// Note: id cannot start with zero.
    #[prost(string, repeated, tag = "4")]
    #[serde(alias = "tokenIDs")]
    pub token_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgOperatorBurnNFTResponse is the Msg/OperatorBurnNFT response type.
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
#[proto_message(type_url = "/lbm.collection.v1.MsgOperatorBurnNFTResponse")]
pub struct MsgOperatorBurnNftResponse {}
/// MsgModify is the Msg/Modify request type.
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
#[proto_message(type_url = "/lbm.collection.v1.MsgModify")]
pub struct MsgModify {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// the address of the grantee which must have modify permission.
    #[prost(string, tag = "2")]
    pub owner: ::prost::alloc::string::String,
    /// token type of the token.
    /// refer to TokenType for the definition.
    #[prost(string, tag = "3")]
    pub token_type: ::prost::alloc::string::String,
    /// token index of the token.
    /// if index is empty, it would modify the corresponding token type.
    /// if index is not empty, it would modify the corresponding nft.
    /// Note: if token type is of FTs, the index cannot be empty.
    #[prost(string, tag = "4")]
    pub token_index: ::prost::alloc::string::String,
    /// changes to apply.
    /// possible attribute keys on modifying collection: name, uri, base_img_uri (deprecated), meta.
    /// possible attribute keys on modifying token type and token: name, meta.
    #[prost(message, repeated, tag = "5")]
    pub changes: ::prost::alloc::vec::Vec<Attribute>,
}
/// MsgModifyResponse is the Msg/Modify response type.
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
#[proto_message(type_url = "/lbm.collection.v1.MsgModifyResponse")]
pub struct MsgModifyResponse {}
/// MsgGrantPermission is the Msg/GrantPermission request type.
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
#[proto_message(type_url = "/lbm.collection.v1.MsgGrantPermission")]
pub struct MsgGrantPermission {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// address of the granter which must have the permission to give.
    #[prost(string, tag = "2")]
    pub from: ::prost::alloc::string::String,
    /// address of the grantee.
    #[prost(string, tag = "3")]
    pub to: ::prost::alloc::string::String,
    /// permission on the contract.
    #[prost(string, tag = "4")]
    pub permission: ::prost::alloc::string::String,
}
/// MsgGrantPermissionResponse is the Msg/GrantPermission response type.
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
#[proto_message(type_url = "/lbm.collection.v1.MsgGrantPermissionResponse")]
pub struct MsgGrantPermissionResponse {}
/// MsgRevokePermission is the Msg/RevokePermission request type.
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
#[proto_message(type_url = "/lbm.collection.v1.MsgRevokePermission")]
pub struct MsgRevokePermission {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// address of the grantee which abandons the permission.
    #[prost(string, tag = "2")]
    pub from: ::prost::alloc::string::String,
    /// permission on the contract.
    #[prost(string, tag = "3")]
    pub permission: ::prost::alloc::string::String,
}
/// MsgRevokePermissionResponse is the Msg/RevokePermission response type.
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
#[proto_message(type_url = "/lbm.collection.v1.MsgRevokePermissionResponse")]
pub struct MsgRevokePermissionResponse {}
/// MsgAttach is the Msg/Attach request type.
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
#[proto_message(type_url = "/lbm.collection.v1.MsgAttach")]
pub struct MsgAttach {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// address of the owner of the token.
    #[prost(string, tag = "2")]
    pub from: ::prost::alloc::string::String,
    /// token id of the token to attach.
    #[prost(string, tag = "3")]
    #[serde(alias = "tokenID")]
    pub token_id: ::prost::alloc::string::String,
    /// to token id which one attachs the token to.
    #[prost(string, tag = "4")]
    #[serde(alias = "to_tokenID")]
    pub to_token_id: ::prost::alloc::string::String,
}
/// MsgAttachResponse is the Msg/Attach response type.
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
#[proto_message(type_url = "/lbm.collection.v1.MsgAttachResponse")]
pub struct MsgAttachResponse {}
/// MsgDetach is the Msg/Detach request type.
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
#[proto_message(type_url = "/lbm.collection.v1.MsgDetach")]
pub struct MsgDetach {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// address of the owner of the token.
    #[prost(string, tag = "2")]
    pub from: ::prost::alloc::string::String,
    /// token id of the token to detach.
    #[prost(string, tag = "3")]
    #[serde(alias = "tokenID")]
    pub token_id: ::prost::alloc::string::String,
}
/// MsgDetachResponse is the Msg/Detach response type.
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
#[proto_message(type_url = "/lbm.collection.v1.MsgDetachResponse")]
pub struct MsgDetachResponse {}
/// MsgOperatorAttach is the Msg/OperatorAttach request type.
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
#[proto_message(type_url = "/lbm.collection.v1.MsgOperatorAttach")]
pub struct MsgOperatorAttach {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// address of the operator.
    #[prost(string, tag = "2")]
    pub operator: ::prost::alloc::string::String,
    /// address of the owner of the token.
    #[prost(string, tag = "3")]
    pub from: ::prost::alloc::string::String,
    /// token id of the token to attach.
    #[prost(string, tag = "4")]
    #[serde(alias = "tokenID")]
    pub token_id: ::prost::alloc::string::String,
    /// to token id which one attachs the token to.
    #[prost(string, tag = "5")]
    #[serde(alias = "to_tokenID")]
    pub to_token_id: ::prost::alloc::string::String,
}
/// MsgOperatorAttachResponse is the Msg/OperatorAttach response type.
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
#[proto_message(type_url = "/lbm.collection.v1.MsgOperatorAttachResponse")]
pub struct MsgOperatorAttachResponse {}
/// MsgOperatorDetach is the Msg/OperatorDetach request type.
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
#[proto_message(type_url = "/lbm.collection.v1.MsgOperatorDetach")]
pub struct MsgOperatorDetach {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    #[serde(alias = "contractID")]
    pub contract_id: ::prost::alloc::string::String,
    /// address of the operator.
    #[prost(string, tag = "2")]
    pub operator: ::prost::alloc::string::String,
    /// address of the owner of the token.
    #[prost(string, tag = "3")]
    pub from: ::prost::alloc::string::String,
    /// token id of the token to detach.
    #[prost(string, tag = "4")]
    #[serde(alias = "tokenID")]
    pub token_id: ::prost::alloc::string::String,
}
/// MsgOperatorDetachResponse is the Msg/OperatorDetach response type.
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
#[proto_message(type_url = "/lbm.collection.v1.MsgOperatorDetachResponse")]
pub struct MsgOperatorDetachResponse {}
pub struct CollectionQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> CollectionQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn balance(
        &self,
        contract_id: ::prost::alloc::string::String,
        address: ::prost::alloc::string::String,
        token_id: ::prost::alloc::string::String,
    ) -> Result<QueryBalanceResponse, cosmwasm_std::StdError> {
        QueryBalanceRequest {
            contract_id,
            address,
            token_id,
        }
        .query(self.querier)
    }
    pub fn all_balances(
        &self,
        contract_id: ::prost::alloc::string::String,
        address: ::prost::alloc::string::String,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryAllBalancesResponse, cosmwasm_std::StdError> {
        QueryAllBalancesRequest {
            contract_id,
            address,
            pagination,
        }
        .query(self.querier)
    }
    pub fn ft_supply(
        &self,
        contract_id: ::prost::alloc::string::String,
        token_id: ::prost::alloc::string::String,
    ) -> Result<QueryFtSupplyResponse, cosmwasm_std::StdError> {
        QueryFtSupplyRequest {
            contract_id,
            token_id,
        }
        .query(self.querier)
    }
    pub fn ft_minted(
        &self,
        contract_id: ::prost::alloc::string::String,
        token_id: ::prost::alloc::string::String,
    ) -> Result<QueryFtMintedResponse, cosmwasm_std::StdError> {
        QueryFtMintedRequest {
            contract_id,
            token_id,
        }
        .query(self.querier)
    }
    pub fn ft_burnt(
        &self,
        contract_id: ::prost::alloc::string::String,
        token_id: ::prost::alloc::string::String,
    ) -> Result<QueryFtBurntResponse, cosmwasm_std::StdError> {
        QueryFtBurntRequest {
            contract_id,
            token_id,
        }
        .query(self.querier)
    }
    pub fn nft_supply(
        &self,
        contract_id: ::prost::alloc::string::String,
        token_type: ::prost::alloc::string::String,
    ) -> Result<QueryNftSupplyResponse, cosmwasm_std::StdError> {
        QueryNftSupplyRequest {
            contract_id,
            token_type,
        }
        .query(self.querier)
    }
    pub fn nft_minted(
        &self,
        contract_id: ::prost::alloc::string::String,
        token_type: ::prost::alloc::string::String,
    ) -> Result<QueryNftMintedResponse, cosmwasm_std::StdError> {
        QueryNftMintedRequest {
            contract_id,
            token_type,
        }
        .query(self.querier)
    }
    pub fn nft_burnt(
        &self,
        contract_id: ::prost::alloc::string::String,
        token_type: ::prost::alloc::string::String,
    ) -> Result<QueryNftBurntResponse, cosmwasm_std::StdError> {
        QueryNftBurntRequest {
            contract_id,
            token_type,
        }
        .query(self.querier)
    }
    pub fn contract(
        &self,
        contract_id: ::prost::alloc::string::String,
    ) -> Result<QueryContractResponse, cosmwasm_std::StdError> {
        QueryContractRequest { contract_id }.query(self.querier)
    }
    pub fn token_class_type_name(
        &self,
        contract_id: ::prost::alloc::string::String,
        class_id: ::prost::alloc::string::String,
    ) -> Result<QueryTokenClassTypeNameResponse, cosmwasm_std::StdError> {
        QueryTokenClassTypeNameRequest {
            contract_id,
            class_id,
        }
        .query(self.querier)
    }
    pub fn token_type(
        &self,
        contract_id: ::prost::alloc::string::String,
        token_type: ::prost::alloc::string::String,
    ) -> Result<QueryTokenTypeResponse, cosmwasm_std::StdError> {
        QueryTokenTypeRequest {
            contract_id,
            token_type,
        }
        .query(self.querier)
    }
    pub fn token(
        &self,
        contract_id: ::prost::alloc::string::String,
        token_id: ::prost::alloc::string::String,
    ) -> Result<QueryTokenResponse, cosmwasm_std::StdError> {
        QueryTokenRequest {
            contract_id,
            token_id,
        }
        .query(self.querier)
    }
    pub fn root(
        &self,
        contract_id: ::prost::alloc::string::String,
        token_id: ::prost::alloc::string::String,
    ) -> Result<QueryRootResponse, cosmwasm_std::StdError> {
        QueryRootRequest {
            contract_id,
            token_id,
        }
        .query(self.querier)
    }
    pub fn has_parent(
        &self,
        contract_id: ::prost::alloc::string::String,
        token_id: ::prost::alloc::string::String,
    ) -> Result<QueryHasParentResponse, cosmwasm_std::StdError> {
        QueryHasParentRequest {
            contract_id,
            token_id,
        }
        .query(self.querier)
    }
    pub fn parent(
        &self,
        contract_id: ::prost::alloc::string::String,
        token_id: ::prost::alloc::string::String,
    ) -> Result<QueryParentResponse, cosmwasm_std::StdError> {
        QueryParentRequest {
            contract_id,
            token_id,
        }
        .query(self.querier)
    }
    pub fn children(
        &self,
        contract_id: ::prost::alloc::string::String,
        token_id: ::prost::alloc::string::String,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryChildrenResponse, cosmwasm_std::StdError> {
        QueryChildrenRequest {
            contract_id,
            token_id,
            pagination,
        }
        .query(self.querier)
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
