pub mod spec_response;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::client::gke_client::GKEClient;
use crate::operator::gke_cluster_status_operator::GKEClusterStatusOperator;
use crate::spec::spec_response::SpecResponse;

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Clone)]
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
                project,
                location,
                cluster,
            } => {
                writeln!(f, "  - spec:")?;
                writeln!(f, "      operator: GKEClusterStatus")?;
                writeln!(f, "      project: {}", project)?;
                writeln!(f, "      location: {}", location)?;
                write!(f, "      cluster: {}", cluster)
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
                GKEClusterStatusOperator::new(
                    project.clone(),
                    location.clone(),
                    cluster.clone(),
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
    #[case(
        Spec::GKEClusterStatus {
            project: format!("project-001"),
            location: format!("location-001"),
            cluster: format!("cluster-001"),
        },
        format!(r#"  - spec:
      operator: GKEClusterStatus
      project: project-001
      location: location-001
      cluster: cluster-001"#)
    )]
    #[trace]
    fn test_fmt(#[case] spec: Spec, #[case] expected: String) {
        assert_eq!(format!("{}", spec), expected);
    }
}
