use anyhow::Result;
use googapis::google::container::v1::cluster;

use crate::client::gke_client::GKEClientTrait;
use crate::spec::cluster_status::ClusterStatus;
use crate::spec::result::SpecResult;

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

    pub async fn check(&self) -> Result<SpecResult> {
        let status = self
            .client
            .fetch_cluster_status(&self.project, &self.location, &self.cluster)
            .await?;
        self.compare(status)
    }

    fn compare(&self, status: cluster::Status) -> Result<SpecResult> {
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

    fn compare_with(&self, cluster_status: ClusterStatus) -> SpecResult {
        if self.status.contains(&cluster_status) {
            SpecResult::Success {
                description: format!("{} is {}", self.cluster, cluster_status),
            }
        } else {
            SpecResult::Failure {
                description: format!("{} is {}", self.cluster, cluster_status),
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
    #[case(
        format!("cluster-001"),
        vec![ClusterStatus::Provisioning, ClusterStatus::Running],
        cluster::Status::Unspecified,
        SpecResult::Failure{description: format!("cluster-001 is Unspecified")}
    )]
    #[case(
        format!("cluster-002"),
        vec![ClusterStatus::Provisioning, ClusterStatus::Running],
        cluster::Status::Provisioning,
        SpecResult::Success{description: format!("cluster-002 is Provisioning")}
    )]
    #[case(
        format!("cluster-003"),
        vec![ClusterStatus::Provisioning,
        ClusterStatus::Running],
        cluster::Status::Running,
        SpecResult::Success{description: format!("cluster-003 is Running")}
    )]
    #[case(
        format!("cluster-004"),
        vec![ClusterStatus::Provisioning,
        ClusterStatus::Running],
        cluster::Status::Reconciling,
        SpecResult::Failure{description: format!("cluster-004 is Reconciling")}
    )]
    #[case(
        format!("cluster-005"),
        vec![ClusterStatus::Provisioning, ClusterStatus::Running],
        cluster::Status::Stopping,
        SpecResult::Failure{description: format!("cluster-005 is Stopping")}
    )]
    #[case(
        format!("cluster-006"),
        vec![ClusterStatus::Provisioning, ClusterStatus::Running],
        cluster::Status::Error,
        SpecResult::Failure{description: format!("cluster-006 is Error")}
    )]
    #[case(
        format!("cluster-007"),
        vec![ClusterStatus::Provisioning, ClusterStatus::Running],
        cluster::Status::Degraded,
        SpecResult::Failure{description: format!("cluster-007 is Degraded")}
    )]
    #[trace]
    async fn test_check(
        #[case] cluster: String,
        #[case] cluster_status: Vec<ClusterStatus>,
        #[case] mocked_status: cluster::Status,
        #[case] expected: SpecResult,
    ) {
        let mut client = MockGKEClientTrait::new();
        client
            .expect_fetch_cluster_status()
            .returning(move |_, _, _| Ok(mocked_status));

        let operator = GKEClusterStatusOperator::new(
            format!("project"),
            format!("location"),
            cluster,
            cluster_status,
            Box::new(client),
        );

        match operator.check().await {
            Ok(spec_result) => {
                assert_eq!(spec_result, expected);
            }
            Err(_) => {
                assert!(false, "check gke cluster status error")
            }
        }
    }

    #[rstest]
    #[case(
        format!("cluster-001"),
        vec![ClusterStatus::Provisioning,
        ClusterStatus::Running],
        cluster::Status::Unspecified,
        SpecResult::Failure{description: format!("cluster-001 is Unspecified")}
    )]
    #[case(
        format!("cluster-002"),
        vec![ClusterStatus::Provisioning,ClusterStatus::Running],
        cluster::Status::Provisioning,
        SpecResult::Success{description: format!("cluster-002 is Provisioning")}
    )]
    #[case(
        format!("cluster-003"),
        vec![ClusterStatus::Provisioning, ClusterStatus::Running],
        cluster::Status::Running,
        SpecResult::Success{description: format!("cluster-003 is Running")}
    )]
    #[case(
        format!("cluster-004"),
        vec![ClusterStatus::Provisioning, ClusterStatus::Running],
        cluster::Status::Reconciling,
        SpecResult::Failure{description: format!("cluster-004 is Reconciling")}
    )]
    #[case(
        format!("cluster-005"),
        vec![ClusterStatus::Provisioning, ClusterStatus::Running],
        cluster::Status::Stopping,
        SpecResult::Failure{description: format!("cluster-005 is Stopping")}
    )]
    #[case(
        format!("cluster-006"),
        vec![ClusterStatus::Provisioning, ClusterStatus::Running],
        cluster::Status::Error,
        SpecResult::Failure{description: format!("cluster-006 is Error")}
    )]
    #[case(
        format!("cluster-007"),
        vec![ClusterStatus::Provisioning, ClusterStatus::Running],
        cluster::Status::Degraded,
        SpecResult::Failure{description: format!("cluster-007 is Degraded")}
    )]
    #[trace]
    fn test_compare(
        #[case] cluster: String,
        #[case] cluster_status: Vec<ClusterStatus>,
        #[case] input_status: cluster::Status,
        #[case] expected: SpecResult,
    ) {
        let operator = GKEClusterStatusOperator::new(
            format!("project"),
            format!("location"),
            cluster,
            cluster_status,
            Box::new(GKEClient::new()),
        );

        match operator.compare(input_status) {
            Ok(spec_result) => {
                assert_eq!(spec_result, expected)
            }
            Err(_) => {
                assert!(false, "compare gke cluster status error")
            }
        }
    }
}
