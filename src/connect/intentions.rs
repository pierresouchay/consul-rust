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

/// Used to specify action type in
/// [UpsertIntentionPayload].
#[derive(Debug, Default, Serialize)]
pub enum IntentionAction {
    #[serde(rename = "allow")]
    Allow,
    #[serde(rename = "deny")]
    #[default]
    Deny,
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
    pub methods: Vec<String>,
    /// A set of criteria that can match on HTTP request headers. If more than
    /// one is configured all must match for the overall match to apply.
    pub header: Vec<IntentionHttpHeaderPermission>,
}

/// A set of criteria that can match on HTTP request headers. If more than one
/// is configured all must match for the overall match to apply.
#[derive(Debug, Serialize, Default)]
pub struct IntentionHttpHeaderPermission {
	pub name: String,
	pub present: bool,
	pub exact: String,
	pub prefix: String,
	pub suffix: String,
	pub regex: String,
	pub invert: bool
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
    async fn upsert_intention_by_name<S: ToString>(source: S, desintation: S) -> ConsulResult<()>;
}
