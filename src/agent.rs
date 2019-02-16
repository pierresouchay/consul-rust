use std::collections::HashMap;
use std::fmt;

use error::*;
use health::HealthCheckDefinition;
use request::{Method, Request, StatusCode};

use Client;

// Types
#[serde(default, rename_all = "PascalCase")]
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct AgentCheck {
    pub node: String,
    #[serde(rename = "CheckID")]
    pub check_id: String,
    pub name: String,
    pub status: String,
    pub notes: String,
    pub output: String,
    #[serde(rename = "ServiceID")]
    pub service_id: String,
    pub service_name: String,
    pub definition: HealthCheckDefinition,
}

#[serde(default, rename_all = "PascalCase")]
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct AgentWeights {
    pub passing: isize,
    pub warning: isize,
}

#[serde(default, rename_all = "PascalCase")]
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct AgentService {
    pub kind: String,
    #[serde(rename = "ID")]
    pub id: String,
    pub service: String,
    pub tags: Vec<String>,
    pub meta: HashMap<String, String>,
    pub port: isize,
    pub address: String,
    pub weights: AgentWeights,
    pub enable_tag_override: bool,
    pub create_index: u64,
    pub modify_index: u64,
    pub proxy_destination: String,
    pub connect: AgentServiceConnect,
}

#[serde(default, rename_all = "PascalCase")]
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct AgentServiceConnect {
    pub native: bool,
    pub proxy: AgentServiceConnectProxy,
}

#[serde(default, rename_all = "PascalCase")]
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct AgentServiceConnectProxy {
    pub destination_service_name: String,
    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "DestinationServiceID"
    )]
    pub destination_service_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_service_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_service_port: Option<isize>,
    // pub config: XXX: This is an interface in go
    // pub upstream: XXX: Another structure containing an interface
}

#[serde(default, rename_all = "PascalCase")]
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct AgentMember {
    pub name: String,
    pub addr: String,
    pub port: u16,
    pub tags: HashMap<String, String>,
    pub status: isize,
    pub protocol_min: u8,
    pub protocol_max: u8,
    pub protocol_cur: u8,
    pub delegate_min: u8,
    pub delegate_max: u8,
    pub delegate_cur: u8,
}

#[serde(default, rename_all = "PascalCase")]
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct AgentServiceRegistration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ID")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<isize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_tag_override: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weights: Option<AgentWeights>,
    pub check: AgentServiceCheck,
    pub checks: AgentServiceChecks,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_destination: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect: Option<AgentServiceConnect>,
}

#[serde(default, rename_all = "PascalCase")]
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct AgentCheckRegistration {
    #[serde(skip_serializing_if = "Option::is_none", rename = "ID")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceID")]
    pub service_id: Option<String>,
    #[serde(flatten)]
    pub check: AgentServiceCheck,
}

#[serde(default, rename_all = "PascalCase")]
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct AgentServiceCheck {
    #[serde(skip_serializing_if = "Option::is_none", rename = "CheckID")]
    pub check_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "DockerContainerID")]
    pub docker_container_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shell: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "TTL")]
    pub ttl: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "HTTP")]
    pub http: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "TCP")]
    pub tcp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_skip_verify: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "GRPC")]
    pub grpc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "GRPCUseTLS")]
    pub grpc_use_tls: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deregister_critical_service_after: Option<String>,
}
type AgentServiceChecks = Vec<AgentServiceCheck>;

#[serde(default, rename_all = "PascalCase")]
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct AgentToken {
    pub token: String,
}

#[serde(default, rename_all = "PascalCase")]
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct AgentMetrics {
    pub timestamp: String,
    pub gauges: Vec<GaugeValue>,
    pub points: Vec<PointValue>,
    pub counters: Vec<SampledValue>,
    pub samples: Vec<SampledValue>,
}

#[serde(default, rename_all = "PascalCase")]
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct GaugeValue {
    pub name: String,
    pub value: f32,
    pub labels: HashMap<String, String>,
}

#[serde(default, rename_all = "PascalCase")]
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct PointValue {
    pub name: String,
    pub value: Vec<f32>,
}

#[serde(default, rename_all = "PascalCase")]
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct SampledValue {
    pub name: String,
    pub count: isize,
    pub sum: f64,
    pub min: f64,
    pub max: f64,
    pub mean: f64,
    pub stddev: f64,
    pub labels: HashMap<String, String>,
}

