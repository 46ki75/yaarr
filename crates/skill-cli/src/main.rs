//! CLI orchestrator that validates, archives, and publishes Agent Skills from
//! both channels: standalone skills under `skills/` and plugin-bundled skills
//! under `plugins/*/skills/`.
//!
//! Subcommands:
//!
//! - `check` — parse and validate every skill; exit non-zero on any failure.
//! - `build` — write a `<name>-v<version>.zip` per valid skill into `dist/`;
//!   aborts before touching `dist/` if any skill fails validation.
//! - `upload` — build, then for each artifact:
//!   - create the release and upload the ZIP if no release with that tag exists,
//!   - upload the ZIP to the existing release if the tag exists but the ZIP
//!     asset is missing (orphan-asset recovery),
//!   - skip if the release and asset are already present.
//! - `sources` — report skills that have drifted from the upstream submodule
//!   commit they were written against (see each skill's `.sources.json`).
//!
//! Logging is controlled via `RUST_LOG` (defaults to `info`). Exit codes:
//! `0` success, `1` validation failure surfaced via `check`/`build`, `2` any
//! unhandled error from the CLI.

mod cli;
mod github;
mod pipeline;
mod sources;

use std::process::ExitCode;

use clap::Parser;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> ExitCode {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let args = cli::Cli::parse();

    match cli::run(args).await {
        Ok(code) => code,
        Err(e) => {
            tracing::error!("{e:#}");
            ExitCode::from(2)
        }
    }
}
