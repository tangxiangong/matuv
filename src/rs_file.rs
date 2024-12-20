use anyhow::{Context, Result};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::process::Command;

pub async fn make_rs_file(path_str: &str, package_name: &str) -> Result<()> {
    let _ = Command::new("mkdir")
        .current_dir(path_str)
        .arg("src")
        .output()
        .with_context(|| format!("could not make directory {}/src", path_str));
    let file_path_str = format!("{}/src/lib.rs", path_str);
    let file_path = Path::new(&file_path_str);
    let file = File::create(file_path)
        .with_context(|| format!("could not create file lib.rs in {}/src", path_str))?;
    let mut buff_writer = BufWriter::new(file);

    let code = format!(
        "use pyo3::prelude::*;
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
",
        package_name
    );

    buff_writer.write_all(code.as_bytes()).with_context(|| {
        format!(
            "could not write content into file lib.rs in {}/src",
            path_str
        )
    })?;
    println!("make template src/lib.rs..... OK");
    Ok(())
}
