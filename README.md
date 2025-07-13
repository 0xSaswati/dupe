# Dupe

**A fast, extensible Rust CLI for detecting and managing duplicate files**

---

## Table of Contents

* [Overview](#overview)
* [Features](#features)
* [Tech Stack](#tech-stack)
* [Project Structure](#project-structure)
* [Installation](#installation)
* [Usage](#usage)
* [Module Breakdown](#module-breakdown)
* [Configuration](#configuration)
* [Contributing](#contributing)
* [License](#license)

---

## Overview

Dupe is a command‑line utility written in Rust for identifying, grouping, and managing duplicate files in a directory tree. It traverses file systems efficiently, computes content hashes, filters by customizable criteria, and optionally quarantines duplicates or generates summary reports.

---

## Features

* 🚀 **High performance**: Built with Rust and asynchronous file I/O
* 📂 **Directory traversal**: Recursively walks directories, handles symlinks
* 🔍 **Content hashing**: SHA‑256 based hashing for accurate duplicate detection
* 🗂️ **Custom filters**: Include/exclude files by size, extension, path patterns
* 📑 **Grouping**: Cluster duplicate sets together for easy review
* 🚫 **Quarantine**: Move duplicates to a quarantine folder for manual inspection
* 📊 **Reporting**: Generate JSON/CSV report of detected duplicates

---

## Tech Stack

* **Language**: Rust 1.70+—memory safety, performance
* **CLI Framework**: `clap` (via `structopt` in `main.rs`)
* **Async I/O**: `tokio` for non‑blocking file operations
* **Hashing**: `sha2` crate for SHA‑256
* **Filesystem**: `ignore` crate for efficient recursive traversal respecting `.gitignore`

---

## Project Structure

```
dupe-main/
├── .gitignore         # Git ignore rules
├── Cargo.toml         # Project metadata & dependencies
├── Cargo.lock         # Locked dependency graph
└── src/               # Source files
    ├── main.rs        # CLI entrypoint & argument parsing
    ├── walker.rs      # Directory traversal & file discovery
    ├── hasher.rs      # File content hashing
    ├── filter.rs      # Filter logic for include/exclude rules
    ├── group.rs       # Grouping duplicates by hash
    ├── report.rs      # Report generation (JSON/CSV)
    └── quarantine.rs  # Quarantine duplicate files
```

---

## Installation

1. **Clone the repository**

   ```bash
   git clone https://github.com/0xSaswati/dupe.git
   cd dupe-main
   ```

2. **Install Rust toolchain**

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

3. **Build the binary**

   ```bash
   cargo build --release
   ```

4. **(Optional) Install globally**

   ```bash
   cargo install --path .
   ```

---

## Usage

```bash
dupe [OPTIONS] <DIRECTORY>
```

### Common Options

| Flag               | Description                                           |
| ------------------ | ----------------------------------------------------- |
| `-e, --extension`  | Include only files with given extension (e.g., `jpg`) |
| `-s, --size-min`   | Minimum file size (bytes) to consider                 |
| `-x, --exclude`    | Exclude paths matching pattern                        |
| `--quarantine-dir` | Directory where duplicates are moved                  |
| `--report-json`    | Output JSON report to stdout                          |
| `--report-csv`     | Output CSV report to file                             |
| `-h, --help`       | Print help information                                |

### Example

```bash
# Find all duplicates in ~/photos, minimum size 1MB, move duplicates
dupe -s 1048576 --quarantine-dir ~/dupe_quarantine ~/photos --report-json > dupes.json
```

---

## Module Breakdown

### `main.rs`

* Parses CLI arguments using `clap`.
* Initializes async runtime (`tokio::main`).
* Coordinates pipeline: traverse → filter → hash → group → report/quarantine.

### `walker.rs`

* Recursively walks directories using `ignore::Walk`.
* Respects `.gitignore` and handles symlink loops.
* Returns stream of file paths.

### `hasher.rs`

* Reads files in chunks asynchronously.
* Computes SHA‑256 hash via `sha2::Sha256`.
* Returns `(filepath, hash)` tuples.

### `filter.rs`

* Applies user‑defined filters: size, extension, path patterns.
* Efficiently skips non‑matching files before hashing.

### `group.rs`

* Collects hashed files into a `HashMap<hash, Vec<path>>`.
* Identifies groups where `Vec` length > 1 (duplicates).

### `report.rs`

* Formats duplicate groups into JSON or CSV.
* Uses `serde_json` for JSON serialization.
* Writes CSV via `csv` crate.

### `quarantine.rs`

* Moves duplicate files to a designated quarantine directory.
* Preserves directory structure under quarantine root.

---

## Configuration

* All behavior is driven by CLI flags—no separate config file needed.
* Default thresholds (e.g., no size limit) can be overridden at runtime.

---

## Contributing

1. Fork the repo.
2. Create a feature branch: `git checkout -b feature/your-feature`.
3. Implement your changes and add tests.
4. Open a Pull Request.
