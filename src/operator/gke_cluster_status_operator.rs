use anyhow::Result;
use googapis::google::container::v1::cluster;

use crate::client::gke_client::GKEClientTrait;
use crate::spec::spec_response::SpecResponse;
use crate::spec::ClusterStatus;

pub struct GKEClusterStatusOperator {
    project: String,
    location: String,
    cluster: String,
    status: Vec<ClusterStatus>,
    client: Box<dyn GKEClientTrait>,
}

impl GKEClusterStatusOperator {
    pub fn new(
        project: String,
        location: String,
        cluster: String,
        status: Vec<ClusterStatus>,
        client: Box<dyn GKEClientTrait>,
    ) -> GKEClusterStatusOperator {
        GKEClusterStatusOperator {
            project: project,
            location: location,
            cluster: cluster,
            status: status,
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
            cluster::Status::Unspecified => Ok(self.compare_with(ClusterStatus::Unspecified)),
            cluster::Status::Provisioning => Ok(self.compare_with(ClusterStatus::Provisioning)),
            cluster::Status::Running => Ok(self.compare_with(ClusterStatus::Running)),
            cluster::Status::Reconciling => Ok(self.compare_with(ClusterStatus::Reconciling)),
            cluster::Status::Stopping => Ok(self.compare_with(ClusterStatus::Stopping)),
            cluster::Status::Error => Ok(self.compare_with(ClusterStatus::Error)),
            cluster::Status::Degraded => Ok(self.compare_with(ClusterStatus::Degraded)),
        }
    }

    fn compare_with(&self, cluster_status: ClusterStatus) -> SpecResponse {
        if self.status.contains(&cluster_status) {
            SpecResponse::Success {
                message: format!("{} is {}", self.cluster, cluster_status),
            }
        } else {
            SpecResponse::Failure {
                message: format!("{} is {}", self.cluster, cluster_status),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::client::gke_client::*;
    use crate::operator::gke_cluster_status_operator::*;
    use rstest::*;

    #[rstest]
    #[case(format!("cluster-001"), vec![ClusterStatus::Provisioning, ClusterStatus::Running], cluster::Status::Unspecified, SpecResponse::Failure{message: format!("cluster-001 is Unspecified")})]
    #[case(format!("cluster-002"), vec![ClusterStatus::Provisioning, ClusterStatus::Running], cluster::Status::Provisioning, SpecResponse::Success{message: format!("cluster-002 is Provisioning")})]
    #[case(format!("cluster-003"), vec![ClusterStatus::Provisioning, ClusterStatus::Running], cluster::Status::Running, SpecResponse::Success{message: format!("cluster-003 is Running")})]
    #[case(format!("cluster-004"), vec![ClusterStatus::Provisioning, ClusterStatus::Running], cluster::Status::Reconciling, SpecResponse::Failure{message: format!("cluster-004 is Reconciling")})]
    #[case(format!("cluster-005"), vec![ClusterStatus::Provisioning, ClusterStatus::Running], cluster::Status::Stopping, SpecResponse::Failure{message: format!("cluster-005 is Stopping")})]
    #[case(format!("cluster-006"), vec![ClusterStatus::Provisioning, ClusterStatus::Running], cluster::Status::Error, SpecResponse::Failure{message: format!("cluster-006 is Error")})]
    #[case(format!("cluster-007"), vec![ClusterStatus::Provisioning, ClusterStatus::Running], cluster::Status::Degraded, SpecResponse::Failure{message: format!("cluster-007 is Degraded")})]
    #[trace]
    async fn test_check(
        #[case] cluster: String,
        #[case] cluster_status: Vec<ClusterStatus>,
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
            cluster_status,
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
    #[case(format!("cluster-001"), vec![ClusterStatus::Provisioning, ClusterStatus::Running], cluster::Status::Unspecified, SpecResponse::Failure{message: format!("cluster-001 is Unspecified")})]
    #[case(format!("cluster-002"), vec![ClusterStatus::Provisioning, ClusterStatus::Running], cluster::Status::Provisioning, SpecResponse::Success{message: format!("cluster-002 is Provisioning")})]
    #[case(format!("cluster-003"), vec![ClusterStatus::Provisioning, ClusterStatus::Running], cluster::Status::Running, SpecResponse::Success{message: format!("cluster-003 is Running")})]
    #[case(format!("cluster-004"), vec![ClusterStatus::Provisioning, ClusterStatus::Running], cluster::Status::Reconciling, SpecResponse::Failure{message: format!("cluster-004 is Reconciling")})]
    #[case(format!("cluster-005"), vec![ClusterStatus::Provisioning, ClusterStatus::Running], cluster::Status::Stopping, SpecResponse::Failure{message: format!("cluster-005 is Stopping")})]
    #[case(format!("cluster-006"), vec![ClusterStatus::Provisioning, ClusterStatus::Running], cluster::Status::Error, SpecResponse::Failure{message: format!("cluster-006 is Error")})]
    #[case(format!("cluster-007"), vec![ClusterStatus::Provisioning, ClusterStatus::Running], cluster::Status::Degraded, SpecResponse::Failure{message: format!("cluster-007 is Degraded")})]
    #[trace]
    fn test_compare(
        #[case] cluster: String,
        #[case] cluster_status: Vec<ClusterStatus>,
        #[case] status: cluster::Status,
        #[case] expected: SpecResponse,
    ) {
        let operator = GKEClusterStatusOperator::new(
            format!("project"),
            format!("location"),
            cluster,
            cluster_status,
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
