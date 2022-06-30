use anyhow::Result;
use googapis::google::cloud::sql::v1::database_instance;

use crate::client::cloud_sql_client::CloudSqlClientTrait;
use crate::spec::result::SpecResult;
use crate::spec::sql_instance_status::SqlInstanceStatus;

pub struct CloudSqlInstanceStatusOperator {
    project: String,
    instance: String,
    status: Vec<SqlInstanceStatus>,
    client: Box<dyn CloudSqlClientTrait>,
}

impl CloudSqlInstanceStatusOperator {
    pub fn new(
        project: String,
        instance: String,
        status: Vec<SqlInstanceStatus>,
        client: Box<dyn CloudSqlClientTrait>,
    ) -> CloudSqlInstanceStatusOperator {
        CloudSqlInstanceStatusOperator {
            project: project,
            instance: instance,
            status: status,
            client: client,
        }
    }

    pub async fn check(&self) -> Result<SpecResult> {
        let status = self
            .client
            .fetch_sql_instance_status(&self.project, &self.instance)
            .await?;
        self.compare(status)
    }

    fn compare(&self, status: database_instance::SqlInstanceState) -> Result<SpecResult> {
        match status {
            database_instance::SqlInstanceState::Unspecified => {
                Ok(self.compare_with(SqlInstanceStatus::Unspecified))
            }
            database_instance::SqlInstanceState::Runnable => {
                Ok(self.compare_with(SqlInstanceStatus::Runnable))
            }
            database_instance::SqlInstanceState::Suspended => {
                Ok(self.compare_with(SqlInstanceStatus::Suspended))
            }
            database_instance::SqlInstanceState::PendingDelete => {
                Ok(self.compare_with(SqlInstanceStatus::PendingDelete))
            }
            database_instance::SqlInstanceState::PendingCreate => {
                Ok(self.compare_with(SqlInstanceStatus::PendingCreate))
            }
            database_instance::SqlInstanceState::Maintenance => {
                Ok(self.compare_with(SqlInstanceStatus::Maintenance))
            }
            database_instance::SqlInstanceState::Failed => {
                Ok(self.compare_with(SqlInstanceStatus::Failed))
            }
            database_instance::SqlInstanceState::OnlineMaintenance => {
                Ok(self.compare_with(SqlInstanceStatus::OnlineMaintenance))
            }
        }
    }

