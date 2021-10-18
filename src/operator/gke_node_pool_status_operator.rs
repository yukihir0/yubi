use anyhow::Result;
use googapis::google::container::v1::node_pool;

use crate::client::gke_client::GKEClientTrait;
use crate::spec::node_pool_status::NodePoolStatus;
use crate::spec::result::SpecResult;

pub struct GKENodePoolStatusOperator {
    project: String,
    location: String,
    cluster: String,
    node_pool: String,
    status: Vec<NodePoolStatus>,
    client: Box<dyn GKEClientTrait>,
}

impl GKENodePoolStatusOperator {
    pub fn new(
        project: String,
        location: String,
        cluster: String,
        node_pool: String,
        status: Vec<NodePoolStatus>,
        client: Box<dyn GKEClientTrait>,
    ) -> GKENodePoolStatusOperator {
        GKENodePoolStatusOperator {
            project: project,
            location: location,
            cluster: cluster,
            node_pool: node_pool,
            status: status,
            client: client,
        }
    }

    pub async fn check(&self) -> Result<SpecResult> {
        let status = self
            .client
            .fetch_node_pool_status(
                &self.project,
                &self.location,
                &self.cluster,
                &self.node_pool,
            )
            .await?;
        self.compare(status)
    }

    fn compare(&self, status: node_pool::Status) -> Result<SpecResult> {
        match status {
            node_pool::Status::Unspecified => Ok(self.compare_with(NodePoolStatus::Unspecified)),
            node_pool::Status::Provisioning => Ok(self.compare_with(NodePoolStatus::Provisioning)),
            node_pool::Status::Running => Ok(self.compare_with(NodePoolStatus::Running)),
            node_pool::Status::RunningWithError => {
                Ok(self.compare_with(NodePoolStatus::RunningWithError))
            }
            node_pool::Status::Reconciling => Ok(self.compare_with(NodePoolStatus::Reconciling)),
            node_pool::Status::Stopping => Ok(self.compare_with(NodePoolStatus::Stopping)),
            node_pool::Status::Error => Ok(self.compare_with(NodePoolStatus::Error)),
        }
    }

    fn compare_with(&self, node_pool_status: NodePoolStatus) -> SpecResult {
        if self.status.contains(&node_pool_status) {
            SpecResult::Success {
                description: format!("{} is {}", self.node_pool, node_pool_status),
            }
        } else {
            SpecResult::Failure {
                description: format!("{} is {}", self.node_pool, node_pool_status),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::client::gke_client::*;
    use crate::operator::gke_node_pool_status_operator::*;
    use rstest::*;

    #[rstest]
    #[case(
        format!("node_pool-001"),
        vec![NodePoolStatus::Provisioning, NodePoolStatus::Running],
        node_pool::Status::Unspecified,
        SpecResult::Failure{description: format!("node_pool-001 is Unspecified")}
    )]
    #[case(
        format!("node_pool-002"),
        vec![NodePoolStatus::Provisioning, NodePoolStatus::Running],
        node_pool::Status::Provisioning,
        SpecResult::Success{description: format!("node_pool-002 is Provisioning")}
    )]
    #[case(
        format!("node_pool-003"),
        vec![NodePoolStatus::Provisioning, NodePoolStatus::Running],
        node_pool::Status::Running,
        SpecResult::Success{description: format!("node_pool-003 is Running")}
    )]
    #[case(
        format!("node_pool-004"),
        vec![NodePoolStatus::Provisioning, NodePoolStatus::Running],
        node_pool::Status::RunningWithError,
        SpecResult::Failure{description: format!("node_pool-004 is RunningWithError")}
    )]
    #[case(
        format!("node_pool-005"),
        vec![NodePoolStatus::Provisioning, NodePoolStatus::Running],
        node_pool::Status::Reconciling,
        SpecResult::Failure{description: format!("node_pool-005 is Reconciling")}
    )]
    #[case(
        format!("node_pool-006"),
        vec![NodePoolStatus::Provisioning, NodePoolStatus::Running],
        node_pool::Status::Stopping,
        SpecResult::Failure{description: format!("node_pool-006 is Stopping")}
    )]
    #[case(
        format!("node_pool-007"),
        vec![NodePoolStatus::Provisioning, NodePoolStatus::Running],
        node_pool::Status::Error,
        SpecResult::Failure{description: format!("node_pool-007 is Error")}
    )]
    #[trace]
    async fn test_check(
        #[case] node_pool: String,
        #[case] node_pool_status: Vec<NodePoolStatus>,
        #[case] mocked_status: node_pool::Status,
        #[case] expected: SpecResult,
    ) {
        let mut client = MockGKEClientTrait::new();
        client
            .expect_fetch_node_pool_status()
            .returning(move |_, _, _, _| Ok(mocked_status));

        let operator = GKENodePoolStatusOperator::new(
            format!("project"),
            format!("location"),
            format!("cluster"),
            node_pool,
            node_pool_status,
            Box::new(client),
        );

        match operator.check().await {
            Ok(spec_result) => {
                assert_eq!(spec_result, expected);
            }
            Err(_) => {
                assert!(false, "check gke node_pool status error")
            }
        }
    }

    #[rstest]
    #[case(
        format!("node_pool-001"),
        vec![NodePoolStatus::Provisioning, NodePoolStatus::Running],
        node_pool::Status::Unspecified,
        SpecResult::Failure{description: format!("node_pool-001 is Unspecified")}
    )]
    #[case(
        format!("node_pool-002"),
        vec![NodePoolStatus::Provisioning, NodePoolStatus::Running],
        node_pool::Status::Provisioning,
        SpecResult::Success{description: format!("node_pool-002 is Provisioning")}
    )]
    #[case(
        format!("node_pool-003"),
        vec![NodePoolStatus::Provisioning, NodePoolStatus::Running],
        node_pool::Status::Running,
        SpecResult::Success{description: format!("node_pool-003 is Running")}
    )]
    #[case(
        format!("node_pool-004"),
        vec![NodePoolStatus::Provisioning, NodePoolStatus::Running],
        node_pool::Status::RunningWithError,
        SpecResult::Failure{description: format!("node_pool-004 is RunningWithError")}
    )]
    #[case(
        format!("node_pool-005"),
        vec![NodePoolStatus::Provisioning, NodePoolStatus::Running],
        node_pool::Status::Reconciling,
        SpecResult::Failure{description: format!("node_pool-005 is Reconciling")}
    )]
    #[case(
        format!("node_pool-006"),
        vec![NodePoolStatus::Provisioning, NodePoolStatus::Running],
        node_pool::Status::Stopping,
        SpecResult::Failure{description: format!("node_pool-006 is Stopping")}
    )]
    #[case(
        format!("node_pool-007"),
        vec![NodePoolStatus::Provisioning, NodePoolStatus::Running],
        node_pool::Status::Error,
        SpecResult::Failure{description: format!("node_pool-007 is Error")}
    )]
    #[trace]
    fn test_compare(
        #[case] node_pool: String,
        #[case] node_pool_status: Vec<NodePoolStatus>,
        #[case] input_status: node_pool::Status,
        #[case] expected: SpecResult,
    ) {
        let operator = GKENodePoolStatusOperator::new(
            format!("project"),
            format!("location"),
            format!("cluster"),
            node_pool,
            node_pool_status,
            Box::new(GKEClient::new()),
        );

        match operator.compare(input_status) {
            Ok(spec_result) => {
                assert_eq!(spec_result, expected)
            }
            Err(_) => {
                assert!(false, "compare gke node_pool status error")
            }
        }
    }
}
