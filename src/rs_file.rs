use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::process::Command;
use anyhow::{Context, Result};

pub async fn make_rs_file(name: &str) -> Result<()> {
    let _ = Command::new("mkdir").current_dir(name).arg("src").output()
        .with_context(|| format!("could not make directory {}/src", name));
    let path_str = format!("{}/src/lib.rs", name);
    let path = Path::new(&path_str);
    let file = File::create(path)
        .with_context(|| format!("could not create file lib.rs in {}/src", name))?;
    let mut buff_writer = BufWriter::new(file);

    let code = format!("use pyo3::prelude::*;
/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {{
    Ok((a + b).to_string())
}}

/// A Python module implemented in Rust.
#[pymodule]
fn {}(m: &Bound<'_, PyModule>) -> PyResult<()> {{
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}}
", name);

    buff_writer.write_all(code.as_bytes())
        .with_context(|| format!("could not write content into file lib.rs in {}/src", name))?;
    println!("make template src/lib.rs..... OK");
    Ok(())
}