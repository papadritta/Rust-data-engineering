
# Rust Data Engineering Course
![366938608-5d41b6e5-bd41-4a5a-9f1c-50ec7e21ebfc](https://github.com/user-attachments/assets/44d3f44b-34e4-49da-b177-61182c7f8214)

This repository contains projects and exercises for the **Rust Data Engineering** course. Each module covers a different aspect of Rust programming, focusing on data structures, concurrency, data engineering libraries, and advanced applications.

## Repository Structure

The repository is organized into modules, each representing a course topic:

- **[Module 1 - Rust Data Structures](Module1-Rust-Data-Structures)**: Exercises with collections like vectors, hash maps, and sets.
- **[Module 2 - Safety, Security, and Concurrency](Module2-Safety-Security-Concurrency)**: Focus on Rust's concurrency and memory safety features.
- **[Module 3 - Data Engineering Libraries and Tools](Module3-Data-Engineering-Libraries-Tools)**: Examples using libraries for data processing, API handling, and database interactions.
- **[Module 4 - Advanced Applications](Module4-Advanced-Applications)**: Projects on building ETL pipelines, web servers, and machine learning models.

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
- **Format Code**:
```bash
make format
```
- **Lint Code**:
```bash
make lint
```
- **Run Tests**:
```bash
make test
```
- **Run Application**:
```bash
make run
```
- **Build for Release**:
```bash
make release
```
## Modules Overview

### Module 1: Rust Data Structures
Contains examples and exercises to work with Rust's built-in data structures:
#### examples:
- [vector-fruit-salad](Module1-Rust-Data-Structures/vector-fruit-salad)
- [vecdeque-fruit-salad](Module1-Rust-Data-Structures/vecdeque-fruit-salad)
- [linked-list-fruit-salad](Module1-Rust-Data-Structures/linked-list-fruit-salad)
- [cli-salad](Module1-Rust-Data-Structures/cli-salad)
- [hashmap-count](Module1-Rust-Data-Structures/hashmap-count)
- [hashmap-language](Module1-Rust-Data-Structures/hashmap-language)
#### exercises:
- [grocery-list](Module1-Rust-Data-Structures/exercises/grocery-list)
- [phone-book](Module1-Rust-Data-Structures/exercises/phone-book)
- [queue-simulation](Module1-Rust-Data-Structures/exercises/queue-simulation)
- [word-count](Module1-Rust-Data-Structures/exercises/word-count)
- [rgb-colors](Module1-Rust-Data-Structures/exercises/rgb-colors)

### Module 2: Safety, Security, and Concurrency
Explores Rust’s memory safety and concurrency:

- mutable-fruit-salad
- data-race-example
- cli-customize-fruit-salad

### Module 3: Data Engineering Libraries and Tools
Uses Rust libraries for data handling:

- csv-data-processing
- database-interaction
- data-visualization

### Module 4: Advanced Applications
Advanced projects for real-world Rust applications:

- etl-pipeline
- rest-api-server
- machine-learning

## Contributing

Contributions are welcome! Feel free to open issues, submit PRs, or suggest improvements.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
