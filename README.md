# Dog Tricks API 🐕 - Error Handling

This repository contains the example code for the blog post [Error Handling in Rust and Axum](https://cbinzer.de/blog/rust-axum-error-handling).
It builds on the Dog Tricks API introduced in the [previous article](https://cbinzer.de/blog/rest-api-axum).

## Features

- A service layer between handlers and repositories
- Validation logic for create and replace operations
- A custom TrickError enum with NotFound and Validation variants
- Conversion of domain errors into consistent JSON API error responses via IntoResponse

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable recommended)
- Cargo (comes with Rust)

### Run the server

```bash
cargo run
