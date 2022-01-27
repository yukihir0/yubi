use crate::spec::result::SpecResult;
use crate::spec::Spec;
use serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Debug, Clone)]
pub struct Record {
    pub spec: Spec,
    pub spec_result: SpecResult,
}

impl Record {
    pub fn new(spec: Spec, spec_result: SpecResult) -> Record {
        Record { spec, spec_result }
    }
}

impl Serialize for Record {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Record", 2)?;
        state.serialize_field("spec", &self.spec)?;
        state.serialize_field("spec_result", &self.spec_result)?;
        state.end()
    }
}

#[cfg(test)]
mod tests {
    use crate::report::*;
    use crate::spec::cluster_status::*;
    use crate::spec::node_pool_status::*;
    use crate::spec::sql_instance_status::*;
    use crate::spec::*;
    use rstest::*;

    #[rstest]
    #[case(
    Spec::GKEClusterStatus {
      project: format!("success_project"),
      location: format!("success_location"),
      cluster: format!("success_cluster"),
      status: vec![ClusterStatus::Provisioning, ClusterStatus::Running],
    },
    SpecResult::Success { description: format!("success_description") },
    format!(
r#"---
spec:
  operator: GKEClusterStatus
  project: success_project
  location: success_location
  cluster: success_cluster
  status:
    - Provisioning
    - Running
spec_result:
  code: success
  description: success_description
"#
    )
  )]
    #[case(
    Spec::GKEClusterStatus {
      project: format!("failure_project"),
      location: format!("failure_location"),
      cluster: format!("failure_cluster"),
      status: vec![ClusterStatus::Provisioning, ClusterStatus::Running],
    },
    SpecResult::Failure { description: format!("failure_description") },
    format!(
r#"---
spec:
  operator: GKEClusterStatus
  project: failure_project
  location: failure_location
  cluster: failure_cluster
  status:
    - Provisioning
    - Running
spec_result:
  code: failure
  description: failure_description
"#
    )
  )]
    #[case(
    Spec::GKEClusterStatus {
      project: format!("error_project"),
      location: format!("error_location"),
      cluster: format!("error_cluster"),
      status: vec![ClusterStatus::Provisioning, ClusterStatus::Running],
    },
    SpecResult::Error { description: format!("error_description") },
    format!(
r#"---
spec:
  operator: GKEClusterStatus
  project: error_project
  location: error_location
  cluster: error_cluster
  status:
    - Provisioning
    - Running
spec_result:
  code: error
  description: error_description
"#
    )
  )]
    #[case(
    Spec::GKENodePoolStatus {
      project: format!("success_project"),
      location: format!("success_location"),
      cluster: format!("success_cluster"),
      node_pool: format!("success_node_pool"),
      status: vec![NodePoolStatus::Provisioning, NodePoolStatus::Running],
    },
    SpecResult::Success { description: format!("success_description") },
    format!(
r#"---
spec:
  operator: GKENodePoolStatus
  project: success_project
  location: success_location
  cluster: success_cluster
  node_pool: success_node_pool
  status:
    - Provisioning
    - Running
spec_result:
  code: success
  description: success_description
"#
    )
  )]
    #[case(
    Spec::GKENodePoolStatus {
      project: format!("failure_project"),
      location: format!("failure_location"),
      cluster: format!("failure_cluster"),
      node_pool: format!("failure_node_pool"),
      status: vec![NodePoolStatus::Provisioning, NodePoolStatus::Running],
    },
    SpecResult::Failure { description: format!("failure_description") },
    format!(
r#"---
spec:
  operator: GKENodePoolStatus
  project: failure_project
  location: failure_location
  cluster: failure_cluster
  node_pool: failure_node_pool
  status:
    - Provisioning
    - Running
spec_result:
  code: failure
  description: failure_description
"#
    )
  )]
    #[case(
    Spec::GKENodePoolStatus {
      project: format!("error_project"),
      location: format!("error_location"),
      cluster: format!("error_cluster"),
      node_pool: format!("error_node_pool"),
      status: vec![NodePoolStatus::Provisioning, NodePoolStatus::Running],
      },
    SpecResult::Error { description: format!("error_description") },
    format!(
r#"---
spec:
  operator: GKENodePoolStatus
  project: error_project
  location: error_location
  cluster: error_cluster
  node_pool: error_node_pool
  status:
    - Provisioning
    - Running
spec_result:
  code: error
  description: error_description
"#
    )
  )]
    #[case(
    Spec::CloudSqlInstanceStatus {
      project: format!("success_project"),
      instance: format!("success_instance"),
      status: vec![SqlInstanceStatus::Unspecified, SqlInstanceStatus::Runnable],
      },
    SpecResult::Success { description: format!("success_description") },
    format!(
r#"---
spec:
  operator: CloudSqlInstanceStatus
  project: success_project
  instance: success_instance
  status:
    - Unspecified
    - Runnable
spec_result:
  code: success
  description: success_description
"#
    )
  )]
    #[case(
    Spec::CloudSqlInstanceStatus {
      project: format!("failure_project"),
      instance: format!("failure_instance"),
      status: vec![SqlInstanceStatus::Unspecified, SqlInstanceStatus::Runnable],
    },
    SpecResult::Failure { description: format!("failure_description") },
    format!(
r#"---
spec:
  operator: CloudSqlInstanceStatus
  project: failure_project
  instance: failure_instance
  status:
    - Unspecified
    - Runnable
spec_result:
  code: failure
  description: failure_description
"#
    )
  )]
    #[case(
    Spec::CloudSqlInstanceStatus {
      project: format!("error_project"),
      instance: format!("error_instance"),
      status: vec![SqlInstanceStatus::Unspecified, SqlInstanceStatus::Runnable],
    },
    SpecResult::Error { description: format!("error_description") },
    format!(
r#"---
spec:
  operator: CloudSqlInstanceStatus
  project: error_project
  instance: error_instance
  status:
    - Unspecified
    - Runnable
spec_result:
  code: error
  description: error_description
"#
    )
  )]
    #[trace]
    fn test_serialize(
        #[case] spec: Spec,
        #[case] spec_result: SpecResult,
        #[case] expected: String,
    ) {
        let record = Record::new(spec, spec_result);
        assert_eq!(serde_yaml::to_string(&record).unwrap(), expected);
    }
}