#[serde(default, rename_all = "PascalCase")]
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct AgentAuthorizeParams {
    pub target: String,
    #[serde(rename = "ClientCertURI")]
    pub client_cert_uri: String,
    pub client_cert_serial: String,
}

#[serde(default, rename_all = "PascalCase")]
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct AgentAuthorize {
    pub authorized: bool,
    pub reason: String,
}

#[serde(default, rename_all = "PascalCase")]
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct CaRootList {
    #[serde(rename = "ActiveRootID")]
    pub active_root_id: String,
    pub trust_domain: String,
    pub roots: Vec<CaRoot>,
}

#[serde(default, rename_all = "PascalCase")]
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct CaRoot {
    #[serde(rename = "ID")]
    pub id: String,
    pub name: String,
    pub root_cert: String,
    pub active: bool,
    pub create_index: u64,
    pub modify_index: u64,
}

#[serde(default, rename_all = "PascalCase")]
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct LeafCert {
    pub serial_number: String,
    #[serde(skip_serializing_if = "Option::is_none", rename = "CertPEM")]
    pub cert_pem: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "PrivateKeyPEM")]
    pub private_key_pem: Option<String>,
    pub service: String,
    #[serde(rename = "ServiceURI")]
    pub service_uri: String,
    pub valid_after: String,
    pub valid_before: String,
    pub create_index: u64,
    pub modify_index: u64,
}

// API
#[serde(default)]
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct AgentOptions {
    wan: Option<bool>,
    segment: Option<String>,
}

#[serde(default)]
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct JoinOptions {
    wan: Option<bool>,
}

#[serde(default)]
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct MaintenanceOptions {
    reason: Option<String>,
}
type ServiceMaintenanceOptions = MaintenanceOptions;

#[serde(default)]
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct CheckOptions {
    note: Option<String>,
}
type CheckPassOptions = CheckOptions;
type CheckWarnOptions = CheckOptions;
type CheckFailOptions = CheckOptions;

#[derive(Clone, Debug)]
pub enum HealthStatus {
    Passing,
    Warning,
    Critical,
}

impl fmt::Display for HealthStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HealthStatus::Passing => write!(f, "passing"),
            HealthStatus::Warning => write!(f, "warning"),
            HealthStatus::Critical => write!(f, "critical"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum TokenType {
    AclToken,
    AclAgentToken,
    AclAgentMasterToken,
    AclReplicationToken,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenType::AclToken => write!(f, "acl_token"),
            TokenType::AclAgentToken => write!(f, "acl_agent_token"),
            TokenType::AclAgentMasterToken => write!(f, "acl_agent_master_token"),
            TokenType::AclReplicationToken => write!(f, "acl_replication_token"),
        }
    }
}

pub trait Agent {
    // Agent
    fn members(&self, options: Option<&AgentOptions>) -> Result<Vec<AgentMember>>;
    fn agent(&self) -> Result<AgentMember>;
    fn reload(&self) -> Result<()>;
    fn maintenance(&self, enable: bool, options: Option<&MaintenanceOptions>) -> Result<()>;
    fn metrics(&self) -> Result<AgentMetrics>;
    fn join(&self, address: &str, options: Option<&JoinOptions>) -> Result<()>;
    fn leave(&self) -> Result<()>;
    fn force_leave(&self, node: &str) -> Result<()>;
    fn token(&self, token_type: TokenType, token: &str) -> Result<()>;
    // TODO: https://www.consul.io/api/agent.html#stream-logs
    // Checks
    fn checks(&self) -> Result<HashMap<String, AgentCheck>>;
    fn check_register(&self, check: &AgentCheckRegistration) -> Result<()>;
    fn check_deregister(&self, check_id: &str) -> Result<()>;
    fn check_pass(&self, check_id: &str, options: Option<&CheckPassOptions>) -> Result<()>;
    fn check_warn(&self, check_id: &str, options: Option<&CheckWarnOptions>) -> Result<()>;
    fn check_fail(&self, check_id: &str, options: Option<&CheckFailOptions>) -> Result<()>;
    fn check_update(&self, check_id: &str, status: HealthStatus, output: &str) -> Result<()>;
    // Services
    fn services(&self) -> Result<HashMap<String, AgentService>>;
    // TODO: blocking
    fn service(&self, service_id: &str) -> Result<AgentService>;
    // TODO: Needs access to response status
    // fn agent_service_health(&self, service_name: &str) -> Result<HashMap<HealthStatus, Vec<AgentService>>>;
    // TODO: Needs access to response status
    // fn agent_service_health_by_id(&self, service_id: &str) -> Result<HashMap<HealthStatus, Vec<AgentService>>>;
    fn service_register(&self, service: &AgentServiceRegistration) -> Result<()>;
    fn service_deregister(&self, service_id: &str) -> Result<()>;
    fn service_maintenance(
        &self,
        service_id: &str,
        enable: bool,
        options: Option<ServiceMaintenanceOptions>,
    ) -> Result<()>;
    // Connect
    fn authorize(&self, auth: &AgentAuthorizeParams) -> Result<AgentAuthorize>;
    // TODO: blocking
    fn connect_ca_roots(&self) -> Result<CaRootList>;
    // TODO: blocking
    fn connect_ca_leaf(&self, service: &str) -> Result<LeafCert>;
}

