#[macro_use]
extern crate clap;
extern crate env_logger;

use anyhow::{Context, Result};
use clap::{App, Arg};
use log;
use std::fs;
use std::io;
use yubi::report::Report;
use yubi::spec::Spec;

enum StatusCode {
    Success = 0,
    Failure = 1,
}

fn parse_specfile(path: &str) -> Result<Vec<Spec>> {
    let specfile =
        fs::read_to_string(path).with_context(|| format!("failed to open specfile: {}", path))?;
    let specs = serde_yaml::from_str::<Vec<Spec>>(&specfile)
        .with_context(|| format!("failed to parse specfile: {}", path))?;
    Ok(specs)
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

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
    let path = args.value_of("specfile").unwrap();

    log::debug!("parse specfile");
    let specs = parse_specfile(path)?;

    log::debug!("initialize report");
    let mut stdout = io::stdout();
    let mut report = Report::new(&mut stdout);

    log::debug!("check specs");
    for spec in specs {
        match spec.check().await {
            Ok(res) => {
                report.add_response(spec, res);
            }
            Err(e) => {
                report.add_error(spec, e);
            }
        }
    }

    log::debug!("print report");
    report.print()?;

    log::debug!("exit with status code");
    std::process::exit(if report.is_all_green() {
        StatusCode::Success as i32
    } else {
        StatusCode::Failure as i32
    });
}
