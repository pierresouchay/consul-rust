use std::collections::HashMap;

use async_trait::async_trait;

use crate::ConsulResult;

/// Request payload for the [ConnectIntentions::upsert_intention_by_name]
/// method.
#[derive(Debug, Serialize, Default)]
pub struct UpsertIntentionPayload {
    /// The type for the SourceName value. This can be only "consul" today to
    /// represent a Consul service. If not provided, this will be defaulted to
    /// "consul".
    #[serde(rename = "SourceType")]
    pub soucre_type: String,
    /// For an L4 intention this is required, and should be set to one of
    /// "allow" or "deny" for the action that should be taken if this intention
    /// matches a request.
    #[serde(rename = "Action")]
    pub action: IntentionAction,
    /// The name of the source service. This is required for an L4 intention.
    #[serde(rename = "Permissions")]
    pub permissions: Vec<IntentionPermision>,
    /// Description for the intention. This is not used by Consul, but is
    /// presented in API responses to assist tooling.
    #[serde(rename = "Description")]
    pub description: String,
}

/// Used to specify action type in [UpsertIntentionPayload].
#[derive(Debug, Serialize, Deserialize)]
pub enum IntentionAction {
    #[serde(rename = "allow")]
    Allow,
    #[serde(rename = "deny")]
    Deny,
}

impl Default for IntentionAction {
    fn default() -> Self {
        IntentionAction::Deny
    }
}

/// L7 attributes datatype.
///
/// This is used to specify the attributes for a L7 intention, as used by the
/// [ConnectIntentions::upsert_intention_by_name] method.
#[derive(Debug, Serialize)]
pub struct IntentionPermision {
    /// This is one of "allow" or "deny" for the action that should be taken if
    /// this permission matches a request.
    pub action: IntentionAction,
    /// A set of HTTP-specific authorization criteria
    pub http: IntentionHttpPermission,
}

/// HTTP-specific authorization criteria, as used in [IntentionPermision].
///
/// At most only one of PathExact, PathPrefix, or PathRegex may be configured.
#[derive(Debug, Serialize, Default)]
pub struct IntentionHttpPermission {
    /// Exact path to match on the HTTP request path.
    #[serde(rename = "PathExact")]
    pub path_exact: String,
    /// Path prefix to match on the HTTP request path.
    #[serde(rename = "PathPrefix")]
    pub path_prefix: String,
    /// Regular expression to match on the HTTP request path.
    #[serde(rename = "PathRegex")]
    pub path_regex: String,
    /// A list of HTTP methods for which this match applies. If unspecified all
    /// HTTP methods are matched. If provided the names must be a valid method.
    #[serde(rename = "Methods")]
    pub methods: Vec<String>,
    /// A set of criteria that can match on HTTP request headers. If more than
    /// one is configured all must match for the overall match to apply.
    #[serde(rename = "Header")]
    pub header: Vec<IntentionHttpHeaderPermission>,
}

/// A set of criteria that can match on HTTP request headers. If more than one
/// is configured all must match for the overall match to apply.
///
/// At most only one of `Exact`, `Prefix`, `Suffix`, `Regex`, or `Present` may
/// be configured.
#[derive(Debug, Serialize, Default)]
pub struct IntentionHttpHeaderPermission {
    /// Name of the header to match.
    #[serde(rename = "Name")]
    pub name: String,
    /// Match if the header with the given name is present with any value.
    #[serde(rename = "Present")]
    pub present: bool,
    /// Match if the header with the given name is this value.
    #[serde(rename = "Exact")]
    pub exact: String,
    /// Match if the header with the given name has this prefix.
    #[serde(rename = "Prefix")]
    pub prefix: String,
    /// Match if the header with the given name has this suffix.\
    #[serde(rename = "Suffix")]
    pub suffix: String,
    /// Match if the header with the given name matches this pattern.
    #[serde(rename = "Regex")]
    pub regex: String,
    /// Inverts the logic of the match.
    #[serde(rename = "Invert")]
    pub invert: bool,
}