impl Agent for Client {
    /// https://www.consul.io/api/agent.html#list-members
    fn members(&self, options: Option<&AgentOptions>) -> Result<Vec<AgentMember>> {
        let mut params: HashMap<String, String> = HashMap::new();
        if let Some(options) = options {
            if let Some(wan) = options.wan {
                params.insert(String::from("wan"), wan.to_string());
            }
            if let Some(segment) = &options.segment {
                params.insert(String::from("segment"), segment.to_string());
            }
        }
        let mut r = Request::new_with_params(&self, Method::GET, "agent/members", params).send()?;
        if r.status() != StatusCode::OK {
            Err(ErrorKind::UnexpectedResponse(r.text()?))?
        }
        Ok(r.json().context(ErrorKind::InvalidResponse)?)
    }

    /// https://www.consul.io/api/agent.html#read-configuration
    fn agent(&self) -> Result<AgentMember> {
        let mut r = Request::new(&self, Method::GET, "agent/self").send()?;
        if r.status() != StatusCode::OK {
            Err(ErrorKind::UnexpectedResponse(r.text()?))?
        }
        Ok(r.json().context(ErrorKind::InvalidResponse)?)
    }

    /// https://www.consul.io/api/agent.html#reload-agent
    fn reload(&self) -> Result<()> {
        let mut r = Request::new(&self, Method::PUT, "agent/reload").send()?;
        if r.status() != StatusCode::OK {
            Err(ErrorKind::UnexpectedResponse(r.text()?))?
        }
        Ok(())
    }

    /// https://www.consul.io/api/agent.html#enable-maintenance-mode
    fn maintenance(&self, enable: bool, options: Option<&MaintenanceOptions>) -> Result<()> {
        let mut params: HashMap<String, String> = HashMap::new();
        params.insert(String::from("enable"), enable.to_string());
        if let Some(options) = options {
            if let Some(reason) = &options.reason {
                params.insert(String::from("reason"), reason.to_string());
            }
        }
        let mut r =
            Request::new_with_params(&self, Method::PUT, "agent/maintenance", params).send()?;
        if r.status() != StatusCode::OK {
            Err(ErrorKind::UnexpectedResponse(r.text()?))?
        }
        Ok(())
    }

    /// https://www.consul.io/api/agent.html#view-metrics
    fn metrics(&self) -> Result<AgentMetrics> {
        let mut r = Request::new(&self, Method::GET, "agent/metrics").send()?;
        if r.status() != StatusCode::OK {
            Err(ErrorKind::UnexpectedResponse(r.text()?))?
        }
        Ok(r.json().context(ErrorKind::InvalidResponse)?)
    }

    /// https://www.consul.io/api/agent.html#join-agent
    fn join(&self, address: &str, options: Option<&JoinOptions>) -> Result<()> {
        let mut params: HashMap<String, String> = HashMap::new();
        if let Some(options) = options {
            if let Some(wan) = options.wan {
                params.insert(String::from("wan"), wan.to_string());
            }
        }
        let mut r = Request::new_with_params(
            &self,
            Method::PUT,
            &format!("agent/join/{}", address),
            params,
        )
        .send()?;
        if r.status() != StatusCode::OK {
            Err(ErrorKind::UnexpectedResponse(r.text()?))?
        }
        Ok(())
    }

    /// https://www.consul.io/api/agent.html#graceful-leave-and-shutdown
    fn leave(&self) -> Result<()> {
        let mut r = Request::new(&self, Method::PUT, "agent/leave").send()?;
        if r.status() != StatusCode::OK {
            Err(ErrorKind::UnexpectedResponse(r.text()?))?
        }
        Ok(())
    }

