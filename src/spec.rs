pub mod cluster_status;
pub mod node_pool_status;
pub mod result;
pub mod sql_instance_status;

use anyhow::Result;
use serde::ser::{Serialize, SerializeMap, Serializer};
use serde::Deserialize;

use crate::client::cloud_sql_client::CloudSqlClient;
use crate::client::gke_client::GKEClient;
use crate::operator::cloud_sql_instance_status_operator::CloudSqlInstanceStatusOperator;
use crate::operator::gke_cluster_status_operator::GKEClusterStatusOperator;
use crate::operator::gke_node_pool_status_operator::GKENodePoolStatusOperator;
use crate::spec::cluster_status::ClusterStatus;
use crate::spec::node_pool_status::NodePoolStatus;
use crate::spec::result::SpecResult;
use crate::spec::sql_instance_status::SqlInstanceStatus;

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Clone)]
#[serde(tag = "operator")]
pub enum Spec {
    GKEClusterStatus {
        project: String,
        location: String,
        cluster: String,
        status: Vec<ClusterStatus>,
    },
    GKENodePoolStatus {
        project: String,
        location: String,
        cluster: String,
        node_pool: String,
        status: Vec<NodePoolStatus>,
    },
    CloudSqlInstanceStatus {
        project: String,
        instance: String,
        status: Vec<SqlInstanceStatus>,
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
            Self::GKENodePoolStatus {
                project,
                location,
                cluster,
                node_pool,
                status,
            } => {
                let mut map = serializer.serialize_map(Some(6))?;
                map.serialize_entry("operator", "GKENodePoolStatus")?;
                map.serialize_entry("project", project)?;
                map.serialize_entry("location", location)?;
                map.serialize_entry("cluster", cluster)?;
                map.serialize_entry("node_pool", node_pool)?;
                map.serialize_entry("status", status)?;
                map.end()
            }
            Self::CloudSqlInstanceStatus {
                project,
                instance,
                status,
            } => {
                let mut map = serializer.serialize_map(Some(4))?;
                map.serialize_entry("operator", "CloudSqlInstanceStatus")?;
                map.serialize_entry("project", project)?;
                map.serialize_entry("instance", instance)?;
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
            Self::GKENodePoolStatus {
                project,
                location,
                cluster,
                node_pool,
                status,
            } => {
                GKENodePoolStatusOperator::new(
                    project.clone(),
                    location.clone(),
                    cluster.clone(),
                    node_pool.clone(),
                    status.clone(),
                    Box::new(GKEClient::new()),
                )
                .check()
                .await
            }
            Self::CloudSqlInstanceStatus {
                project,
                instance,
                status,
            } => {
                CloudSqlInstanceStatusOperator::new(
                    project.clone(),
                    instance.clone(),
                    status.clone(),
                    Box::new(CloudSqlClient::new()),
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
    #[case(
        Spec::GKENodePoolStatus {
            project: format!("project-001"),
            location: format!("location-001"),
            cluster: format!("cluster-001"),
            node_pool: format!("node_pool-001"),
            status: vec![NodePoolStatus::Provisioning],
        },
        format!(
r#"---
operator: GKENodePoolStatus
project: project-001
location: location-001
cluster: cluster-001
node_pool: node_pool-001
status:
  - Provisioning
"#
        )
    )]
    #[case(
        Spec::GKENodePoolStatus {
            project: format!("project-002"),
            location: format!("location-002"),
            cluster: format!("cluster-002"),
            node_pool: format!("node_pool-002"),
            status: vec![NodePoolStatus::Provisioning, NodePoolStatus::Running],
        },
        format!(
r#"---
operator: GKENodePoolStatus
project: project-002
location: location-002
cluster: cluster-002
node_pool: node_pool-002
status:
  - Provisioning
  - Running
"#
        )
    )]
    #[case(
        Spec::CloudSqlInstanceStatus {
            project: format!("project-001"),
            instance: format!("sql_instance-001"),
            status: vec![SqlInstanceStatus::Unspecified],
        },
        format!(
r#"---
operator: CloudSqlInstanceStatus
project: project-001
instance: sql_instance-001
status:
  - Unspecified
"#
        )
    )]
    #[case(
        Spec::CloudSqlInstanceStatus {
            project: format!("project-002"),
            instance: format!("sql_instance-002"),
            status: vec![SqlInstanceStatus::Unspecified, SqlInstanceStatus::Runnable],
        },
        format!(
r#"---
operator: CloudSqlInstanceStatus
project: project-002
instance: sql_instance-002
status:
  - Unspecified
  - Runnable
"#
        )
    )]
    #[trace]
    fn test_spec_serialize(#[case] spec: Spec, #[case] expected: String) {
        assert_eq!(serde_yaml::to_string(&spec).unwrap(), expected);
    }
}
