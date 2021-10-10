pub mod result;

use anyhow::Result;
use serde::ser::{Serialize, SerializeMap, Serializer};
use serde::Deserialize;
use std::fmt;

use crate::client::gke_client::GKEClient;
use crate::operator::gke_cluster_status::GKEClusterStatusOperator;
use crate::spec::result::SpecResult;

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Clone)]
pub enum ClusterStatus {
    Unspecified,
    Provisioning,
    Running,
    Reconciling,
    Stopping,
    Error,
    Degraded,
}

impl Serialize for ClusterStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{:?}", self))
    }
}

impl fmt::Display for ClusterStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)?;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Clone)]
#[serde(tag = "operator")]
pub enum Spec {
    GKEClusterStatus {
        project: String,
        location: String,
        cluster: String,
        status: Vec<ClusterStatus>,
    },
}

impl Serialize for Spec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::GKEClusterStatus {
                project,
                location,
                cluster,
                status,
            } => {
                let mut map = serializer.serialize_map(Some(5))?;
                map.serialize_entry("operator", "GKEClusterStatus")?;
                map.serialize_entry("project", project)?;
                map.serialize_entry("location", location)?;
                map.serialize_entry("cluster", cluster)?;
                map.serialize_entry("status", status)?;
                map.end()
            }
        }
    }
}

impl Spec {
    pub async fn check(&self) -> Result<SpecResult> {
        match self {
            Self::GKEClusterStatus {
                project,
                location,
                cluster,
                status,
            } => {
                GKEClusterStatusOperator::new(
                    project.clone(),
                    location.clone(),
                    cluster.clone(),
                    status.clone(),
                    Box::new(GKEClient::new()),
                )
                .check()
                .await
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::spec::*;
    use rstest::*;

    #[rstest]
    #[case(
        ClusterStatus::Unspecified,
        format!(
r#"---
Unspecified
"#
        )
    )]
    #[case(
        ClusterStatus::Provisioning,
        format!(
r#"---
Provisioning
"#
        )
    )]
    #[case(
        ClusterStatus::Running,
        format!(
r#"---
Running
"#
        )
    )]
    #[case(
        ClusterStatus::Reconciling,
        format!(
r#"---
Reconciling
"#
        )
    )]
    #[case(
        ClusterStatus::Stopping,
        format!(
r#"---
Stopping
"#
        )
    )]
    #[case(
        ClusterStatus::Error,
        format!(
r#"---
Error
"#
        )
    )]
    #[case(
        ClusterStatus::Degraded,
        format!(
r#"---
Degraded
"#
        )
    )]
    #[trace]
    fn test_cluster_status_serialize(
        #[case] cluster_status: ClusterStatus,
        #[case] expected: String,
    ) {
        assert_eq!(serde_yaml::to_string(&cluster_status).unwrap(), expected);
    }

    #[rstest]
    #[case(
        Spec::GKEClusterStatus {
            project: format!("project-001"),
            location: format!("location-001"),
            cluster: format!("cluster-001"),
            status: vec![ClusterStatus::Provisioning],
        },
        format!(
r#"---
operator: GKEClusterStatus
project: project-001
location: location-001
cluster: cluster-001
status:
  - Provisioning
"#
        )
    )]
    #[case(
        Spec::GKEClusterStatus {
            project: format!("project-002"),
            location: format!("location-002"),
            cluster: format!("cluster-002"),
            status: vec![ClusterStatus::Provisioning, ClusterStatus::Running],
        },
        format!(
r#"---
operator: GKEClusterStatus
project: project-002
location: location-002
cluster: cluster-002
status:
  - Provisioning
  - Running
"#
        )
    )]
    #[trace]
    fn test_spec_serialize(#[case] spec: Spec, #[case] expected: String) {
        assert_eq!(serde_yaml::to_string(&spec).unwrap(), expected);
    }
}
