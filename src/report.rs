use crate::spec::spec_response::SpecResponse;
use crate::spec::Spec;
use anyhow::Result;
use handlebars::Handlebars;
use indexmap::IndexMap;
use std::collections::HashMap;
use std::io::Write;

pub struct Report<'a> {
    // use IndexMap to guarantee the order of insertion
    responses: IndexMap<Spec, SpecResponse>,
    errors: IndexMap<Spec, anyhow::Error>,
    output: &'a mut dyn Write,
}

impl<'a> Report<'a> {
    pub fn new(output: &mut dyn Write) -> Report {
        Report {
            responses: IndexMap::new(),
            errors: IndexMap::new(),
            output: output,
        }
    }

    pub fn add_response(&mut self, spec: Spec, response: SpecResponse) {
        self.responses.insert(spec, response);
    }

    pub fn add_error(&mut self, spec: Spec, error: anyhow::Error) {
        self.errors.insert(spec, error);
    }

    pub fn print(&mut self) -> Result<()> {
        self.print_summary()?;
        self.print_detail()?;
        Ok(())
    }

    pub fn is_all_green(&self) -> bool {
        self.total_count() == self.success_count()
    }

    fn print_summary(&mut self) -> Result<()> {
        let mut data = HashMap::<&str, usize>::new();
        data.insert("total_count", self.total_count());
        data.insert("success_count", self.success_count());
        data.insert("failure_count", self.failure_count());
        data.insert("error_count", self.error_count());

        let mut hbs = Handlebars::new();
        hbs.register_template_file("summary", "./src/templates/summary.hbs")?;
        writeln!(self.output, "{}", hbs.render("summary", &data)?)?;
        Ok(())
    }

    fn print_detail(&mut self) -> Result<()> {
        let mut data = HashMap::<&str, usize>::new();

        let mut hbs = Handlebars::new();
        hbs.register_template_file("detail", "./src/templates/detail.hbs")?;
        writeln!(self.output, "{}", hbs.render("detail", &data)?)?;

        for (spec, spec_response) in &self.responses {
            writeln!(self.output, "{}", spec)?;
            writeln!(self.output, "{}", spec_response)?;
        }
        for (spec, error) in &self.errors {
            writeln!(self.output, "{}", spec)?;
            writeln!(self.output, "    spec_response:")?;
            writeln!(self.output, "      result: error")?;
            writeln!(self.output, "      message: {}", error)?;
        }
        writeln!(self.output, "")?;
        Ok(())
    }

    fn total_count(&self) -> usize {
        self.success_count() + self.failure_count() + self.error_count()
    }

    fn success_count(&self) -> usize {
        let success_responses = self.responses.values().filter(|res| match res {
            SpecResponse::Success { message: _ } => true,
            _ => false,
        });

        success_responses.count()
    }

    fn failure_count(&self) -> usize {
        let failure_responses = self.responses.values().filter(|res| match res {
            SpecResponse::Failure { message: _ } => true,
            _ => false,
        });

        failure_responses.count()
    }

    fn error_count(&self) -> usize {
        self.errors.values().len()
    }
}

#[cfg(test)]
mod tests {
    use crate::report::*;
    use crate::spec::*;
    use rstest::*;

