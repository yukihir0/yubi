mod detail;
mod record;
mod summary;

use crate::report::detail::ReportDetail;
use crate::report::record::Record;
use crate::report::summary::ReportSummary;
use crate::spec::result::SpecResult;
use crate::spec::Spec;
use anyhow::Result;
use serde::ser::{Serialize, SerializeMap, Serializer};

pub struct Report {
    records: Vec<Record>,
}

impl Serialize for Report {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry(
            "summary",
            &ReportSummary::new(
                self.total_record_count(),
                self.success_record_count(),
                self.failure_record_count(),
                self.error_record_count(),
            ),
        )?;
        map.serialize_entry("detail", &ReportDetail::new(self.records.clone()))?;
        map.end()
    }
}

impl Report {
    pub fn new() -> Report {
        Report { records: vec![] }
    }

    pub fn record_ok(&mut self, spec: Spec, spec_result: SpecResult) {
        self.records.push(Record::new(spec, spec_result));
    }

    pub fn record_ng(&mut self, spec: Spec, error: anyhow::Error) {
        self.records.push(Record::new(
            spec,
            SpecResult::Error {
                description: format!("{}", error),
            },
        ));
    }

    pub fn is_all_green(&self) -> bool {
        self.total_record_count() == self.success_record_count()
    }

    fn total_record_count(&self) -> usize {
        self.success_record_count() + self.failure_record_count() + self.error_record_count()
    }

    fn success_record_count(&self) -> usize {
        self.records
            .iter()
            .filter(|record| match record.spec_result {
                SpecResult::Success { description: _ } => true,
                _ => false,
            })
            .count()
    }

    fn failure_record_count(&self) -> usize {
        self.records
            .iter()
            .filter(|record| match record.spec_result {
                SpecResult::Failure { description: _ } => true,
                _ => false,
            })
            .count()
    }

    fn error_record_count(&self) -> usize {
        self.records
            .iter()
            .filter(|record| match record.spec_result {
                SpecResult::Error { description: _ } => true,
                _ => false,
            })
            .count()
    }
}

#[cfg(test)]
mod tests {
    use crate::report::*;
    use crate::spec::cluster_status::*;
    use crate::spec::node_pool_status::*;
    use crate::spec::*;
    use rstest::*;

