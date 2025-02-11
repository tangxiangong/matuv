use anyhow::{Context, Result};
use clap::Parser;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Confirm;
use matuv::{make_ci, make_py_toml, make_rs_file, make_rs_toml, Cli};
use std::fs::exists;
use std::process::Command;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let path = cli.project_path();
    let path_str = path.to_str().unwrap();
    let package_name = cli.package_name();

    let is_already_exist = exists(path)?;
    if is_already_exist {
        if Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(format!(
                "The dictionary {} will be removed as it is already exist, \
             do you want to continue?",
                path_str
            ))
            .default(true)
            .show_default(true)
            .wait_for_newline(true)
            .interact()?
        {
            println!("The dictionary is already exist, deleting...!");
            let _ = Command::new("rm")
                .arg("-rf")
                .arg(path)
                .output()
                .with_context(|| format!("could not delete {}", path_str))?;
        } else {
            println!("nevermind then :(");
            return Ok(());
        }
    }
    let _ = Command::new("uv")
        .arg("init")
        .arg(path_str)
        .output()
        .with_context(|| format!("could not initialize uv project {}", path_str))?;
    println!("uv init..... OK");
    let _ = Command::new("uv")
        .current_dir(path_str)
        .arg("add")
        .arg("maturin")
        .output()
        .with_context(|| "could not add maturin using uv")?;
    println!("add maturin..... OK");
    make_ci(path_str).await?;
    make_rs_toml(path_str, &package_name).await?;
    make_rs_file(path_str, &package_name).await?;
    make_py_toml(path_str).await?;
    println!("All DONE!");
    Ok(())
}
