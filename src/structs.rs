use std::error;
use std::fmt;

#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
pub struct ConsulAddress(String);

#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
pub struct ConsulID(String);

pub type ServiceID = ConsulID;
pub type NodeID = ConsulID;

#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
pub struct ConsulName(String);

pub type ServiceName = ConsulName;
pub type NodeName = ConsulName;

pub type ServicePort = u16;
pub type OptionalServicePort = Option<ServicePort>;
pub type ServiceTags = Vec<String>;

#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
pub struct Metadata(std::collections::HashMap<String, String>);

#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
pub struct TaggedAddresses(std::collections::HashMap<String, ConsulAddress>);

#[derive(Debug, Clone)]
pub struct InvalidName;

impl fmt::Display for InvalidName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

// This is important for other errors to wrap this one.
impl error::Error for InvalidName {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

impl ConsulID {
    pub fn from(name: &str) -> std::result::Result<ConsulID, InvalidName> {
        if name.contains("/") {
            Err(InvalidName)
        } else {
            Ok(ConsulID(String::from(name)))
        }
    }
    pub fn to_str(&self) -> &str {
        &self.0.as_str()
    }
}

impl ConsulName {
    pub fn from(name: &str) -> std::result::Result<ConsulName, InvalidName> {
        if name.contains("/") {
            Err(InvalidName)
        } else {
            Ok(ConsulName(String::from(name)))
        }
    }
    pub fn to_str(&self) -> &str {
        &self.0.as_str()
    }
}
