use finschia_std_derive::CosmwasmExt;
/// ReceiveFromTreasuryAuthorization allows the grantee to receive coins
/// up to receive_limit from the treasury.
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
#[proto_message(type_url = "/lbm.foundation.v1.ReceiveFromTreasuryAuthorization")]
pub struct ReceiveFromTreasuryAuthorization {}
/// Params defines the parameters for the foundation module.
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
#[proto_message(type_url = "/lbm.foundation.v1.Params")]
pub struct Params {
    #[prost(string, tag = "1")]
    pub foundation_tax: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/lbm.foundation.v1.Censorship")]
pub struct Censorship {
    #[prost(string, tag = "1")]
    pub msg_type_url: ::prost::alloc::string::String,
    #[prost(enumeration = "CensorshipAuthority", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub authority: i32,
}
/// Member represents a foundation member with an account address and metadata.
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
#[proto_message(type_url = "/lbm.foundation.v1.Member")]
pub struct Member {
    /// address is the member's account address.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// metadata is any arbitrary metadata to attached to the member.
    #[prost(string, tag = "2")]
    pub metadata: ::prost::alloc::string::String,
    /// added_at is a timestamp specifying when a member was added.
    #[prost(message, optional, tag = "4")]
    pub added_at: ::core::option::Option<crate::shim::Timestamp>,
}
/// MemberRequest represents a foundation member to be used in Msg server requests.
/// Contrary to `Member`, it doesn't have any `added_at` field
/// since this field cannot be set as part of requests.
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
#[proto_message(type_url = "/lbm.foundation.v1.MemberRequest")]
pub struct MemberRequest {
    /// address is the member's account address.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// remove is the flag which allows one to remove the member by setting the flag to true.
    #[prost(bool, tag = "2")]
    pub remove: bool,
    /// metadata is any arbitrary metadata attached to the member.
    #[prost(string, tag = "3")]
    pub metadata: ::prost::alloc::string::String,
}
/// ThresholdDecisionPolicy is a decision policy where a proposal passes when it
/// satisfies the two following conditions:
/// 1. The sum of all `YES` voters' weights is greater or equal than the defined
///     `threshold`.
/// 2. The voting and execution periods of the proposal respect the parameters
///     given by `windows`.
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
#[proto_message(type_url = "/lbm.foundation.v1.ThresholdDecisionPolicy")]
pub struct ThresholdDecisionPolicy {
    /// threshold is the minimum sum of yes votes that must be met or exceeded for a proposal to succeed.
    #[prost(string, tag = "1")]
    pub threshold: ::prost::alloc::string::String,
    /// windows defines the different windows for voting and execution.
    #[prost(message, optional, tag = "2")]
    pub windows: ::core::option::Option<DecisionPolicyWindows>,
}
/// PercentageDecisionPolicy is a decision policy where a proposal passes when
/// it satisfies the two following conditions:
/// 1. The percentage of all `YES` voters' weights out of the total group weight
///     is greater or equal than the given `percentage`.
/// 2. The voting and execution periods of the proposal respect the parameters
///     given by `windows`.
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
#[proto_message(type_url = "/lbm.foundation.v1.PercentageDecisionPolicy")]
pub struct PercentageDecisionPolicy {
    /// percentage is the minimum percentage the sum of yes votes must meet for a proposal to succeed.
    #[prost(string, tag = "1")]
    pub percentage: ::prost::alloc::string::String,
    /// windows defines the different windows for voting and execution.
    #[prost(message, optional, tag = "2")]
    pub windows: ::core::option::Option<DecisionPolicyWindows>,
}
/// DecisionPolicyWindows defines the different windows for voting and execution.
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
#[proto_message(type_url = "/lbm.foundation.v1.DecisionPolicyWindows")]
pub struct DecisionPolicyWindows {
    /// voting_period is the duration from submission of a proposal to the end of voting period
    /// Within this times votes can be submitted with MsgVote.
    #[prost(message, optional, tag = "1")]
    pub voting_period: ::core::option::Option<crate::shim::Duration>,
    /// min_execution_period is the minimum duration after the proposal submission
    /// where members can start sending MsgExec. This means that the window for
    /// sending a MsgExec transaction is:
    /// `[ submission + min_execution_period ; submission + voting_period + max_execution_period]`
    /// where max_execution_period is a app-specific config, defined in the keeper.
    /// If not set, min_execution_period will default to 0.
    ///
    /// Please make sure to set a `min_execution_period` that is smaller than
    /// `voting_period + max_execution_period`, or else the above execution window
    /// is empty, meaning that all proposals created with this decision policy
    /// won't be able to be executed.
    #[prost(message, optional, tag = "2")]
    pub min_execution_period: ::core::option::Option<crate::shim::Duration>,
}
/// OutsourcingDecisionPolicy is a dummy decision policy which is set after
/// the proposal feature has been outsourced to x/group.
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
#[proto_message(type_url = "/lbm.foundation.v1.OutsourcingDecisionPolicy")]
pub struct OutsourcingDecisionPolicy {
    #[prost(string, tag = "1")]
    pub description: ::prost::alloc::string::String,
}
/// FoundationInfo represents the high-level on-chain information for the foundation.
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
#[proto_message(type_url = "/lbm.foundation.v1.FoundationInfo")]
pub struct FoundationInfo {
    /// version is used to track changes to the foundation's membership structure that
    /// would break existing proposals. Whenever any member is added or removed,
    /// this version is incremented and will cause proposals based on older versions
    /// of the foundation to fail
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub version: u64,
    /// total_weight is the number of the foundation members.
    #[prost(string, tag = "2")]
    pub total_weight: ::prost::alloc::string::String,
    /// decision_policy specifies the foundation's decision policy.
    #[prost(message, optional, tag = "3")]
    pub decision_policy: ::core::option::Option<crate::shim::Any>,
}
/// Proposal defines a foundation proposal. Any member of the foundation can submit a proposal
/// for a group policy to decide upon.
/// A proposal consists of a set of `sdk.Msg`s that will be executed if the proposal
/// passes as well as some optional metadata associated with the proposal.
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
#[proto_message(type_url = "/lbm.foundation.v1.Proposal")]
pub struct Proposal {
    /// id is the unique id of the proposal.
    #[prost(uint64, tag = "1")]
    #[serde(alias = "ID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub id: u64,
    /// metadata is any arbitrary metadata to attached to the proposal.
    #[prost(string, tag = "2")]
    pub metadata: ::prost::alloc::string::String,
    /// proposers are the account addresses of the proposers.
    #[prost(string, repeated, tag = "3")]
    pub proposers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// submit_time is a timestamp specifying when a proposal was submitted.
    #[prost(message, optional, tag = "4")]
    pub submit_time: ::core::option::Option<crate::shim::Timestamp>,
    /// foundation_version tracks the version of the foundation that this proposal corresponds to.
    /// When foundation info is changed, existing proposals from previous foundation versions will become invalid.
    #[prost(uint64, tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub foundation_version: u64,
    /// status represents the high level position in the life cycle of the proposal. Initial value is Submitted.
    #[prost(enumeration = "ProposalStatus", tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub status: i32,
    /// final_tally_result contains the sums of all votes for this
    /// proposal for each vote option, after tallying. When querying a proposal
    /// via gRPC, this field is not populated until the proposal's voting period
    /// has ended.
    #[prost(message, optional, tag = "7")]
    pub final_tally_result: ::core::option::Option<TallyResult>,
    /// voting_period_end is the timestamp before which voting must be done.
    /// Unless a successfull MsgExec is called before (to execute a proposal whose
    /// tally is successful before the voting period ends), tallying will be done
    /// at this point, and the `final_tally_result`, as well
    /// as `status` and `result` fields will be accordingly updated.
    #[prost(message, optional, tag = "8")]
    pub voting_period_end: ::core::option::Option<crate::shim::Timestamp>,
    /// executor_result is the final result based on the votes and election rule. Initial value is NotRun.
    #[prost(enumeration = "ProposalExecutorResult", tag = "9")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub executor_result: i32,
    /// messages is a list of Msgs that will be executed if the proposal passes.
    #[prost(message, repeated, tag = "10")]
    pub messages: ::prost::alloc::vec::Vec<crate::shim::Any>,
}
/// TallyResult represents the sum of votes for each vote option.
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
#[proto_message(type_url = "/lbm.foundation.v1.TallyResult")]
pub struct TallyResult {
    /// yes_count is the sum of yes votes.
    #[prost(string, tag = "1")]
    pub yes_count: ::prost::alloc::string::String,
    /// abstain_count is the sum of abstainers.
    #[prost(string, tag = "2")]
    pub abstain_count: ::prost::alloc::string::String,
    /// no is the sum of no votes.
    #[prost(string, tag = "3")]
    pub no_count: ::prost::alloc::string::String,
    /// no_with_veto_count is the sum of veto.
    #[prost(string, tag = "4")]
    pub no_with_veto_count: ::prost::alloc::string::String,
}
/// Vote represents a vote for a proposal.
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
#[proto_message(type_url = "/lbm.foundation.v1.Vote")]
pub struct Vote {
    /// proposal is the unique ID of the proposal.
    #[prost(uint64, tag = "1")]
    #[serde(alias = "proposalID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub proposal_id: u64,
    /// voter is the account address of the voter.
    #[prost(string, tag = "2")]
    pub voter: ::prost::alloc::string::String,
    /// option is the voter's choice on the proposal.
    #[prost(enumeration = "VoteOption", tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub option: i32,
    /// metadata is any arbitrary metadata to attached to the vote.
    #[prost(string, tag = "4")]
    pub metadata: ::prost::alloc::string::String,
    /// submit_time is the timestamp when the vote was submitted.
    #[prost(message, optional, tag = "5")]
    pub submit_time: ::core::option::Option<crate::shim::Timestamp>,
}
/// Pool is used for tracking treasury.
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
#[proto_message(type_url = "/lbm.foundation.v1.Pool")]
pub struct Pool {
    #[prost(message, repeated, tag = "1")]
    pub treasury: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::DecCoin>,
}
/// FoundationExecProposal is x/gov proposal to trigger the x/foundation messages on behalf of x/gov.
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
#[proto_message(type_url = "/lbm.foundation.v1.FoundationExecProposal")]
pub struct FoundationExecProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// x/foundation messages to execute
    /// all the signers must be x/gov authority.
    #[prost(message, repeated, tag = "3")]
    pub messages: ::prost::alloc::vec::Vec<crate::shim::Any>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum CensorshipAuthority {
    /// CENSORSHIP_AUTHORITY_UNSPECIFIED defines an invalid authority.
    Unspecified = 0,
    /// CENSORSHIP_AUTHORITY_GOVERNANCE defines x/gov authority.
    Governance = 1,
    /// CENSORSHIP_AUTHORITY_FOUNDATION defines x/foundation authority.
    Foundation = 2,
}
impl CensorshipAuthority {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CensorshipAuthority::Unspecified => "CENSORSHIP_AUTHORITY_UNSPECIFIED",
            CensorshipAuthority::Governance => "CENSORSHIP_AUTHORITY_GOVERNANCE",
            CensorshipAuthority::Foundation => "CENSORSHIP_AUTHORITY_FOUNDATION",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CENSORSHIP_AUTHORITY_UNSPECIFIED" => Some(Self::Unspecified),
            "CENSORSHIP_AUTHORITY_GOVERNANCE" => Some(Self::Governance),
            "CENSORSHIP_AUTHORITY_FOUNDATION" => Some(Self::Foundation),
            _ => None,
        }
    }
}
/// VoteOption enumerates the valid vote options for a given proposal.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum VoteOption {
    /// VOTE_OPTION_UNSPECIFIED defines a no-op vote option.
    Unspecified = 0,
    /// VOTE_OPTION_YES defines a yes vote option.
    Yes = 1,
    /// VOTE_OPTION_ABSTAIN defines an abstain vote option.
    Abstain = 2,
    /// VOTE_OPTION_NO defines a no vote option.
    No = 3,
    /// VOTE_OPTION_NO_WITH_VETO defines a no with veto vote option.
    NoWithVeto = 4,
}
impl VoteOption {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            VoteOption::Unspecified => "VOTE_OPTION_UNSPECIFIED",
            VoteOption::Yes => "VOTE_OPTION_YES",
            VoteOption::Abstain => "VOTE_OPTION_ABSTAIN",
            VoteOption::No => "VOTE_OPTION_NO",
            VoteOption::NoWithVeto => "VOTE_OPTION_NO_WITH_VETO",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "VOTE_OPTION_UNSPECIFIED" => Some(Self::Unspecified),
            "VOTE_OPTION_YES" => Some(Self::Yes),
            "VOTE_OPTION_ABSTAIN" => Some(Self::Abstain),
            "VOTE_OPTION_NO" => Some(Self::No),
            "VOTE_OPTION_NO_WITH_VETO" => Some(Self::NoWithVeto),
            _ => None,
        }
    }
}
/// ProposalStatus defines proposal statuses.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum ProposalStatus {
    /// An empty value is invalid and not allowed.
    Unspecified = 0,
    /// Initial status of a proposal when submitted.
    Submitted = 1,
    /// Final status of a proposal when the final tally is done and the outcome
    /// passes the foundation's decision policy.
    Accepted = 2,
    /// Final status of a proposal when the final tally is done and the outcome
    /// is rejected by the foundation's decision policy.
    Rejected = 3,
    /// Final status of a proposal when the decision policy is modified before the
    /// final tally.
    Aborted = 4,
    /// A proposal can be withdrawn before the voting start time by the owner.
    /// When this happens the final status is Withdrawn.
    Withdrawn = 5,
}
impl ProposalStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ProposalStatus::Unspecified => "PROPOSAL_STATUS_UNSPECIFIED",
            ProposalStatus::Submitted => "PROPOSAL_STATUS_SUBMITTED",
            ProposalStatus::Accepted => "PROPOSAL_STATUS_ACCEPTED",
            ProposalStatus::Rejected => "PROPOSAL_STATUS_REJECTED",
            ProposalStatus::Aborted => "PROPOSAL_STATUS_ABORTED",
            ProposalStatus::Withdrawn => "PROPOSAL_STATUS_WITHDRAWN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PROPOSAL_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "PROPOSAL_STATUS_SUBMITTED" => Some(Self::Submitted),
            "PROPOSAL_STATUS_ACCEPTED" => Some(Self::Accepted),
            "PROPOSAL_STATUS_REJECTED" => Some(Self::Rejected),
            "PROPOSAL_STATUS_ABORTED" => Some(Self::Aborted),
            "PROPOSAL_STATUS_WITHDRAWN" => Some(Self::Withdrawn),
            _ => None,
        }
    }
}
/// ProposalExecutorResult defines types of proposal executor results.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum ProposalExecutorResult {
    /// An empty value is not allowed.
    Unspecified = 0,
    /// We have not yet run the executor.
    NotRun = 1,
    /// The executor was successful and proposed action updated state.
    Success = 2,
    /// The executor returned an error and proposed action didn't update state.
    Failure = 3,
}
impl ProposalExecutorResult {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ProposalExecutorResult::Unspecified => "PROPOSAL_EXECUTOR_RESULT_UNSPECIFIED",
            ProposalExecutorResult::NotRun => "PROPOSAL_EXECUTOR_RESULT_NOT_RUN",
            ProposalExecutorResult::Success => "PROPOSAL_EXECUTOR_RESULT_SUCCESS",
            ProposalExecutorResult::Failure => "PROPOSAL_EXECUTOR_RESULT_FAILURE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PROPOSAL_EXECUTOR_RESULT_UNSPECIFIED" => Some(Self::Unspecified),
            "PROPOSAL_EXECUTOR_RESULT_NOT_RUN" => Some(Self::NotRun),
            "PROPOSAL_EXECUTOR_RESULT_SUCCESS" => Some(Self::Success),
            "PROPOSAL_EXECUTOR_RESULT_FAILURE" => Some(Self::Failure),
            _ => None,
        }
    }
}
/// EventFundTreasury is an event emitted when one funds the treasury.
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
#[proto_message(type_url = "/lbm.foundation.v1.EventFundTreasury")]
pub struct EventFundTreasury {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub amount: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// EventWithdrawFromTreasury is an event emitted when coins are withdrawn from the treasury.
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
#[proto_message(type_url = "/lbm.foundation.v1.EventWithdrawFromTreasury")]
pub struct EventWithdrawFromTreasury {
    #[prost(string, tag = "1")]
    pub to: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub amount: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// EventUpdateMembers is an event emitted when the members have been updated.
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
#[proto_message(type_url = "/lbm.foundation.v1.EventUpdateMembers")]
pub struct EventUpdateMembers {
    #[prost(message, repeated, tag = "1")]
    pub member_updates: ::prost::alloc::vec::Vec<MemberRequest>,
}
/// EventUpdateDecisionPolicy is an event emitted when the decision policy have been updated.
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
#[proto_message(type_url = "/lbm.foundation.v1.EventUpdateDecisionPolicy")]
pub struct EventUpdateDecisionPolicy {
    #[prost(message, optional, tag = "1")]
    pub decision_policy: ::core::option::Option<crate::shim::Any>,
}
/// EventSubmitProposal is an event emitted when a proposal is created.
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
#[proto_message(type_url = "/lbm.foundation.v1.EventSubmitProposal")]
pub struct EventSubmitProposal {
    /// proposal is the unique ID of the proposal.
    #[prost(message, optional, tag = "1")]
    pub proposal: ::core::option::Option<Proposal>,
}
/// EventWithdrawProposal is an event emitted when a proposal is withdrawn.
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
#[proto_message(type_url = "/lbm.foundation.v1.EventWithdrawProposal")]
pub struct EventWithdrawProposal {
    /// proposal_id is the unique ID of the proposal.
    #[prost(uint64, tag = "1")]
    #[serde(alias = "proposalID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub proposal_id: u64,
}
/// EventVote is an event emitted when a voter votes on a proposal.
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
#[proto_message(type_url = "/lbm.foundation.v1.EventVote")]
pub struct EventVote {
    #[prost(message, optional, tag = "1")]
    pub vote: ::core::option::Option<Vote>,
}
/// EventExec is an event emitted when a proposal is executed.
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
#[proto_message(type_url = "/lbm.foundation.v1.EventExec")]
pub struct EventExec {
    /// proposal_id is the unique ID of the proposal.
    #[prost(uint64, tag = "1")]
    #[serde(alias = "proposalID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub proposal_id: u64,
    /// result is the proposal execution result.
    #[prost(enumeration = "ProposalExecutorResult", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub result: i32,
    /// logs contains error logs in case the execution result is FAILURE.
    #[prost(string, tag = "3")]
    pub logs: ::prost::alloc::string::String,
}
/// EventLeaveFoundation is an event emitted when a foundation member leaves the foundation.
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
#[proto_message(type_url = "/lbm.foundation.v1.EventLeaveFoundation")]
pub struct EventLeaveFoundation {
    /// address is the account address of the foundation member.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// EventUpdateCensorship is emitted when a censorship information updated.
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
#[proto_message(type_url = "/lbm.foundation.v1.EventUpdateCensorship")]
pub struct EventUpdateCensorship {
    #[prost(message, optional, tag = "1")]
    pub censorship: ::core::option::Option<Censorship>,
}
/// EventGrant is emitted on Msg/Grant
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
#[proto_message(type_url = "/lbm.foundation.v1.EventGrant")]
pub struct EventGrant {
    /// the address of the grantee.
    #[prost(string, tag = "1")]
    pub grantee: ::prost::alloc::string::String,
    /// authorization granted.
    #[prost(message, optional, tag = "2")]
    pub authorization: ::core::option::Option<crate::shim::Any>,
}
/// EventRevoke is emitted on Msg/Revoke
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
#[proto_message(type_url = "/lbm.foundation.v1.EventRevoke")]
pub struct EventRevoke {
    /// address of the grantee.
    #[prost(string, tag = "1")]
    pub grantee: ::prost::alloc::string::String,
    /// message type url for which an autorization is revoked.
    #[prost(string, tag = "2")]
    pub msg_type_url: ::prost::alloc::string::String,
}
/// GenesisState defines the foundation module's genesis state.
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
#[proto_message(type_url = "/lbm.foundation.v1.GenesisState")]
pub struct GenesisState {
    /// params defines the module parameters at genesis.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// foundation is the foundation info.
    #[prost(message, optional, tag = "2")]
    pub foundation: ::core::option::Option<FoundationInfo>,
    /// members is the list of the foundation members.
    #[prost(message, repeated, tag = "3")]
    pub members: ::prost::alloc::vec::Vec<Member>,
    /// it is used to get the next proposal ID.
    #[prost(uint64, tag = "4")]
    #[serde(alias = "previous_proposalID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub previous_proposal_id: u64,
    /// proposals is the list of proposals.
    #[prost(message, repeated, tag = "5")]
    pub proposals: ::prost::alloc::vec::Vec<Proposal>,
    /// votes is the list of votes.
    #[prost(message, repeated, tag = "6")]
    pub votes: ::prost::alloc::vec::Vec<Vote>,
    /// grants
    #[prost(message, repeated, tag = "7")]
    pub authorizations: ::prost::alloc::vec::Vec<GrantAuthorization>,
    /// pool
    #[prost(message, optional, tag = "8")]
    pub pool: ::core::option::Option<Pool>,
    #[prost(message, repeated, tag = "10")]
    pub censorships: ::prost::alloc::vec::Vec<Censorship>,
}
/// GrantAuthorization defines authorization grant to grantee via route.
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
#[proto_message(type_url = "/lbm.foundation.v1.GrantAuthorization")]
pub struct GrantAuthorization {
    #[prost(string, tag = "1")]
    pub grantee: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub authorization: ::core::option::Option<crate::shim::Any>,
}
/// QueryParamsRequest is the request type for the Query/Params RPC method.
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
#[proto_message(type_url = "/lbm.foundation.v1.QueryParamsRequest")]
#[proto_query(
    path = "/lbm.foundation.v1.Query/Params",
    response_type = QueryParamsResponse
)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is the response type for the Query/Params RPC method.
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
#[proto_message(type_url = "/lbm.foundation.v1.QueryParamsResponse")]
pub struct QueryParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryTreasuryRequest is the request type for the
/// Query/Treasury RPC method.
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
#[proto_message(type_url = "/lbm.foundation.v1.QueryTreasuryRequest")]
#[proto_query(
    path = "/lbm.foundation.v1.Query/Treasury",
    response_type = QueryTreasuryResponse
)]
pub struct QueryTreasuryRequest {}
/// QueryTreasuryResponse is the response type for the
/// Query/Treasury RPC method.
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
#[proto_message(type_url = "/lbm.foundation.v1.QueryTreasuryResponse")]
pub struct QueryTreasuryResponse {
    #[prost(message, repeated, tag = "1")]
    pub amount: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::DecCoin>,
}
/// QueryFoundationInfoRequest is the Query/FoundationInfo request type.
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
#[proto_message(type_url = "/lbm.foundation.v1.QueryFoundationInfoRequest")]
#[proto_query(
    path = "/lbm.foundation.v1.Query/FoundationInfo",
    response_type = QueryFoundationInfoResponse
)]
pub struct QueryFoundationInfoRequest {}
/// QueryFoundationInfoResponse is the Query/FoundationInfo response type.
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
#[proto_message(type_url = "/lbm.foundation.v1.QueryFoundationInfoResponse")]
pub struct QueryFoundationInfoResponse {
    /// info is the FoundationInfo for the foundation.
    #[prost(message, optional, tag = "1")]
    pub info: ::core::option::Option<FoundationInfo>,
}
/// QueryMemberRequest is the Query/Member request type.
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
#[proto_message(type_url = "/lbm.foundation.v1.QueryMemberRequest")]
#[proto_query(
    path = "/lbm.foundation.v1.Query/Member",
    response_type = QueryMemberResponse
)]
pub struct QueryMemberRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// QueryMemberResponse is the Query/MemberResponse response type.
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
#[proto_message(type_url = "/lbm.foundation.v1.QueryMemberResponse")]
pub struct QueryMemberResponse {
    /// member is the members of the foundation.
    #[prost(message, optional, tag = "1")]
    pub member: ::core::option::Option<Member>,
}
/// QueryMembersRequest is the Query/Members request type.
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
#[proto_message(type_url = "/lbm.foundation.v1.QueryMembersRequest")]
#[proto_query(
    path = "/lbm.foundation.v1.Query/Members",
    response_type = QueryMembersResponse
)]
pub struct QueryMembersRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryMembersResponse is the Query/MembersResponse response type.
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
#[proto_message(type_url = "/lbm.foundation.v1.QueryMembersResponse")]
pub struct QueryMembersResponse {
    /// members are the members of the foundation.
    #[prost(message, repeated, tag = "1")]
    pub members: ::prost::alloc::vec::Vec<Member>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryProposalRequest is the Query/Proposal request type.
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
#[proto_message(type_url = "/lbm.foundation.v1.QueryProposalRequest")]
#[proto_query(
    path = "/lbm.foundation.v1.Query/Proposal",
    response_type = QueryProposalResponse
)]
pub struct QueryProposalRequest {
    /// proposal_id is the unique ID of a proposal.
    #[prost(uint64, tag = "1")]
    #[serde(alias = "proposalID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub proposal_id: u64,
}
/// QueryProposalResponse is the Query/Proposal response type.
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
#[proto_message(type_url = "/lbm.foundation.v1.QueryProposalResponse")]
pub struct QueryProposalResponse {
    /// proposal is the proposal info.
    #[prost(message, optional, tag = "1")]
    pub proposal: ::core::option::Option<Proposal>,
}
/// QueryProposals is the Query/Proposals request type.
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
#[proto_message(type_url = "/lbm.foundation.v1.QueryProposalsRequest")]
#[proto_query(
    path = "/lbm.foundation.v1.Query/Proposals",
    response_type = QueryProposalsResponse
)]
pub struct QueryProposalsRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryProposalsResponse is the Query/Proposals response type.
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
#[proto_message(type_url = "/lbm.foundation.v1.QueryProposalsResponse")]
pub struct QueryProposalsResponse {
    /// proposals are the proposals of the foundation.
    #[prost(message, repeated, tag = "1")]
    pub proposals: ::prost::alloc::vec::Vec<Proposal>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryVote is the Query/Vote request type.
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
#[proto_message(type_url = "/lbm.foundation.v1.QueryVoteRequest")]
#[proto_query(path = "/lbm.foundation.v1.Query/Vote", response_type = QueryVoteResponse)]
pub struct QueryVoteRequest {
    /// proposal_id is the unique ID of a proposal.
    #[prost(uint64, tag = "1")]
    #[serde(alias = "proposalID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub proposal_id: u64,
    /// voter is a proposal voter account address.
    #[prost(string, tag = "2")]
    pub voter: ::prost::alloc::string::String,
}
/// QueryVoteResponse is the Query/Vote response type.
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
#[proto_message(type_url = "/lbm.foundation.v1.QueryVoteResponse")]
pub struct QueryVoteResponse {
    /// vote is the vote with given proposal_id and voter.
    #[prost(message, optional, tag = "1")]
    pub vote: ::core::option::Option<Vote>,
}
/// QueryVotes is the Query/Votes request type.
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
#[proto_message(type_url = "/lbm.foundation.v1.QueryVotesRequest")]
#[proto_query(
    path = "/lbm.foundation.v1.Query/Votes",
    response_type = QueryVotesResponse
)]
pub struct QueryVotesRequest {
    /// proposal_id is the unique ID of a proposal.
    #[prost(uint64, tag = "1")]
    #[serde(alias = "proposalID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub proposal_id: u64,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryVotesResponse is the Query/Votes response type.
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
#[proto_message(type_url = "/lbm.foundation.v1.QueryVotesResponse")]
pub struct QueryVotesResponse {
    /// votes are the list of votes for given proposal_id.
    #[prost(message, repeated, tag = "1")]
    pub votes: ::prost::alloc::vec::Vec<Vote>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryTallyResultRequest is the Query/TallyResult request type.
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
#[proto_message(type_url = "/lbm.foundation.v1.QueryTallyResultRequest")]
#[proto_query(
    path = "/lbm.foundation.v1.Query/TallyResult",
    response_type = QueryTallyResultResponse
)]
pub struct QueryTallyResultRequest {
    /// proposal_id is the unique id of a proposal.
    #[prost(uint64, tag = "1")]
    #[serde(alias = "proposalID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub proposal_id: u64,
}
/// QueryTallyResultResponse is the Query/TallyResult response type.
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
#[proto_message(type_url = "/lbm.foundation.v1.QueryTallyResultResponse")]
pub struct QueryTallyResultResponse {
    /// tally defines the requested tally.
    #[prost(message, optional, tag = "1")]
    pub tally: ::core::option::Option<TallyResult>,
}
/// QueryCensorshipsRequest is the request type for the Query/Censorships RPC method.
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
#[proto_message(type_url = "/lbm.foundation.v1.QueryCensorshipsRequest")]
#[proto_query(
    path = "/lbm.foundation.v1.Query/Censorships",
    response_type = QueryCensorshipsResponse
)]
pub struct QueryCensorshipsRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryCensorshipsResponse is the response type for the Query/Censorships RPC method.
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
#[proto_message(type_url = "/lbm.foundation.v1.QueryCensorshipsResponse")]
pub struct QueryCensorshipsResponse {
    /// authorizations is a list of grants granted for grantee.
    #[prost(message, repeated, tag = "1")]
    pub censorships: ::prost::alloc::vec::Vec<Censorship>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryGrantsRequest is the request type for the Query/Grants RPC method.
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
#[proto_message(type_url = "/lbm.foundation.v1.QueryGrantsRequest")]
#[proto_query(
    path = "/lbm.foundation.v1.Query/Grants",
    response_type = QueryGrantsResponse
)]
pub struct QueryGrantsRequest {
    #[prost(string, tag = "1")]
    pub grantee: ::prost::alloc::string::String,
    /// Optional, msg_type_url, when set, will query only grants matching given msg type.
    #[prost(string, tag = "2")]
    pub msg_type_url: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "3")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryGrantsResponse is the response type for the Query/Grants RPC method.
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
#[proto_message(type_url = "/lbm.foundation.v1.QueryGrantsResponse")]
pub struct QueryGrantsResponse {
    /// authorizations is a list of grants granted for grantee.
    #[prost(message, repeated, tag = "1")]
    pub authorizations: ::prost::alloc::vec::Vec<crate::shim::Any>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// MsgFundTreasury is the Msg/FundTreasury request type.
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
#[proto_message(type_url = "/lbm.foundation.v1.MsgFundTreasury")]
pub struct MsgFundTreasury {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub amount: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgFundTreasuryResponse is the Msg/FundTreasury response type.
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
#[proto_message(type_url = "/lbm.foundation.v1.MsgFundTreasuryResponse")]
pub struct MsgFundTreasuryResponse {}
/// MsgWithdrawFromTreasury is the Msg/WithdrawFromTreasury request type.
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
#[proto_message(type_url = "/lbm.foundation.v1.MsgWithdrawFromTreasury")]
pub struct MsgWithdrawFromTreasury {
    /// authority is the address of the privileged account.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub to: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub amount: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgWithdrawFromTreasuryResponse is the Msg/WithdrawFromTreasury response type.
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
#[proto_message(type_url = "/lbm.foundation.v1.MsgWithdrawFromTreasuryResponse")]
pub struct MsgWithdrawFromTreasuryResponse {}
/// MsgUpdateMembers is the Msg/UpdateMembers request type.
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
#[proto_message(type_url = "/lbm.foundation.v1.MsgUpdateMembers")]
pub struct MsgUpdateMembers {
    /// authority is the address of the privileged account.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// member_updates is the list of members to update,
    /// set remove to true to remove a member.
    #[prost(message, repeated, tag = "2")]
    pub member_updates: ::prost::alloc::vec::Vec<MemberRequest>,
}
/// MsgUpdateMembersResponse is the Msg/UpdateMembers response type.
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
#[proto_message(type_url = "/lbm.foundation.v1.MsgUpdateMembersResponse")]
pub struct MsgUpdateMembersResponse {}
/// MsgUpdateDecisionPolicy is the Msg/UpdateDecisionPolicy request type.
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
#[proto_message(type_url = "/lbm.foundation.v1.MsgUpdateDecisionPolicy")]
pub struct MsgUpdateDecisionPolicy {
    /// authority is the address of the privileged account.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// decision_policy is the updated decision policy.
    #[prost(message, optional, tag = "2")]
    pub decision_policy: ::core::option::Option<crate::shim::Any>,
}
/// MsgUpdateDecisionPolicyResponse is the Msg/UpdateDecisionPolicy response type.
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
#[proto_message(type_url = "/lbm.foundation.v1.MsgUpdateDecisionPolicyResponse")]
pub struct MsgUpdateDecisionPolicyResponse {}
/// MsgSubmitProposal is the Msg/SubmitProposal request type.
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
#[proto_message(type_url = "/lbm.foundation.v1.MsgSubmitProposal")]
pub struct MsgSubmitProposal {
    /// proposers are the account addresses of the proposers.
    /// Proposers signatures will be counted as yes votes.
    #[prost(string, repeated, tag = "1")]
    pub proposers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// metadata is any arbitrary metadata to attached to the proposal.
    #[prost(string, tag = "2")]
    pub metadata: ::prost::alloc::string::String,
    /// messages is a list of `sdk.Msg`s that will be executed if the proposal passes.
    #[prost(message, repeated, tag = "3")]
    pub messages: ::prost::alloc::vec::Vec<crate::shim::Any>,
    /// exec defines the mode of execution of the proposal,
    /// whether it should be executed immediately on creation or not.
    /// If so, proposers signatures are considered as Yes votes.
    #[prost(enumeration = "Exec", tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub exec: i32,
}
/// MsgSubmitProposalResponse is the Msg/SubmitProposal response type.
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
#[proto_message(type_url = "/lbm.foundation.v1.MsgSubmitProposalResponse")]
pub struct MsgSubmitProposalResponse {
    /// proposal is the unique ID of the proposal.
    #[prost(uint64, tag = "1")]
    #[serde(alias = "proposalID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub proposal_id: u64,
}
/// MsgWithdrawProposal is the Msg/WithdrawProposal request type.
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
#[proto_message(type_url = "/lbm.foundation.v1.MsgWithdrawProposal")]
pub struct MsgWithdrawProposal {
    /// proposal is the unique ID of the proposal.
    #[prost(uint64, tag = "1")]
    #[serde(alias = "proposalID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub proposal_id: u64,
    /// address of one of the proposer of the proposal.
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
}
/// MsgWithdrawProposalResponse is the Msg/WithdrawProposal response type.
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
#[proto_message(type_url = "/lbm.foundation.v1.MsgWithdrawProposalResponse")]
pub struct MsgWithdrawProposalResponse {}
/// MsgVote is the Msg/Vote request type.
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
#[proto_message(type_url = "/lbm.foundation.v1.MsgVote")]
pub struct MsgVote {
    /// proposal is the unique ID of the proposal.
    #[prost(uint64, tag = "1")]
    #[serde(alias = "proposalID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub proposal_id: u64,
    /// voter is the voter account address.
    #[prost(string, tag = "2")]
    pub voter: ::prost::alloc::string::String,
    /// option is the voter's choice on the proposal.
    #[prost(enumeration = "VoteOption", tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub option: i32,
    /// metadata is any arbitrary metadata to attached to the vote.
    #[prost(string, tag = "4")]
    pub metadata: ::prost::alloc::string::String,
    /// exec defines whether the proposal should be executed
    /// immediately after voting or not.
    #[prost(enumeration = "Exec", tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub exec: i32,
}
/// MsgVoteResponse is the Msg/Vote response type.
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
#[proto_message(type_url = "/lbm.foundation.v1.MsgVoteResponse")]
pub struct MsgVoteResponse {}
/// MsgExec is the Msg/Exec request type.
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
#[proto_message(type_url = "/lbm.foundation.v1.MsgExec")]
pub struct MsgExec {
    /// proposal is the unique ID of the proposal.
    #[prost(uint64, tag = "1")]
    #[serde(alias = "proposalID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub proposal_id: u64,
    /// signer is the account address used to execute the proposal.
    #[prost(string, tag = "2")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgExecResponse is the Msg/Exec request type.
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
#[proto_message(type_url = "/lbm.foundation.v1.MsgExecResponse")]
pub struct MsgExecResponse {}
/// MsgLeaveFoundation is the Msg/LeaveFoundation request type.
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
#[proto_message(type_url = "/lbm.foundation.v1.MsgLeaveFoundation")]
pub struct MsgLeaveFoundation {
    /// address is the account address of the foundation member.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// MsgLeaveFoundationResponse is the Msg/LeaveFoundation response type.
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
#[proto_message(type_url = "/lbm.foundation.v1.MsgLeaveFoundationResponse")]
pub struct MsgLeaveFoundationResponse {}
/// MsgUpdateCensorship is the Msg/UpdateCensorship request type.
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
#[proto_message(type_url = "/lbm.foundation.v1.MsgUpdateCensorship")]
pub struct MsgUpdateCensorship {
    /// authority over the target censorship.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// new censorship information
    #[prost(message, optional, tag = "2")]
    pub censorship: ::core::option::Option<Censorship>,
}
/// MsgUpdateCensorshipResponse is the Msg/UpdateCensorship response type.
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
#[proto_message(type_url = "/lbm.foundation.v1.MsgUpdateCensorshipResponse")]
pub struct MsgUpdateCensorshipResponse {}
/// MsgGrant is the Msg/Grant request type.
/// on behalf of the foundation.
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
#[proto_message(type_url = "/lbm.foundation.v1.MsgGrant")]
pub struct MsgGrant {
    /// authority is the address of the privileged account.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub grantee: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub authorization: ::core::option::Option<crate::shim::Any>,
}
/// MsgGrantResponse is the Msg/MsgGrant response type.
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
#[proto_message(type_url = "/lbm.foundation.v1.MsgGrantResponse")]
pub struct MsgGrantResponse {}
/// MsgRevoke is the Msg/Revoke request type.
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
#[proto_message(type_url = "/lbm.foundation.v1.MsgRevoke")]
pub struct MsgRevoke {
    /// authority is the address of the privileged account.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub grantee: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub msg_type_url: ::prost::alloc::string::String,
}
/// MsgRevokeResponse is the Msg/MsgRevokeResponse response type.
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
#[proto_message(type_url = "/lbm.foundation.v1.MsgRevokeResponse")]
pub struct MsgRevokeResponse {}
/// Exec defines modes of execution of a proposal on creation or on new vote.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum Exec {
    /// An empty value means that there should be a separate
    /// MsgExec request for the proposal to execute.
    Unspecified = 0,
    /// Try to execute the proposal immediately.
    /// If the proposal is not allowed per the DecisionPolicy,
    /// the proposal will still be open and could
    /// be executed at a later point.
    Try = 1,
}
impl Exec {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Exec::Unspecified => "EXEC_UNSPECIFIED",
            Exec::Try => "EXEC_TRY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EXEC_UNSPECIFIED" => Some(Self::Unspecified),
            "EXEC_TRY" => Some(Self::Try),
            _ => None,
        }
    }
}
pub struct FoundationQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> FoundationQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn params(&self) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
    pub fn treasury(&self) -> Result<QueryTreasuryResponse, cosmwasm_std::StdError> {
        QueryTreasuryRequest {}.query(self.querier)
    }
    pub fn foundation_info(&self) -> Result<QueryFoundationInfoResponse, cosmwasm_std::StdError> {
        QueryFoundationInfoRequest {}.query(self.querier)
    }
    pub fn member(
        &self,
        address: ::prost::alloc::string::String,
    ) -> Result<QueryMemberResponse, cosmwasm_std::StdError> {
        QueryMemberRequest { address }.query(self.querier)
    }
    pub fn members(
        &self,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryMembersResponse, cosmwasm_std::StdError> {
        QueryMembersRequest { pagination }.query(self.querier)
    }
    pub fn proposal(
        &self,
        proposal_id: u64,
    ) -> Result<QueryProposalResponse, cosmwasm_std::StdError> {
        QueryProposalRequest { proposal_id }.query(self.querier)
    }
    pub fn proposals(
        &self,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryProposalsResponse, cosmwasm_std::StdError> {
        QueryProposalsRequest { pagination }.query(self.querier)
    }
    pub fn vote(
        &self,
        proposal_id: u64,
        voter: ::prost::alloc::string::String,
    ) -> Result<QueryVoteResponse, cosmwasm_std::StdError> {
        QueryVoteRequest { proposal_id, voter }.query(self.querier)
    }
    pub fn votes(
        &self,
        proposal_id: u64,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryVotesResponse, cosmwasm_std::StdError> {
        QueryVotesRequest {
            proposal_id,
            pagination,
        }
        .query(self.querier)
    }
    pub fn tally_result(
        &self,
        proposal_id: u64,
    ) -> Result<QueryTallyResultResponse, cosmwasm_std::StdError> {
        QueryTallyResultRequest { proposal_id }.query(self.querier)
    }
    pub fn censorships(
        &self,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryCensorshipsResponse, cosmwasm_std::StdError> {
        QueryCensorshipsRequest { pagination }.query(self.querier)
    }
    pub fn grants(
        &self,
        grantee: ::prost::alloc::string::String,
        msg_type_url: ::prost::alloc::string::String,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryGrantsResponse, cosmwasm_std::StdError> {
        QueryGrantsRequest {
            grantee,
            msg_type_url,
            pagination,
        }
        .query(self.querier)
    }
}
