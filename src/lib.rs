pub mod workflow;

use std::path::PathBuf;
use structopt::StructOpt;
pub use workflow::*;

pub mod toml;
pub use toml::*;

pub mod rs_file;
pub use rs_file::*;

#[derive(StructOpt)]
pub struct Cli {
    #[structopt(parse(from_os_str))]
    project: PathBuf,
}

impl Cli {
    pub fn arg(&self) -> &str {
        self.project.to_str().unwrap()
    }
}

// pub fn commands(name: &str) -> Result<(), Box<dyn std::error::Error>> {
//     let _ = Command::new("uv").current_dir(name).arg("sync").output()?;
//     let _ = Command::new("conda").arg("deactivate").output()?;
//     Ok(())
// }