    /// https://www.consul.io/api/agent.html#force-leave-and-shutdown
    fn force_leave(&self, node: &str) -> Result<()> {
        let mut r =
            Request::new(&self, Method::PUT, &format!("agent/force-leave/{}", node)).send()?;
        if r.status() != StatusCode::OK {
            Err(ErrorKind::UnexpectedResponse(r.text()?))?
        }
        Ok(())
    }

    /// https://www.consul.io/api/agent.html#update-acl-tokens
    fn token(&self, token_type: TokenType, token: &str) -> Result<()> {
        let mut body: HashMap<String, String> = HashMap::new();
        body.insert(String::from("Token"), token.to_string());
        let mut r = Request::new(&self, Method::PUT, &format!("agent/token/{}", token_type))
            .json(&body)
            .send()?;
        if r.status() != StatusCode::OK {
            Err(ErrorKind::UnexpectedResponse(r.text()?))?
        }
        Ok(())
    }

    /// https://www.consul.io/api/agent/check.html#list-checks
    fn checks(&self) -> Result<HashMap<String, AgentCheck>> {
        let mut r = Request::new(&self, Method::GET, "agent/checks").send()?;
        if r.status() != StatusCode::OK {
            Err(ErrorKind::UnexpectedResponse(r.text()?))?
        }
        Ok(r.json().context(ErrorKind::InvalidResponse)?)
    }

    /// https://www.consul.io/api/agent/check.html#register-check
    fn check_register(&self, check: &AgentCheckRegistration) -> Result<()> {
        let mut r = Request::new(&self, Method::PUT, "agent/check/register")
            .json(check)
            .send()?;
        if r.status() != StatusCode::OK {
            Err(ErrorKind::UnexpectedResponse(r.text()?))?
        }
        Ok(())
    }

    /// https://www.consul.io/api/agent/check.html#deregister-check
    fn check_deregister(&self, check_id: &str) -> Result<()> {
        let mut r = Request::new(
            &self,
            Method::PUT,
            &format!("agent/check/deregister/{}", check_id),
        )
        .send()?;
        if r.status() != StatusCode::OK {
            Err(ErrorKind::UnexpectedResponse(r.text()?))?
        }
        Ok(())
    }

    /// https://www.consul.io/api/agent/check.html#ttl-check-pass
    fn check_pass(&self, check_id: &str, options: Option<&CheckPassOptions>) -> Result<()> {
        let mut params: HashMap<String, String> = HashMap::new();
        if let Some(options) = options {
            if let Some(note) = &options.note {
                params.insert(String::from("note"), note.to_string());
            }
        }
        let mut r = Request::new_with_params(
            &self,
            Method::PUT,
            &format!("agent/check/pass/{}", check_id),
            params,
        )
        .send()?;
        if r.status() != StatusCode::OK {
            Err(ErrorKind::UnexpectedResponse(r.text()?))?
        }
        Ok(())
    }

    /// https://www.consul.io/api/agent/check.html#ttl-check-warn
    fn check_warn(&self, check_id: &str, options: Option<&CheckWarnOptions>) -> Result<()> {
        let mut params: HashMap<String, String> = HashMap::new();
        if let Some(options) = options {
            if let Some(note) = &options.note {
                params.insert(String::from("note"), note.to_string());
            }
        }
        let mut r = Request::new_with_params(
            &self,
            Method::PUT,
            &format!("agent/check/warn/{}", check_id),
            params,
        )
        .send()?;
        if r.status() != StatusCode::OK {
            Err(ErrorKind::UnexpectedResponse(r.text()?))?
        }
        Ok(())
    }

    /// https://www.consul.io/api/agent/check.html#ttl-check-fail
    fn check_fail(&self, check_id: &str, options: Option<&CheckFailOptions>) -> Result<()> {
        let mut params: HashMap<String, String> = HashMap::new();
        if let Some(options) = options {
            if let Some(note) = &options.note {
                params.insert(String::from("note"), note.to_string());
            }
        }
        let mut r = Request::new_with_params(
            &self,
            Method::PUT,
            &format!("agent/check/fail/{}", check_id),
            params,
        )
        .send()?;
        if r.status() != StatusCode::OK {
            Err(ErrorKind::UnexpectedResponse(r.text()?))?
        }
        Ok(())
    }

