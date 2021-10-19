use serde::ser::{Serialize, Serializer};
use serde::Deserialize;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Clone)]
pub enum ClusterStatus {
    Unspecified,
    Provisioning,
    Running,
    Reconciling,
    Stopping,
    Error,
    Degraded,
}

impl Serialize for ClusterStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{:?}", self))
    }
}

impl fmt::Display for ClusterStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::spec::*;
    use rstest::*;

    #[rstest]
    #[case(
        ClusterStatus::Unspecified,
        format!(
r#"---
Unspecified
"#
        )
    )]
    #[case(
        ClusterStatus::Provisioning,
        format!(
r#"---
Provisioning
"#
        )
    )]
    #[case(
        ClusterStatus::Running,
        format!(
r#"---
Running
"#
        )
    )]
    #[case(
        ClusterStatus::Reconciling,
        format!(
r#"---
Reconciling
"#
        )
    )]
    #[case(
        ClusterStatus::Stopping,
        format!(
r#"---
Stopping
"#
        )
    )]
    #[case(
        ClusterStatus::Error,
        format!(
r#"---
Error
"#
        )
    )]
    #[case(
        ClusterStatus::Degraded,
        format!(
r#"---
Degraded
"#
        )
    )]
    #[trace]
    fn test_cluster_status_serialize(
        #[case] cluster_status: ClusterStatus,
        #[case] expected: String,
    ) {
        assert_eq!(serde_yaml::to_string(&cluster_status).unwrap(), expected);
    }
}
