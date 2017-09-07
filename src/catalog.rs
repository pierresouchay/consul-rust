use std::collections::HashMap;

use {Client, QueryOptions, QueryMeta, WriteOptions, WriteMeta};

use errors::Result;
use request::get;

pub struct CatalogDeregistration {
    pub Node: String,
    pub Datacenter: Option<String>,
    pub CheckID: Option<String>,
    pub ServiceID: Option<String>,
}



pub trait Catalog {
    fn datacenters(&self) -> Result<(Vec<String>, QueryMeta)>;
    fn deregister(&self, &CatalogDeregistration, &WriteOptions) -> Result<((), WriteMeta)>;
}

impl Catalog for Client {
    fn datacenters(&self) -> Result<(Vec<String>, QueryMeta)> {
        get(
            "/v1/catalog/datacenters",
            &self.config,
            HashMap::new(),
            None,
        )
    }

    fn deregister(
        &self,
        dereg: &CatalogDeregistration,
        q: &WriteOptions,
    ) -> Result<((), WriteMeta)> {
        unimplemented!();
    }
}