    /// https://www.consul.io/api/agent/check.html#ttl-check-update
    fn check_update(&self, check_id: &str, status: HealthStatus, output: &str) -> Result<()> {
        let mut json: HashMap<String, String> = HashMap::new();
        json.insert(String::from("status"), status.to_string());
        json.insert(String::from("output"), output.to_string());
        let mut r = Request::new(
            &self,
            Method::PUT,
            &format!("agent/check/update/{}", check_id),
        )
        .json(&json)
        .send()?;
        if r.status() != StatusCode::OK {
            Err(ErrorKind::UnexpectedResponse(r.text()?))?
        }
        Ok(())
    }

    /// https://www.consul.io/api/agent/service.html#list-services
    fn services(&self) -> Result<HashMap<String, AgentService>> {
        let mut r = Request::new(&self, Method::GET, "agent/services").send()?;
        if r.status() != StatusCode::OK {
            Err(ErrorKind::UnexpectedResponse(r.text()?))?
        }
        Ok(r.json().context(ErrorKind::InvalidResponse)?)
    }

    /// https://www.consul.io/api/agent/service.html#get-service-configuration
    fn service(&self, service_id: &str) -> Result<AgentService> {
        let mut r =
            Request::new(&self, Method::GET, &format!("agent/service/{}", service_id)).send()?;
        if r.status() != StatusCode::OK {
            Err(ErrorKind::UnexpectedResponse(r.text()?))?
        }
        Ok(r.json().context(ErrorKind::InvalidResponse)?)
    }

    /// https://www.consul.io/api/agent/service.html#register-service
    fn service_register(&self, service: &AgentServiceRegistration) -> Result<()> {
        let mut r = Request::new(&self, Method::PUT, "agent/service/register")
            .json(service)
            .send()?;
        if r.status() != StatusCode::OK {
            Err(ErrorKind::UnexpectedResponse(r.text()?))?
        }
        Ok(())
    }

    /// https://www.consul.io/api/agent/service.html#deregister-service
    fn service_deregister(&self, service_id: &str) -> Result<()> {
        let mut r = Request::new(
            &self,
            Method::PUT,
            &format!("agent/service/deregister/{}", service_id),
        )
        .send()?;
        if r.status() != StatusCode::OK {
            Err(ErrorKind::UnexpectedResponse(r.text()?))?
        }
        Ok(())
    }

    /// https://www.consul.io/api/agent/service.html#enable-maintenance-mode
    fn service_maintenance(
        &self,
        service_id: &str,
        enable: bool,
        options: Option<ServiceMaintenanceOptions>,
    ) -> Result<()> {
        let mut params: HashMap<String, String> = HashMap::new();
        params.insert(String::from("enable"), enable.to_string());
        if let Some(options) = options {
            if let Some(reason) = &options.reason {
                params.insert(String::from("reason"), reason.to_string());
            }
        }
        let mut r = Request::new_with_params(
            &self,
            Method::PUT,
            &format!("agent/service/maintenance/{}", service_id),
            params,
        )
        .send()?;
        if r.status() != StatusCode::OK {
            Err(ErrorKind::UnexpectedResponse(r.text()?))?
        }
        Ok(())
    }

    /// https://www.consul.io/api/agent/connect.html#authorize
    fn authorize(&self, auth: &AgentAuthorizeParams) -> Result<AgentAuthorize> {
        let mut r = Request::new(&self, Method::POST, "agent/connect/authorize")
            .json(auth)
            .send()?;
        if r.status() != StatusCode::OK {
            Err(ErrorKind::UnexpectedResponse(r.text()?))?
        }
        Ok(r.json().context(ErrorKind::InvalidResponse)?)
    }

    /// https://www.consul.io/api/agent/connect.html#certificate-authority-ca-roots
    fn connect_ca_roots(&self) -> Result<CaRootList> {
        let mut r = Request::new(&self, Method::GET, "agent/connect/ca/roots").send()?;
        if r.status() != StatusCode::OK {
            Err(ErrorKind::UnexpectedResponse(r.text()?))?
        }
        Ok(r.json().context(ErrorKind::InvalidResponse)?)
    }

    /// https://www.consul.io/api/agent/connect.html#service-leaf-certificate
    fn connect_ca_leaf(&self, service: &str) -> Result<LeafCert> {
        let mut r = Request::new(
            &self,
            Method::GET,
            &format!("agent/connect/ca/leaf/{}", service),
        )
        .send()?;
        if r.status() != StatusCode::OK {
            Err(ErrorKind::UnexpectedResponse(r.text()?))?
        }
        Ok(r.json().context(ErrorKind::InvalidResponse)?)
    }
}
