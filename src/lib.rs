pub mod workflow;

use clap::Parser;
use std::path::PathBuf;
pub use workflow::*;

pub mod toml;
pub use toml::*;

pub mod rs_file;
pub use rs_file::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// The project path you want to take
    #[arg(value_parser, value_name = "PROJECT PATH")]
    project: PathBuf,

    /// The name of crate that will be used as python package
    #[arg(value_name = "PYTHON PACKAGE NAME", short = 'n', long = "package-name")]
    package: Option<String>,
}

impl Cli {
    pub fn project_path(&self) -> &PathBuf {
        &self.project
    }

    fn project_name(&self) -> &str {
        self.project.file_name().unwrap().to_str().unwrap()
    }

    pub fn package_name(&self) ->String {
        let raw_str = if let Some(ref s) = self.package {
            s.clone()
        } else {
            format!("{}_core", self.project_name())
        };
        raw_str.replace("-", "_")
    }
}
