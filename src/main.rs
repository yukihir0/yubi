#[macro_use]
extern crate clap;
extern crate env_logger;

use anyhow::{Context, Result};
use clap::{App, Arg};
use log;
use std::fs;
use std::io::{stdout, BufWriter, Write};
use yubi::report::Report;
use yubi::spec::Spec;

enum ExitStatus {
    Success = 0,
    Failure = 1,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    log::debug!("start process");

    log::debug!("parse command line args");
    let args = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::new("specfile")
                .about("path to specfile")
                .value_name("SPEC_FILE")
                .takes_value(true)
                .required(true)
                .index(1),
        )
        .get_matches();

    log::debug!("parse specfile");
    let specfile_path = args
        .value_of("specfile")
        .with_context(|| format!("failed to get specfile path"))?;
    let specfile = fs::read_to_string(specfile_path)
        .with_context(|| format!("failed to open specfile: {}", specfile_path))?;
    let specs = serde_yaml::from_str::<Vec<Spec>>(&specfile)
        .with_context(|| format!("failed to parse specfile: {}", specfile_path))?;

    log::debug!("check specs");
    let mut report = Report::new();
    for spec in specs {
        match spec.check().await {
            Ok(spec_result) => {
                report.record_ok(spec, spec_result);
            }
            Err(error) => {
                report.record_ng(spec, error);
            }
        }
    }

    log::debug!("print report");
    let out = stdout();
    let mut out = BufWriter::new(out.lock());
    writeln!(out, "{}", serde_yaml::to_string(&report)?)?;
    out.flush()?;

    log::debug!("exit process");
    if report.is_all_green() {
        std::process::exit(ExitStatus::Success as i32)
    } else {
        std::process::exit(ExitStatus::Failure as i32)
    }
}