/// Request payload for the [ConnectIntentions::create_intention_with_id]
/// method.
#[derive(Debug, Serialize, Default)]
pub struct CreateIntentionPayload {
    /// The source of the intention. For a `SourceType` of consul this is the
    /// name of a Consul service. The service does not need to be
    /// registered.
    #[serde(rename = "SourceName")]
    pub source_name: String,
    /// The namespace for the `SourceName` parameter.
    #[cfg(feature = "enterprise")]
    #[serde(rename = "SourceNS")]
    pub source_ns: String,
    ///  The destination of the intention. The intention destination is always a
    /// Consul service, unlike the source. The service does not need to be
    /// registered.
    #[serde(rename = "DestinationName")]
    pub destination_name: String,
    /// The namespace for the `DestinationName` parameter.
    #[cfg(feature = "enterprise")]
    #[serde(rename = "DestinationNS")]
    pub destination_ns: String,
    /// This is one of "allow" or "deny" for the action that should be taken if
    /// this intention matches a request.
    #[serde(rename = "Action")]
    pub action: IntentionAction,
    /// Description for the intention. This is not used by Consul, but is
    /// presented in API responses to assist tooling.
    #[serde(rename = "Description")]
    pub description: String,
    ///  Specifies arbitrary KV metadata pairs.
    #[serde(rename = "Meta")]
    pub meta: HashMap<String, String>,
}

/// Response payload for the [ConnectIntentions::read_intention_by_name]
#[derive(Debug, Deserialize)]
pub struct ReadIntentionByNameResponse {
    #[serde(rename = "Description")]
    pub description: String,
    #[cfg(feature = "enterprise")]
    #[serde(rename = "SourceNS")]
    pub source_ns: String,
    #[serde(rename = "SourceName")]
    pub source_name: String,
    #[cfg(feature = "enterprise")]
    #[serde(rename = "DestinationNS")]
    pub destination_ns: String,
    #[serde(rename = "DestinationName")]
    pub destination_name: String,
    #[serde(rename = "SourceType")]
    pub source_type: String,
    #[serde(rename = "Action")]
    pub action: IntentionAction,
    #[serde(rename = "Meta")]
    pub meta: HashMap<String, String>,
    #[serde(rename = "Precedence")]
    pub precedence: u64,
    #[serde(rename = "CreateIndex")]
    pub create_index: u64,
    #[serde(rename = "ModifyIndex")]
    pub modify_index: u64,
}

/// This trait provides implementations for the  `/connect/intentions` endpoint.
///
/// The /connect/intentions endpoint provide tools for managing intentions.
///
/// See the [API documentation](https://www.consul.io/api-docs/connect/intentions) for more information.
#[async_trait]
pub trait ConnectIntentions {
    /// This method creates a new intention and returns true if it was created
    /// successfully.
    ///
    /// The name and destination pair must be unique. If another intention
    /// matches the name and destination, the creation will replace the previous
    /// intention.
    ///
    /// See the [API documentation](https://www.consul.io/api-docs/connect/intentions#upsert-intention-by-name) for more information.
    async fn upsert_intention_by_name<S: ToString>(
        source: S,
        desintation: S,
        payload: UpsertIntentionPayload,
    ) -> ConsulResult<bool>;
    /// This method reads a specific intention by its unique source and
    /// destination.
    async fn read_intention_by_name(
        source: String,
        destination: String,
    ) -> ConsulResult<ReadIntentionByNameResponse>;
    /// This method lists all intentions.
    async fn list_intentions(
        filter: Option<String>,
    ) -> ConsulResult<Vec<ReadIntentionByNameResponse>>;
    /// This method deletes a specific intention by its unique source and
    /// destination.
    async fn delete_intention_by_name<S: ToString>(source: S, destination: S) -> ConsulResult<()>;
}
