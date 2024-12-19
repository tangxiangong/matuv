use std::fs::exists;
use std::process::Command;
use anyhow::{Context, Result};
use structopt::StructOpt;
use matuv::{make_ci, make_py_toml, make_rs_file, make_rs_toml, Cli};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::from_args();
    let name = cli.arg();
    let is_already_exist = exists(name)?;
    if is_already_exist {
        println!("The dictionary is already exist, deleting...!");
        let _ = Command::new("rm").arg("-rf").arg(name).output()
            .with_context(|| format!("could not delete {}", name))?;
    }
    // let _ = Command::new("conda").arg("activate").output()?;
    let _ = Command::new("uv").arg("init").arg(name).output()
        .with_context(|| format!("could not initialize uv project {}", name))?;
    println!("uv init..... OK");
    let _ = Command::new("uv").current_dir(name).arg("add").arg("maturin").output()
        .with_context(|| "could not add maturin using uv")?;
    println!("add maturin..... OK");
    make_ci(name).await?;
    make_rs_toml(name).await?;
    make_rs_file(name).await?;
    make_py_toml(name).await?;
    println!("All DONE!");
    Ok(())
}
