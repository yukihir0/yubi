use serde::ser::{Serialize, Serializer};
use serde::Deserialize;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Clone)]
pub enum NodePoolStatus {
    Unspecified,
    Provisioning,
    Running,
    RunningWithError,
    Reconciling,
    Stopping,
    Error,
}

impl Serialize for NodePoolStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{:?}", self))
    }
}

impl fmt::Display for NodePoolStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::spec::node_pool_status::*;
    use rstest::*;

    #[rstest]
    #[case(
        NodePoolStatus::Unspecified,
        format!(
r#"---
Unspecified
"#
        )
    )]
    #[case(
        NodePoolStatus::Provisioning,
        format!(
r#"---
Provisioning
"#
        )
    )]
    #[case(
        NodePoolStatus::Running,
        format!(
r#"---
Running
"#
        )
    )]
    #[case(
        NodePoolStatus::RunningWithError,
        format!(
r#"---
RunningWithError
"#
        )
    )]
    #[case(
        NodePoolStatus::Reconciling,
        format!(
r#"---
Reconciling
"#
        )
    )]
    #[case(
        NodePoolStatus::Stopping,
        format!(
r#"---
Stopping
"#
        )
    )]
    #[case(
        NodePoolStatus::Error,
        format!(
r#"---
Error
"#
        )
    )]
    #[trace]
    fn test_node_pool_status_serialize(
        #[case] node_pool_status: NodePoolStatus,
        #[case] expected: String,
    ) {
        assert_eq!(serde_yaml::to_string(&node_pool_status).unwrap(), expected);
    }
}
