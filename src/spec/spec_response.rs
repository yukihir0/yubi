use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum SpecResponse {
    Success { message: String },
    Failure { message: String },
}

impl fmt::Display for SpecResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Success { message } => {
                writeln!(f, "    spec_response:")?;
                writeln!(f, "      result: {}", self.status())?;
                write!(f, "      message: {}", message)?;
                Ok(())
            }
            Self::Failure { message } => {
                writeln!(f, "    spec_response:")?;
                writeln!(f, "      result: {}", self.status())?;
                write!(f, "      message: {}", message)?;
                Ok(())
            }
        }
    }
}

impl SpecResponse {
    pub fn status(&self) -> String {
        match self {
            Self::Success { message: _ } => {
                format!("success")
            }
            Self::Failure { message: _ } => {
                format!("failure")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::spec::*;
    use rstest::rstest;

    #[rstest]
    #[case(SpecResponse::Success{ message: format!("success_message")}, format!(r#"    spec_response:
      result: success
      message: success_message"#))]
    #[case(SpecResponse::Failure{ message: format!("failure_message")}, format!(r#"    spec_response:
      result: failure
      message: failure_message"#))]
    #[trace]
    fn test_fmt(#[case] spec_response: SpecResponse, #[case] expected: String) {
        assert_eq!(format!("{}", spec_response), expected);
    }

    #[rstest]
    #[case(SpecResponse::Success{ message: format!("success_message")}, format!("success"))]
    #[case(SpecResponse::Failure{ message: format!("failure_message")}, format!("failure"))]
    #[trace]
    fn test_status(#[case] spec_response: SpecResponse, #[case] expected: String) {
        assert_eq!(spec_response.status(), expected);
    }
}
