use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::Path;
use anyhow::{Context, Result};

pub async fn make_rs_toml(name: &str) -> Result<()> {
    let file_path = format!("{}/Cargo.toml", name);
    let path = Path::new(&file_path);
    let file = File::create(path)
        .with_context(|| format!("could not create file Cargo.toml in {}/", name))?;
    let mut buff_writer = BufWriter::new(file);
    let toml = format!("[package]
name = \"{}\"
version = \"0.1.0\"
edition = \"2021\"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
[lib]
name = \"{}\"
crate-type = [\"cdylib\"]

[dependencies]
pyo3 = \"0.23.1\"\
", name, name).into_bytes();
    buff_writer.write_all(&toml)
        .with_context(|| format!("could not write content into file Cargo.toml in {}/", name))?;
    println!("make Cargo.toml..... OK");
    Ok(())
}

pub async fn make_py_toml(name: &str) -> Result<()> {
    let path = format!("{}/pyproject.toml", name);
    let mut file = OpenOptions::new().append(true).open(&path)
        .with_context(|| format!("could not open file pyproject.toml in {}/", name))?;
    let toml = "classifiers = [
\"Programming Language :: Rust\",
\"Programming Language :: Python :: Implementation :: CPython\",
\"Programming Language :: Python :: Implementation :: PyPy\",
]
dynamic = [\"version\"]
[tool.maturin]
features = [\"pyo3/extension-module\"]
[build-system]
requires = [\"maturin>=1.7,<2.0\"]
build-backend = \"maturin\"
";
    file.write_all(toml.as_bytes())
        .with_context(|| format!("could not write content into file pyproject.toml in {}/", name))?;
    println!("make pyproject.toml..... OK");
    Ok(())
}