    fn compare_with(&self, instance_status: SqlInstanceStatus) -> SpecResult {
        if self.status.contains(&instance_status) {
            SpecResult::Success {
                description: format!("{} is {}", self.instance, instance_status),
            }
        } else {
            SpecResult::Failure {
                description: format!("{} is {}", self.instance, instance_status),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::client::cloud_sql_client::*;
    use crate::operator::cloud_sql_instance_status_operator::*;
    use rstest::*;

    #[rstest]
    #[case(
        format!("instance-001"),
        vec![SqlInstanceStatus::Unspecified, SqlInstanceStatus::Runnable],
        database_instance::SqlInstanceState::Unspecified,
        SpecResult::Success{description: format!("instance-001 is Unspecified")}
    )]
    #[case(
        format!("instance-002"),
        vec![SqlInstanceStatus::Unspecified, SqlInstanceStatus::Runnable],
        database_instance::SqlInstanceState::Runnable,
        SpecResult::Success{description: format!("instance-002 is Runnable")}
    )]
    #[case(
        format!("instance-003"),
        vec![SqlInstanceStatus::Unspecified, SqlInstanceStatus::Runnable],
        database_instance::SqlInstanceState::Suspended,
        SpecResult::Failure{description: format!("instance-003 is Suspended")}
    )]
    #[case(
        format!("instance-004"),
        vec![SqlInstanceStatus::Unspecified, SqlInstanceStatus::Runnable],
        database_instance::SqlInstanceState::PendingDelete,
        SpecResult::Failure{description: format!("instance-004 is PendingDelete")}
    )]
    #[case(
        format!("instance-005"),
        vec![SqlInstanceStatus::Unspecified, SqlInstanceStatus::Runnable],
        database_instance::SqlInstanceState::PendingCreate,
        SpecResult::Failure{description: format!("instance-005 is PendingCreate")}
    )]
    #[case(
        format!("instance-006"),
        vec![SqlInstanceStatus::Unspecified, SqlInstanceStatus::Runnable],
        database_instance::SqlInstanceState::Maintenance,
        SpecResult::Failure{description: format!("instance-006 is Maintenance")}
    )]
    #[case(
        format!("instance-007"),
        vec![SqlInstanceStatus::Unspecified, SqlInstanceStatus::Runnable],
        database_instance::SqlInstanceState::Failed,
        SpecResult::Failure{description: format!("instance-007 is Failed")}
    )]
    #[case(
        format!("instance-008"),
        vec![SqlInstanceStatus::Unspecified, SqlInstanceStatus::Runnable],
        database_instance::SqlInstanceState::OnlineMaintenance,
        SpecResult::Failure{description: format!("instance-008 is OnlineMaintenance")}
    )]
    #[trace]
    async fn test_check(
        #[case] instance: String,
        #[case] instance_status: Vec<SqlInstanceStatus>,
        #[case] mocked_status: database_instance::SqlInstanceState,
        #[case] expected: SpecResult,
    ) {
        let mut client = MockCloudSqlClientTrait::new();
        client
            .expect_fetch_sql_instance_status()
            .returning(move |_, _| Ok(mocked_status));

        let operator = CloudSqlInstanceStatusOperator::new(
            format!("project"),
            instance,
            instance_status,
            Box::new(client),
        );

        match operator.check().await {
            Ok(spec_result) => {
                assert_eq!(spec_result, expected);
            }
            Err(_) => {
                assert!(false, "check cloud sql instance status error")
            }
        }
    }

    #[rstest]
    #[case(
        format!("instance-001"),
        vec![SqlInstanceStatus::Unspecified, SqlInstanceStatus::Runnable],
        database_instance::SqlInstanceState::Unspecified,
        SpecResult::Success{description: format!("instance-001 is Unspecified")}
    )]
    #[case(
        format!("instance-002"),
        vec![SqlInstanceStatus::Unspecified, SqlInstanceStatus::Runnable],
        database_instance::SqlInstanceState::Runnable,
        SpecResult::Success{description: format!("instance-002 is Runnable")}
    )]
    #[case(
        format!("instance-003"),
        vec![SqlInstanceStatus::Unspecified, SqlInstanceStatus::Runnable],
        database_instance::SqlInstanceState::Suspended,
        SpecResult::Failure{description: format!("instance-003 is Suspended")}
    )]
    #[case(
        format!("instance-004"),
        vec![SqlInstanceStatus::Unspecified, SqlInstanceStatus::Runnable],
        database_instance::SqlInstanceState::PendingDelete,
        SpecResult::Failure{description: format!("instance-004 is PendingDelete")}
    )]
    #[case(
        format!("instance-005"),
        vec![SqlInstanceStatus::Unspecified, SqlInstanceStatus::Runnable],
        database_instance::SqlInstanceState::PendingCreate,
        SpecResult::Failure{description: format!("instance-005 is PendingCreate")}
    )]
    #[case(
        format!("instance-006"),
        vec![SqlInstanceStatus::Unspecified, SqlInstanceStatus::Runnable],
        database_instance::SqlInstanceState::Maintenance,
        SpecResult::Failure{description: format!("instance-006 is Maintenance")}
    )]
    #[case(
        format!("instance-007"),
        vec![SqlInstanceStatus::Unspecified, SqlInstanceStatus::Runnable],
        database_instance::SqlInstanceState::Failed,
        SpecResult::Failure{description: format!("instance-007 is Failed")}
    )]
    #[case(
        format!("instance-008"),
        vec![SqlInstanceStatus::Unspecified, SqlInstanceStatus::Runnable],
        database_instance::SqlInstanceState::OnlineMaintenance,
        SpecResult::Failure{description: format!("instance-008 is OnlineMaintenance")}
    )]
    #[trace]
    fn test_compare(
        #[case] instance: String,
        #[case] instance_status: Vec<SqlInstanceStatus>,
        #[case] input_status: database_instance::SqlInstanceState,
        #[case] expected: SpecResult,
    ) {
        let operator = CloudSqlInstanceStatusOperator::new(
            format!("project"),
            instance,
            instance_status,
            Box::new(CloudSqlClient::new()),
        );

        match operator.compare(input_status) {
            Ok(spec_result) => {
                assert_eq!(spec_result, expected)
            }
            Err(_) => {
                assert!(false, "compare cloud sql instance status error")
            }
        }
    }
}