    #[rstest]
    #[case(
        SpecResponse::Success {message: format!("response")},
        1,
        1,
        0,
        0,
        true
    )]
    #[case(
        SpecResponse::Failure{message: format!("response")},
        1,
        0,
        1,
        0,
        false
    )]
    #[trace]
    fn test_add_response(
        #[case] spec_response: SpecResponse,
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

        let mut stdout = vec![];
        let mut report = Report::new(&mut stdout);
        report.add_response(spec, spec_response);

        assert_eq!(report.total_count(), expected_total);
        assert_eq!(report.success_count(), expected_success_count);
        assert_eq!(report.failure_count(), expected_failure_count);
        assert_eq!(report.error_count(), expected_error_count);
        assert_eq!(report.is_all_green(), expected_is_all_green);
    }

    #[rstest]
    #[case(
        anyhow::Error::new(std::io::Error::new(std::io::ErrorKind::Other, "error")),
        1,
        0,
        0,
        1,
        false
    )]
    #[trace]
    fn test_add_error(
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

        let mut stdout = vec![];
        let mut report = Report::new(&mut stdout);
        report.add_error(spec, error);

        assert_eq!(report.total_count(), expected_total);
        assert_eq!(report.success_count(), expected_success_count);
        assert_eq!(report.failure_count(), expected_failure_count);
        assert_eq!(report.error_count(), expected_error_count);
        assert_eq!(report.is_all_green(), expected_is_all_green);
    }

    #[fixture]
    fn spec_responses_map_fixture() -> IndexMap<Spec, SpecResponse> {
        let mut spec_responses = IndexMap::new();
        spec_responses.insert(
            Spec::GKEClusterStatus {
                project: format!("success_project"),
                location: format!("success_location"),
                cluster: format!("success_cluster"),
                status: vec![ClusterStatus::Provisioning, ClusterStatus::Running],
            },
            SpecResponse::Success {
                message: format!("success_response"),
            },
        );
        spec_responses.insert(
            Spec::GKEClusterStatus {
                project: format!("failure_project"),
                location: format!("failure_location"),
                cluster: format!("failure_cluster"),
                status: vec![ClusterStatus::Provisioning, ClusterStatus::Running],
            },
            SpecResponse::Failure {
                message: format!("failure_response"),
            },
        );
        spec_responses
    }

    #[fixture]
    fn errors_map_fixture() -> IndexMap<Spec, anyhow::Error> {
        let mut errors = IndexMap::new();
        errors.insert(
            Spec::GKEClusterStatus {
                project: format!("error_project"),
                location: format!("error_location"),
                cluster: format!("error_cluster"),
                status: vec![ClusterStatus::Provisioning, ClusterStatus::Running],
            },
            anyhow::Error::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "error_response",
            )),
        );

        errors
    }

    #[rstest]
    #[case(
        3,
        1,
        1,
        1,
        false,
        format!(r#"summary:
  total: 3
  success: 1
  failure: 1
  error: 1

detail:
  - spec:
      operator: GKEClusterStatus
      project: success_project
      location: success_location
      cluster: success_cluster
      status:
        - Provisioning
        - Running
    spec_response:
      result: success
      message: success_response
  - spec:
      operator: GKEClusterStatus
      project: failure_project
      location: failure_location
      cluster: failure_cluster
      status:
        - Provisioning
        - Running
    spec_response:
      result: failure
      message: failure_response
  - spec:
      operator: GKEClusterStatus
      project: error_project
      location: error_location
      cluster: error_cluster
      status:
        - Provisioning
        - Running
    spec_response:
      result: error
      message: error_response

"#)
    )]
    #[trace]
    fn test_print(
        spec_responses_map_fixture: IndexMap<Spec, SpecResponse>,
        errors_map_fixture: IndexMap<Spec, anyhow::Error>,
        #[case] expected_total: usize,
        #[case] expected_success_count: usize,
        #[case] expected_failure_count: usize,
        #[case] expected_error_count: usize,
        #[case] expected_is_all_green: bool,
        #[case] expected_report: String,
    ) {
        let mut stdout = vec![];
        let mut report = Report::new(&mut stdout);
        for (spec, spec_response) in spec_responses_map_fixture {
            report.add_response(spec, spec_response);
        }
        for (spec, error) in errors_map_fixture {
            report.add_error(spec, error);
        }

        match report.print() {
            Ok(()) => {
                assert!(true, "print report success")
            }
            Err(_) => {
                assert!(false, "print report error")
            }
        }

        assert_eq!(report.total_count(), expected_total);
        assert_eq!(report.success_count(), expected_success_count);
        assert_eq!(report.failure_count(), expected_failure_count);
        assert_eq!(report.error_count(), expected_error_count);
        assert_eq!(report.is_all_green(), expected_is_all_green);
        assert_eq!(String::from_utf8(stdout.to_vec()).unwrap(), expected_report);
    }
}
