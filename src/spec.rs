pub mod spec_response;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::client::gke_client::GKEClient;
use crate::operator::gke_cluster_status_operator::GKEClusterStatusOperator;
use crate::spec::spec_response::SpecResponse;

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Clone)]
pub enum ClusterStatus {
    Unspecified,
    Provisioning,
    Running,
    Reconciling,
    Stopping,
    Error,
    Degraded,
}

impl fmt::Display for ClusterStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)?;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Clone)]
#[serde(tag = "operator")]
pub enum Spec {
    GKEClusterStatus {
        project: String,
        location: String,
        cluster: String,
        status: Vec<ClusterStatus>,
    },
}

impl fmt::Display for Spec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::GKEClusterStatus {
                project,
                location,
                cluster,
                status,
            } => {
                writeln!(f, "  - spec:")?;
                writeln!(f, "      operator: GKEClusterStatus")?;
                writeln!(f, "      project: {}", project)?;
                writeln!(f, "      location: {}", location)?;
                writeln!(f, "      cluster: {}", cluster)?;
                writeln!(f, "      status:")?;
                for (i, s) in status.iter().enumerate() {
                    if i == status.len() - 1 {
                        write!(f, "        - {:?}", s)?;
                    } else {
                        writeln!(f, "        - {:?}", s)?;
                    }
                }
                Ok(())
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
    use rstest::rstest;

    #[rstest]
    #[case(ClusterStatus::Unspecified, format!("Unspecified"))]
    #[case(ClusterStatus::Provisioning, format!("Provisioning"))]
    #[case(ClusterStatus::Running, format!("Running"))]
    #[case(ClusterStatus::Reconciling, format!("Reconciling"))]
    #[case(ClusterStatus::Stopping, format!("Stopping"))]
    #[case(ClusterStatus::Error, format!("Error"))]
    #[case(ClusterStatus::Degraded, format!("Degraded"))]
    #[trace]
    fn test_cluster_status_fmt(#[case] cluster_status: ClusterStatus, #[case] expected: String) {
        assert_eq!(format!("{}", cluster_status), expected);
    }

    #[rstest]
    #[case(
        Spec::GKEClusterStatus {
            project: format!("project-001"),
            location: format!("location-001"),
            cluster: format!("cluster-001"),
            status: vec![ClusterStatus::Provisioning],
        },
        format!(r#"  - spec:
      operator: GKEClusterStatus
      project: project-001
      location: location-001
      cluster: cluster-001
      status:
        - Provisioning"#)
    )]
    #[case(
        Spec::GKEClusterStatus {
            project: format!("project-002"),
            location: format!("location-002"),
            cluster: format!("cluster-002"),
            status: vec![ClusterStatus::Provisioning, ClusterStatus::Running],
        },
        format!(r#"  - spec:
      operator: GKEClusterStatus
      project: project-002
      location: location-002
      cluster: cluster-002
      status:
        - Provisioning
        - Running"#)
    )]
    #[trace]
    fn test_spec_fmt(#[case] spec: Spec, #[case] expected: String) {
        assert_eq!(format!("{}", spec), expected);
    }
}