    #[rstest]
    #[case(
        SpecResult::Success {description: format!("success_description")},
        1,
        1,
        0,
        0,
        true
    )]
    #[case(
        SpecResult::Failure{description: format!("failure_description")},
        1,
        0,
        1,
        0,
        false
    )]
    #[case(
        SpecResult::Error{description: format!("error_description")},
        1,
        0,
        0,
        1,
        false
    )]
    #[trace]
    fn test_record_ok(
        #[case] spec_result: SpecResult,
        #[case] expected_total: usize,
        #[case] expected_success_count: usize,
        #[case] expected_failure_count: usize,
        #[case] expected_error_count: usize,
        #[case] expected_is_all_green: bool,
    ) {
        let spec = Spec::GKEClusterStatus {
            project: format!("project"),
            location: format!("location"),
            cluster: format!("cluster"),
            status: vec![ClusterStatus::Provisioning, ClusterStatus::Running],
        };

        let mut report = Report::new();
        report.record_ok(spec, spec_result);

        assert_eq!(report.total_record_count(), expected_total);
        assert_eq!(report.success_record_count(), expected_success_count);
        assert_eq!(report.failure_record_count(), expected_failure_count);
        assert_eq!(report.error_record_count(), expected_error_count);
        assert_eq!(report.is_all_green(), expected_is_all_green);
    }

    #[rstest]
    #[case(
        anyhow::Error::new(std::io::Error::new(std::io::ErrorKind::Other, "error_message")),
        1,
        0,
        0,
        1,
        false
    )]
    #[trace]
    fn test_record_ng(
        #[case] error: anyhow::Error,
        #[case] expected_total: usize,
        #[case] expected_success_count: usize,
        #[case] expected_failure_count: usize,
        #[case] expected_error_count: usize,
        #[case] expected_is_all_green: bool,
    ) {
        let spec = Spec::GKEClusterStatus {
            project: format!("project"),
            location: format!("location"),
            cluster: format!("cluster"),
            status: vec![ClusterStatus::Provisioning, ClusterStatus::Running],
        };

        let mut report = Report::new();
        report.record_ng(spec, error);

        assert_eq!(report.total_record_count(), expected_total);
        assert_eq!(report.success_record_count(), expected_success_count);
        assert_eq!(report.failure_record_count(), expected_failure_count);
        assert_eq!(report.error_record_count(), expected_error_count);
        assert_eq!(report.is_all_green(), expected_is_all_green);
    }

    #[fixture]
    fn fixture_records() -> Vec<Record> {
        let mut records = vec![];
        records.push(Record::new(
            Spec::GKEClusterStatus {
                project: format!("success_project"),
                location: format!("success_location"),
                cluster: format!("success_cluster"),
                status: vec![ClusterStatus::Provisioning, ClusterStatus::Running],
            },
            SpecResult::Success {
                description: format!("success_description"),
            },
        ));
        records.push(Record::new(
            Spec::GKEClusterStatus {
                project: format!("failure_project"),
                location: format!("failure_location"),
                cluster: format!("failure_cluster"),
                status: vec![ClusterStatus::Provisioning, ClusterStatus::Running],
            },
            SpecResult::Failure {
                description: format!("failure_description"),
            },
        ));
        records.push(Record::new(
            Spec::GKEClusterStatus {
                project: format!("error_project"),
                location: format!("error_location"),
                cluster: format!("error_cluster"),
                status: vec![ClusterStatus::Provisioning, ClusterStatus::Running],
            },
            SpecResult::Error {
                description: format!("error_description"),
            },
        ));
        records.push(Record::new(
            Spec::GKENodePoolStatus {
                project: format!("success_project"),
                location: format!("success_location"),
                cluster: format!("success_cluster"),
                node_pool: format!("success_node_pool"),
                status: vec![NodePoolStatus::Provisioning, NodePoolStatus::Running],
            },
            SpecResult::Success {
                description: format!("success_description"),
            },
        ));
        records.push(Record::new(
            Spec::GKENodePoolStatus {
                project: format!("failure_project"),
                location: format!("failure_location"),
                cluster: format!("failure_cluster"),
                node_pool: format!("failure_node_pool"),
                status: vec![NodePoolStatus::Provisioning, NodePoolStatus::Running],
            },
            SpecResult::Failure {
                description: format!("failure_description"),
            },
        ));
        records.push(Record::new(
            Spec::GKENodePoolStatus {
                project: format!("error_project"),
                location: format!("error_location"),
                cluster: format!("error_cluster"),
                node_pool: format!("error_node_pool"),
                status: vec![NodePoolStatus::Provisioning, NodePoolStatus::Running],
            },
            SpecResult::Error {
                description: format!("error_description"),
            },
        ));
        records
    }

    #[rstest]
    #[case(
        6,
        2,
        2,
        2,
        false,
        format!(
r#"summary:
  total: 6
  success: 2
  failure: 2
  error: 2
detail:
- spec:
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
- spec:
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
- spec:
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
- spec:
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
- spec:
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
- spec:
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
    #[trace]
    fn test_serialize(
        fixture_records: Vec<Record>,
        #[case] expected_total: usize,
        #[case] expected_success_count: usize,
        #[case] expected_failure_count: usize,
        #[case] expected_error_count: usize,
        #[case] expected_is_all_green: bool,
        #[case] expected_report: String,
    ) {
        let mut report = Report::new();
        for record in fixture_records {
            report.record_ok(record.spec, record.spec_result);
        }

        assert_eq!(report.total_record_count(), expected_total);
        assert_eq!(report.success_record_count(), expected_success_count);
        assert_eq!(report.failure_record_count(), expected_failure_count);
        assert_eq!(report.error_record_count(), expected_error_count);
        assert_eq!(report.is_all_green(), expected_is_all_green);
        assert_eq!(serde_yaml::to_string(&report).unwrap(), expected_report);
    }
}
