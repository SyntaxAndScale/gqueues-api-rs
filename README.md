# GQueues API Client for Rust

A Rust client for the [GQueues](https://www.gqueues.com/) API.

## Disclaimer
This is an unofficial library and is not affiliated with GQueues.
This project is entirely AI generated and not manually reviewed. No guarantees are made or liability assumed. Use at your own risk.
The GQueues API is in private beta as of 2026-05-08.

## Features

- **Asynchronous API**: Built on top of `reqwest` and `tokio`.
- **Error Handling**: Specific error types using `thiserror`.
- **Configurable Client**: Builder pattern for custom timeouts, user-agents, and proxies.
- **Idempotency Support**: Safely retry task creation without duplicates.
- **Comprehensive Models**: Fully typed request and response structures.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
gqueues-api-rs = { git = "https://github.com/SyntaxAndScale/gqueues-api-rs", tag = "v0.2.0" }
tokio = { version = "1.0", features = ["full"] }
```

## Etc
Official Gqueues API Documentation can be found at: https://learn.gqueues.com/en/articles/14730378-gqueues-rest-api-reference