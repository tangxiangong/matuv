# matuv
> 创建 uv 和 maturin 的混合项目模板

使用方法
```shell
  matuv 项目名称
```
项目结构 
```
.
├── .git
├── .github
├── .gitignore
├── .python-version
├── .venv
├── Cargo.toml
├── README.md
├── hello.py
├── pyproject.toml
├── src
└── uv.lock
```


## 附
### uv 创建虚拟环境

- `uv init uvenv` 初始化
- `cd uvenv && uv venv .venv` 创建虚拟环境
- `source .venv/bin/activate` 激活环境 
- `uv add maturin`



pyproject.toml
```toml
[project]
name = "{PlaceHolder}"
version = "0.1.0"
description = "Add your description here"
readme = "README.md"
requires-python = ">=3.12"
dependencies = [
"maturin>=1.7.8",
]
```

### maturin
```shell
  maturin develop --uv
```

src/lib.rs
```rust
use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn maturin(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
```

Cargo.toml 
```toml
[package]
name = "maturin"
version = "0.1.0"
edition = "2021"
 
# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
[lib]
name = "maturin"
crate-type = ["cdylib"]
 
[dependencies]
pyo3 = "0.23.1"
```

pyproject.toml
```toml
[build-system]
requires = ["maturin>=1.7,<2.0"]
build-backend = "maturin"

[project]
name = "maturin"
requires-python = ">=3.8"
classifiers = [
    "Programming Language :: Rust",
    "Programming Language :: Python :: Implementation :: CPython",
    "Programming Language :: Python :: Implementation :: PyPy",
]
dynamic = ["version"]
[tool.maturin]
features = ["pyo3/extension-module"]
```

.github/workflows/CI.yml
```yaml
# This file is autogenerated by maturin v1.7.8
# To update, run
#
#    maturin generate-ci github
#
name: CI

on:
  push:
    branches:
      - main
      - master
    tags:
      - '*'
  pull_request:
  workflow_dispatch:

permissions:
  contents: read

jobs:
  linux:
    runs-on: ${{ matrix.platform.runner }}
    strategy:
      matrix:
        platform:
          - runner: ubuntu-22.04
            target: x86_64
          - runner: ubuntu-22.04
            target: x86
          - runner: ubuntu-22.04
            target: aarch64
          - runner: ubuntu-22.04
            target: armv7
          - runner: ubuntu-22.04
            target: s390x
          - runner: ubuntu-22.04
            target: ppc64le
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-python@v5
        with:
          python-version: 3.x
      - name: Build wheels
        uses: PyO3/maturin-action@v1
        with:
          target: ${{ matrix.platform.target }}
          args: --release --out dist --find-interpreter
          sccache: 'true'
          manylinux: auto
      - name: Upload wheels
        uses: actions/upload-artifact@v4
        with:
          name: wheels-linux-${{ matrix.platform.target }}
          path: dist

  musllinux:
    runs-on: ${{ matrix.platform.runner }}
    strategy:
      matrix:
        platform:
          - runner: ubuntu-22.04
            target: x86_64
          - runner: ubuntu-22.04
            target: x86
          - runner: ubuntu-22.04
            target: aarch64
          - runner: ubuntu-22.04
            target: armv7
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-python@v5
        with:
          python-version: 3.x
      - name: Build wheels
        uses: PyO3/maturin-action@v1
        with:
          target: ${{ matrix.platform.target }}
          args: --release --out dist --find-interpreter
          sccache: 'true'
          manylinux: musllinux_1_2
      - name: Upload wheels
        uses: actions/upload-artifact@v4
        with:
          name: wheels-musllinux-${{ matrix.platform.target }}
          path: dist

  windows:
    runs-on: ${{ matrix.platform.runner }}
    strategy:
      matrix:
        platform:
          - runner: windows-latest
            target: x64
          - runner: windows-latest
            target: x86
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-python@v5
        with:
          python-version: 3.x
          architecture: ${{ matrix.platform.target }}
      - name: Build wheels
        uses: PyO3/maturin-action@v1
        with:
          target: ${{ matrix.platform.target }}
          args: --release --out dist --find-interpreter
          sccache: 'true'
      - name: Upload wheels
        uses: actions/upload-artifact@v4
        with:
          name: wheels-windows-${{ matrix.platform.target }}
          path: dist

  macos:
    runs-on: ${{ matrix.platform.runner }}
    strategy:
      matrix:
        platform:
          - runner: macos-13
            target: x86_64
          - runner: macos-14
            target: aarch64
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-python@v5
        with:
          python-version: 3.x
      - name: Build wheels
        uses: PyO3/maturin-action@v1
        with:
          target: ${{ matrix.platform.target }}
          args: --release --out dist --find-interpreter
          sccache: 'true'
      - name: Upload wheels
        uses: actions/upload-artifact@v4
        with:
          name: wheels-macos-${{ matrix.platform.target }}
          path: dist

  sdist:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Build sdist
        uses: PyO3/maturin-action@v1
        with:
          command: sdist
          args: --out dist
      - name: Upload sdist
        uses: actions/upload-artifact@v4
        with:
          name: wheels-sdist
          path: dist

  release:
    name: Release
    runs-on: ubuntu-latest
    if: ${{ startsWith(github.ref, 'refs/tags/') || github.event_name == 'workflow_dispatch' }}
    needs: [linux, musllinux, windows, macos, sdist]
    permissions:
      # Use to sign the release artifacts
      id-token: write
      # Used to upload release artifacts
      contents: write
      # Used to generate artifact attestation
      attestations: write
    steps:
      - uses: actions/download-artifact@v4
      - name: Generate artifact attestation
        uses: actions/attest-build-provenance@v1
        with:
          subject-path: 'wheels-*/*'
      - name: Publish to PyPI
        if: ${{ startsWith(github.ref, 'refs/tags/') }}
        uses: PyO3/maturin-action@v1
        env:
          MATURIN_PYPI_TOKEN: ${{ secrets.PYPI_API_TOKEN }}
        with:
          command: upload
          args: --non-interactive --skip-existing wheels-*/*

```