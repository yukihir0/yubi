use anyhow::Result;
use googapis::{
    google::container::v1::{
        cluster::Status, cluster_manager_client::ClusterManagerClient, GetClusterRequest,
    },
    CERTIFICATES,
};
use gouth::Token;
use serde::{Deserialize, Serialize};
use std::fmt;
use tonic::{
    metadata::MetadataValue,
    transport::{Certificate, Channel, ClientTlsConfig},
    Request,
};

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "operator")]
pub enum Spec {
    GKEClusterStatus {
        project: String,
        location: String,
        cluster: String,
    },
}

impl fmt::Display for Spec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::GKEClusterStatus {
                project: _,
                location: _,
                cluster: _,
            } => {
                write!(f, "GKEClusterStatus")
            }
        }
    }
}

impl Spec {
    pub async fn check(&self) -> Result<SpecResponse> {
        match self {
            Self::GKEClusterStatus {
                project,
                location,
                cluster,
            } => {
                self.check_gke_cluster_status(project, location, cluster)
                    .await
            }
        }
    }

    async fn check_gke_cluster_status(
        &self,
        project: &String,
        location: &String,
        cluster: &String,
    ) -> Result<SpecResponse> {
        let token = Token::new()?;

        let tls_config = ClientTlsConfig::new()
            .ca_certificate(Certificate::from_pem(CERTIFICATES))
            .domain_name("container.googleapis.com");

        let channel = Channel::from_static("https://container.googleapis.com")
            .tls_config(tls_config)?
            .connect()
            .await?;

        let mut service =
            ClusterManagerClient::with_interceptor(channel, move |mut req: Request<()>| {
                let token = &*token.header_value().unwrap();
                let meta = MetadataValue::from_str(token).unwrap();
                req.metadata_mut().insert("authorization", meta);
                Ok(req)
            });

        let response = service
            .get_cluster(Request::new(GetClusterRequest {
                name: format!(
                    "projects/{}/locations/{}/clusters/{}",
                    project, location, cluster
                ),
                ..Default::default()
            }))
            .await?;

        let status = match response.into_inner().status {
            0 => Status::Unspecified,
            1 => Status::Provisioning,
            2 => Status::Running,
            3 => Status::Reconciling,
            4 => Status::Stopping,
            5 => Status::Error,
            6 => Status::Degraded,
            _ => panic!("none status"),
        };

        match status {
            Status::Running => Ok(SpecResponse::Success {
                message: format!("{} is running", cluster),
            }),
            _ => Ok(SpecResponse::Fail {
                message: format!("{} is not running", cluster),
            }),
        }
    }
}

#[derive(Debug)]
pub enum SpecResponse {
    Success { message: String },
    Fail { message: String },
}

impl fmt::Display for SpecResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Success { message } => {
                write!(f, "{}", message)
            }
            Self::Fail { message } => {
                write!(f, "{}", message)
            }
        }
    }
}

impl SpecResponse {
    pub fn status(&self) -> String {
        match self {
            Self::Success { message: _ } => {
                format!("success")
            }
            Self::Fail { message: _ } => {
                format!("fail")
            }
        }
    }
}
