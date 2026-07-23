//! Kernel OS Cognitive ABI
//!
//! Stable interface between AI applications,
//! agents, runtimes and Kernel OS.

pub mod abi;
pub mod command;
pub mod response;
pub mod error;
pub mod dispatcher;

pub use abi::*;
pub use command::*;
pub use response::*;
pub use error::*;
pub use dispatcher::*;

pub const ABI_VERSION: &str = "0.1.0";
