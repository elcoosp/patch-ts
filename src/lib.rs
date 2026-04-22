pub mod marker;
pub mod matching;
pub mod ast;
pub mod cli;
pub mod diagnostics;
pub mod file;
pub mod patch;
pub mod repair;

pub use cli::run;
pub mod config;
pub mod watch;
