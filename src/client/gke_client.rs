use anyhow::Result;
use async_trait::async_trait;
use googapis::{
    google::container::v1::{
        cluster, cluster_manager_client::ClusterManagerClient, GetClusterRequest,
    },
    CERTIFICATES,
};
use gouth::Token;
use tonic::{
    metadata::MetadataValue,
    transport::{Certificate, Channel, ClientTlsConfig},
    Request,
};

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
#[async_trait]
pub trait GKEClientTrait {
    async fn fetch_cluster_status(
        &self,
        project: &String,
        location: &String,
        cluster: &String,
    ) -> Result<cluster::Status>;
}

pub struct GKEClient {}

impl GKEClient {
    pub fn new() -> GKEClient {
        GKEClient {}
    }
}

#[async_trait]
impl GKEClientTrait for GKEClient {
    async fn fetch_cluster_status(
        &self,
        project: &String,
        location: &String,
        cluster: &String,
    ) -> Result<cluster::Status> {
        let token = Token::new().map_err(|e| {
            let msg = format!("{}", e);
            anyhow::Error::new(e).context(msg)
        })?;

        let tls_config = ClientTlsConfig::new()
            .ca_certificate(Certificate::from_pem(CERTIFICATES))
            .domain_name("container.googleapis.com");

        let channel = Channel::from_static("https://container.googleapis.com")
            .tls_config(tls_config)?
            .connect()
            .await
            .map_err(|e| {
                let msg = format!("{}", e);
                anyhow::Error::new(e).context(msg)
            })?;

        let mut client =
            ClusterManagerClient::with_interceptor(channel, move |mut request: Request<()>| {
                let token = &*token.header_value().unwrap();
                let meta = MetadataValue::from_str(token).unwrap();
                request.metadata_mut().insert("authorization", meta);
                Ok(request)
            });

        let response = client
            .get_cluster(Request::new(GetClusterRequest {
                name: format!(
                    "projects/{}/locations/{}/clusters/{}",
                    project, location, cluster
                ),
                ..Default::default()
            }))
            .await
            .map_err(|e| {
                let msg = format!("{}", e.message());
                anyhow::Error::new(e).context(msg)
            })?;

        let status = match response.into_inner().status {
            0 => cluster::Status::Unspecified,
            1 => cluster::Status::Provisioning,
            2 => cluster::Status::Running,
            3 => cluster::Status::Reconciling,
            4 => cluster::Status::Stopping,
            5 => cluster::Status::Error,
            6 => cluster::Status::Degraded,
            _ => panic!("none status"),
        };

        Ok(status)
    }
}
