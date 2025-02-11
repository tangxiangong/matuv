use anyhow::{Context, Result};
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::Path;
use std::process::Command;

pub async fn make_rs_toml(path_str: &str, package_name: &str) -> Result<()> {
    let file_path_str = format!("{}/Cargo.toml", path_str);
    let file_path = Path::new(&file_path_str);
    let file = File::create(file_path)
        .with_context(|| format!("could not create file Cargo.toml in {}/", path_str))?;
    let mut buff_writer = BufWriter::new(file);
    let toml = format!(
        "[package]
name = \"{}\"
version = \"0.1.0\"
edition = \"2024\"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
[lib]
name = \"{}\"
crate-type = [\"cdylib\"]

[dependencies]
pyo3 = \"0.23.1\"\
",
        package_name, package_name
    )
    .into_bytes();
    buff_writer.write_all(&toml).with_context(|| {
        format!(
            "could not write content into file Cargo.toml in {}/",
            path_str
        )
    })?;
    println!("make Cargo.toml..... OK");
    Ok(())
}

pub async fn make_py_toml(path_str: &str) -> Result<()> {
    let file_path = format!("{}/pyproject.toml", path_str);
    let mut file = OpenOptions::new()
        .append(true)
        .open(&file_path)
        .with_context(|| format!("could not open file pyproject.toml in {}/", path_str))?;

    let git_user_name = match Command::new("git")
        .arg("config")
        .arg("user.name")
        .output() {
            Ok(output) => String::from_utf8(output.stdout).unwrap_or("".to_owned()),
            Err(_) => "".to_owned(),
        };
    let git_user_email = match Command::new("git")
        .arg("config")
        .arg("user.email")
        .output() {
            Ok(output) => String::from_utf8(output.stdout).unwrap_or("".to_owned()),
            Err(_) => "".to_owned(),
        };
    let authors = format!("authors = [ {{ name = \"{}\", email = \"{}\" }} ]", git_user_name, git_user_email);
    let toml = r#"classifiers = [
"Programming Language :: Rust",
"Programming Language :: Python :: Implementation :: CPython",
"Programming Language :: Python :: Implementation :: PyPy",
]
[tool.maturin]
features = ["pyo3/extension-module"]
[build-system]
requires = ["maturin>=1.7,<2.0"]
build-backend = "maturin"
"#.to_owned();
    let toml = format!("{}\n{}", authors, toml);
    file.write_all(toml.as_bytes()).with_context(|| {
        format!(
            "could not write content into file pyproject.toml in {}/",
            path_str
        )
    })?;
    println!("make pyproject.toml..... OK");
    Ok(())
}
