use serde::ser::{Serialize, Serializer};
use serde::Deserialize;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Clone)]
pub enum SqlInstanceStatus {
    Unspecified,
    Runnable,
    Suspended,
    PendingDelete,
    PendingCreate,
    Maintenance,
    Failed,
    OnlineMaintenance,
}

impl Serialize for SqlInstanceStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{:?}", self))
    }
}

impl fmt::Display for SqlInstanceStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::spec::sql_instance_status::*;
    use rstest::*;

    #[rstest]
    #[case(
        SqlInstanceStatus::Unspecified,
        format!(
r#"---
Unspecified
"#
        )
    )]
    #[case(
        SqlInstanceStatus::Runnable,
        format!(
r#"---
Runnable
"#
        )
    )]
    #[case(
        SqlInstanceStatus::Suspended,
        format!(
r#"---
Suspended
"#
        )
    )]
    #[case(
        SqlInstanceStatus::PendingDelete,
        format!(
r#"---
PendingDelete
"#
        )
    )]
    #[case(
        SqlInstanceStatus::PendingCreate,
        format!(
r#"---
PendingCreate
"#
        )
    )]
    #[case(
        SqlInstanceStatus::Maintenance,
        format!(
r#"---
Maintenance
"#
        )
    )]
    #[case(
        SqlInstanceStatus::Failed,
        format!(
r#"---
Failed
"#
        )
    )]
    #[case(
        SqlInstanceStatus::OnlineMaintenance,
        format!(
r#"---
OnlineMaintenance
"#
        )
    )]
    #[trace]
    fn test_sql_instance_status_serialize(
        #[case] instance_status: SqlInstanceStatus,
        #[case] expected: String,
    ) {
        assert_eq!(serde_yaml::to_string(&instance_status).unwrap(), expected);
    }
}
