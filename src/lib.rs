//! # GQueues API Client
//!
//! A Rust client for the GQueues API.
//!
//! This library provides a convenient way to interact with GQueues, allowing you to:
//! - Fetch personal, team, and shared queues.
//! - List active tasks in a specific queue.
//! - Create new tasks (with idempotency support).
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use gqueues_api_rs::GqueuesClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = GqueuesClient::builder("YOUR_ACCESS_TOKEN")
//!         .build()?;
//!
//!     let queues = client.get_queues().await?;
//!     for queue in queues {
//!         println!("Queue: {}", queue.name);
//!     }
//!
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod models;

pub use client::{GqueuesClient, GqueuesClientBuilder, GqueuesError};
pub use models::{Queue, Task};
