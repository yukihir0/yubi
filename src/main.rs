extern crate clap;
extern crate env_logger;

use anyhow::{Context, Result};
use clap::Parser;
use log;
use std::fs;
use std::io::{stdout, BufWriter, Write};
use yubi::report::Report;
use yubi::spec::Spec;

enum ExitStatus {
    Success = 0,
    Failure = 1,
}

#[derive(Parser, Debug)]
#[clap(about, author, version)]
struct Args {
    #[clap(name = "SPEC_FILE", help = "Path to specfile")]
    specfile: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    log::debug!("start process");

    log::debug!("parse command line args");
    let args = Args::parse();

    log::debug!("parse specfile");
    let specfile = fs::read_to_string(&args.specfile)
        .with_context(|| format!("failed to open specfile: {}", &args.specfile))?;
    let specs = serde_yaml::from_str::<Vec<Spec>>(&specfile)
        .with_context(|| format!("failed to parse specfile: {}", &args.specfile))?;

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
