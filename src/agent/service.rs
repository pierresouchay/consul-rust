use async_trait::async_trait;

use crate::{sealed::Sealed, Client};

pub struct Service {}

/// The `/agent/service` endpoints interact with services on the local agent in
/// Consul. These should not be confused with services in the catalog.
#[async_trait]
pub trait AgentServices: Sealed {
    async fn list_local_services(&self) -> Result<Vec<Service>, ()>;
    async fn get_local_service_config(&self) -> Result<(), ()>;
    async fn get_local_service_health(&self) -> Result<(), ()>;
    async fn get_local_service_health_by_id(&self) -> Result<(), ()>;
    async fn register_service(&self) -> Result<(), ()>;
}

#[async_trait]
impl AgentServices for Client {
    async fn list_local_services(&self) -> Result<Vec<Service>, ()> {
        todo!()
    }
    async fn get_local_service_config(&self) -> Result<(), ()> {
        todo!()
    }
    async fn get_local_service_health(&self) -> Result<(), ()> {
        todo!()
    }
    async fn get_local_service_health_by_id(&self) -> Result<(), ()> {
        todo!()
    }
    async fn register_service(&self) -> Result<(), ()> {
        todo!()
    }
}
