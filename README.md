# Rust Data Engineering Course

This repository contains projects and exercises for the **Rust Data Engineering** course. Each module covers a different aspect of Rust programming, focusing on data structures, concurrency, data engineering libraries, and advanced applications.

## Repository Structure

The repository is organized into modules, each representing a course topic:

- **Module 1 - Rust Data Structures**: Exercises with collections like vectors, hash maps, and sets.
- **Module 2 - Safety, Security, and Concurrency**: Focus on Rust's concurrency and memory safety features.
- **Module 3 - Data Engineering Libraries and Tools**: Examples using libraries for data processing, API handling, and database interactions.
- **Module 4 - Advanced Applications**: Projects on building ETL pipelines, web servers, and machine learning models.

## Getting Started

To use the repository, navigate to a module folder and use the Makefile commands to manage each project.

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (Install via rustup)
- [cargo](https://doc.rust-lang.org/cargo/) (Rust package manager)

### Usage

Each module folder includes a `Makefile` with common commands to format, lint, test, and run the code. You can use the following commands within each module directory:

- **Show Rust Tool Versions**:
```bash
make rust-version
```