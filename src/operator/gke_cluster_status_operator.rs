use anyhow::Result;
use googapis::google::container::v1::cluster;

use crate::client::gke_client::GKEClientTrait;
use crate::spec::spec_response::SpecResponse;

pub struct GKEClusterStatusOperator {
    project: String,
    location: String,
    cluster: String,
    client: Box<dyn GKEClientTrait>,
}

impl GKEClusterStatusOperator {
    pub fn new(
        project: String,
        location: String,
        cluster: String,
        client: Box<dyn GKEClientTrait>,
    ) -> GKEClusterStatusOperator {
        GKEClusterStatusOperator {
            project: project,
            location: location,
            cluster: cluster,
            client: client,
        }
    }

    pub async fn check(&self) -> Result<SpecResponse> {
        let status = self
            .client
            .fetch_cluster_status(&self.project, &self.location, &self.cluster)
            .await?;
        self.compare(status)
    }

    fn compare(&self, status: cluster::Status) -> Result<SpecResponse> {
        match status {
            cluster::Status::Running => Ok(SpecResponse::Success {
                message: format!("{} is running", self.cluster),
            }),
            _ => Ok(SpecResponse::Failure {
                message: format!("{} is not running", self.cluster),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::client::gke_client::*;
    use crate::operator::gke_cluster_status_operator::*;
    use rstest::*;

    #[rstest]
    #[case(format!("cluster-001"), cluster::Status::Unspecified, SpecResponse::Failure{message: format!("cluster-001 is not running")})]
    #[case(format!("cluster-002"), cluster::Status::Provisioning, SpecResponse::Failure{message: format!("cluster-002 is not running")})]
    #[case(format!("cluster-003"), cluster::Status::Running, SpecResponse::Success{message: format!("cluster-003 is running")})]
    #[case(format!("cluster-004"), cluster::Status::Reconciling, SpecResponse::Failure{message: format!("cluster-004 is not running")})]
    #[case(format!("cluster-005"), cluster::Status::Stopping, SpecResponse::Failure{message: format!("cluster-005 is not running")})]
    #[case(format!("cluster-006"), cluster::Status::Error, SpecResponse::Failure{message: format!("cluster-006 is not running")})]
    #[case(format!("cluster-007"), cluster::Status::Degraded, SpecResponse::Failure{message: format!("cluster-007 is not running")})]
    #[trace]
    async fn test_check(
        #[case] cluster: String,
        #[case] status: cluster::Status,
        #[case] expected: SpecResponse,
    ) {
        let mut client = MockGKEClientTrait::new();
        client
            .expect_fetch_cluster_status()
            .returning(move |_, _, _| Ok(status));

        let operator = GKEClusterStatusOperator::new(
            format!("project"),
            format!("location"),
            cluster,
            Box::new(client),
        );

        match operator.check().await {
            Ok(spec_response) => {
                assert_eq!(spec_response, expected);
            }
            Err(_) => {
                assert!(false, "check gke cluster status error")
            }
        }
    }

    #[rstest]
    #[case(format!("cluster-001"), cluster::Status::Unspecified, SpecResponse::Failure{message: format!("cluster-001 is not running")})]
    #[case(format!("cluster-002"), cluster::Status::Provisioning, SpecResponse::Failure{message: format!("cluster-002 is not running")})]
    #[case(format!("cluster-003"), cluster::Status::Running, SpecResponse::Success{message: format!("cluster-003 is running")})]
    #[case(format!("cluster-004"), cluster::Status::Reconciling, SpecResponse::Failure{message: format!("cluster-004 is not running")})]
    #[case(format!("cluster-005"), cluster::Status::Stopping, SpecResponse::Failure{message: format!("cluster-005 is not running")})]
    #[case(format!("cluster-006"), cluster::Status::Error, SpecResponse::Failure{message: format!("cluster-006 is not running")})]
    #[case(format!("cluster-007"), cluster::Status::Degraded, SpecResponse::Failure{message: format!("cluster-007 is not running")})]
    #[trace]
    fn test_compare(
        #[case] cluster: String,
        #[case] status: cluster::Status,
        #[case] expected: SpecResponse,
    ) {
        let operator = GKEClusterStatusOperator::new(
            format!("project"),
            format!("location"),
            cluster,
            Box::new(GKEClient::new()),
        );

        match operator.compare(status) {
            Ok(spec_response) => {
                assert_eq!(spec_response, expected)
            }
            Err(_) => {
                assert!(false, "compare gke cluster status error")
            }
        }
    }
}
