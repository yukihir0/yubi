use serde::ser::{Serialize, SerializeMap, Serializer};

#[derive(Debug, PartialEq, Clone)]
pub enum SpecResult {
    Success { description: String },
    Failure { description: String },
    Error { description: String },
}

impl Serialize for SpecResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Success { description } => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("code", &self.code())?;
                map.serialize_entry("description", description)?;
                map.end()
            }
            Self::Failure { description } => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("code", &self.code())?;
                map.serialize_entry("description", description)?;
                map.end()
            }
            Self::Error { description } => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("code", &self.code())?;
                map.serialize_entry("description", description)?;
                map.end()
            }
        }
    }
}

impl SpecResult {
    pub fn code(&self) -> String {
        match self {
            Self::Success { description: _ } => {
                format!("success")
            }
            Self::Failure { description: _ } => {
                format!("failure")
            }
            Self::Error { description: _ } => {
                format!("error")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::spec::*;
    use rstest::*;

    #[rstest]
    #[case(
        SpecResult::Success{ description: format!("success_description")},
        format!("success")
    )]
    #[case(
        SpecResult::Failure{ description: format!("failure_description")},
        format!("failure")
    )]
    #[case(
        SpecResult::Error{ description: format!("error_description")},
        format!("error")
    )]
    #[trace]
    fn test_status(#[case] spec_result: SpecResult, #[case] expected: String) {
        assert_eq!(spec_result.code(), expected);
    }

    #[rstest]
    #[case(
        SpecResult::Success{ description: format!("success_description")},
        format!(
r#"code: success
description: success_description
"#
        )
    )]
    #[case(
        SpecResult::Failure{ description : format!("failure_description")},
        format!(
r#"code: failure
description: failure_description
"#
        )
    )]
    #[case(
        SpecResult::Error{ description: format!("error_description")},
        format!(
r#"code: error
description: error_description
"#
        )
    )]
    #[trace]
    fn test_serialize(#[case] spec_result: SpecResult, #[case] expected: String) {
        assert_eq!(serde_yaml::to_string(&spec_result).unwrap(), expected);
    }
}
