use crate::spec::{Spec, SpecResponse};
use anyhow::Result;
use std::collections::HashMap;
use std::io::Write;

pub struct Report {
    responses: HashMap<Spec, SpecResponse>,
    errors: HashMap<Spec, anyhow::Error>,
    output: Box<dyn Write>,
}

impl Report {
    pub fn new(output: Box<dyn Write>) -> Report {
        Report {
            responses: HashMap::new(),
            errors: HashMap::new(),
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

    fn print_summary(&mut self) -> Result<()> {
        writeln!(self.output, "--- summary ---")?;
        writeln!(self.output, "total: {}", self.total_count())?;
        writeln!(self.output, "success: {}", self.success_count())?;
        writeln!(self.output, "fail: {}", self.fail_count())?;
        writeln!(self.output, "error: {}", self.error_count())?;
        writeln!(self.output, "")?;
        Ok(())
    }

    fn print_detail(&mut self) -> Result<()> {
        writeln!(self.output, "--- detail ---")?;
        for (spec, res) in &self.responses {
            writeln!(self.output, "{}:", spec)?;
            writeln!(self.output, "  result: {}", res.status())?;
            writeln!(self.output, "  message: {}", res)?;
        }
        for (spec, e) in &self.errors {
            writeln!(self.output, "{}:", spec)?;
            writeln!(self.output, "  result: error")?;
            writeln!(self.output, "  message: {}", e)?;
        }
        writeln!(self.output, "")?;
        Ok(())
    }

    pub fn is_all_success(&self) -> bool {
        self.responses.values().all(|res| match res {
            SpecResponse::Success { message: _ } => true,
            _ => false,
        })
    }

    fn total_count(&self) -> usize {
        self.success_count() + self.fail_count() + self.error_count()
    }

    fn success_count(&self) -> usize {
        let success_responses = self.responses.values().filter(|res| match res {
            SpecResponse::Success { message: _ } => true,
            _ => false,
        });

        success_responses.count()
    }

    fn fail_count(&self) -> usize {
        let fail_responses = self.responses.values().filter(|res| match res {
            SpecResponse::Fail { message: _ } => true,
            _ => false,
        });

        fail_responses.count()
    }

    fn error_count(&self) -> usize {
        self.errors.values().len()
    }
}
