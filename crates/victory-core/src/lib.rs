//! Shared plumbing for every VIctory crate and tool.
//!
//! * [`config`]: where the user's files live (never inside the repository).
//! * [`policy`]: the runtime half of rule #1, refusing to write
//!   game-derived data into the source tree.
//! * [`logging`]: one tracing setup for all binaries.

pub mod config;
pub mod logging;
pub mod policy;

pub use config::Config;
pub use policy::{PolicyError, ensure_outside_repo};

// INJECTED FAULT (CI check, to be dropped): a clippy warning and a failing test.
pub fn injected_fault() -> u32 {
    let x: u32 = 5;
    #[allow(clippy::let_and_return)]
    let y = x.clone();
    y
}

#[cfg(test)]
mod injected_fault_tests {
    #[test]
    fn injected_failure() {
        assert_eq!(super::injected_fault(), 6);
    }
}
