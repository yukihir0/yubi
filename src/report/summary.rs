use serde::ser::{Serialize, SerializeMap, Serializer};

pub struct ReportSummary {
    total: usize,
    success: usize,
    failure: usize,
    error: usize,
}

impl ReportSummary {
    pub fn new(total: usize, success: usize, failure: usize, error: usize) -> ReportSummary {
        ReportSummary {
            total,
            success,
            failure,
            error,
        }
    }
}

impl Serialize for ReportSummary {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("total", &self.total)?;
        map.serialize_entry("success", &self.success)?;
        map.serialize_entry("failure", &self.failure)?;
        map.serialize_entry("error", &self.error)?;
        map.end()
    }
}

#[cfg(test)]
mod tests {
    use crate::report::*;
    use rstest::*;

    #[rstest]
    #[case(
        1,
        2,
        3,
        4,
        format!(
r#"total: 1
success: 2
failure: 3
error: 4
"#
        )
    )]
    #[trace]
    fn test_serialize(
        #[case] total: usize,
        #[case] success: usize,
        #[case] failure: usize,
        #[case] error: usize,
        #[case] expected: String,
    ) {
        let report_summary = ReportSummary::new(total, success, failure, error);
        assert_eq!(serde_yaml::to_string(&report_summary).unwrap(), expected);
    }
}
