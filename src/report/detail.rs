use crate::report::record::Record;
use serde::ser::{Serialize, SerializeSeq, Serializer};

pub struct ReportDetail {
    records: Vec<Record>,
}

impl ReportDetail {
    pub fn new(records: Vec<Record>) -> ReportDetail {
        ReportDetail { records }
    }
}

impl Serialize for ReportDetail {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.records.len()))?;
        for record in &self.records {
            seq.serialize_element(&Record::new(
                record.spec.clone(),
                record.spec_result.clone(),
            ))?;
        }
        seq.end()
    }
}

#[cfg(test)]
mod tests {
    use crate::report::*;
    use crate::spec::*;
    use rstest::*;

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
        records
    }

    #[rstest]
    #[case(
        format!(
r#"---
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
"#
        )
    )]
    #[trace]
    fn test_serialize(fixture_records: Vec<Record>, #[case] expected: String) {
        let report_detail = ReportDetail::new(fixture_records);
        assert_eq!(serde_yaml::to_string(&report_detail).unwrap(), expected);
    }
}
