pub mod workflow;

use clap::Parser;
use std::path::PathBuf;
pub use workflow::*;

pub mod toml;
pub use toml::*;

pub mod rs_file;
pub use rs_file::*;

#[derive(Parser)]
#[clap(about = "创建 uv 和 maturin 的混合项目模板")]
pub struct Cli {
    #[clap(value_parser, value_name = "PROJECT")]
    project: PathBuf,
    #[clap(value_parser, value_name = "PACKAGE")]
    package: Option<String>,
}

impl Cli {
    pub fn project_path(&self) -> &PathBuf {
        &self.project
    }

    fn project_name(&self) -> &str {
        self.project.file_name().unwrap().to_str().unwrap()
    }

    pub fn package_name(&self) -> &str {
        if let Some(ref s) = self.package {
            s.as_str()
        } else {
            self.project_name()
        }
    }
}
