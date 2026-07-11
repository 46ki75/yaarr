//! Detects when an Agent Skill has drifted from the upstream source it was
//! written against.
//!
//! Knowledge skills (e.g. the `ai-protocols` plugin) are hand-curated digests
//! of upstream protocol repositories tracked as git submodules. Each such skill
//! carries a `.sources.json` file recording, per upstream repo, the commit SHA
//! the skill currently reflects (`synced`) and the upstream paths that feed the
//! skill (`paths`). When the submodule pin advances past `synced` and the change
//! touches one of those paths, the skill is considered *stale* and should be
//! refreshed.
//!
//! The file is named with a leading dot so the archiver's `is_hidden` filter
//! keeps it out of published skill ZIPs — it is repo metadata, not skill
//! content.

use std::path::{Path, PathBuf};

use anyhow::Context;
use serde::{Deserialize, Serialize};

use crate::pipeline::discover_skill_dirs;

/// Per-skill provenance file name. Hidden so it is excluded from published
/// archives (see [`skill_archiver`]'s `is_hidden`).
pub const SOURCES_FILE: &str = ".sources.json";

/// Parsed `.sources.json` contents.
#[derive(Debug, Deserialize)]
struct SkillSources {
    /// Optional human-facing skill name; falls back to the directory name.
    #[serde(default)]
    skill: Option<String>,
    sources: Vec<SourceSpec>,
}

/// One upstream source the skill is derived from.
#[derive(Debug, Deserialize)]
struct SourceSpec {
    /// Path to the submodule, relative to the repository root (e.g.
    /// `submodules/modelcontextprotocol`).
    repo: String,
    /// Commit SHA the skill currently reflects.
    synced: String,
    /// Upstream pathspecs (directories or files) that feed the skill. Drift is
    /// only reported when the diff touches one of these.
    paths: Vec<String>,
}

/// The full drift report emitted by [`detect`].
#[derive(Debug, Serialize)]
pub struct DriftReport {
    /// True when at least one skill has a source with changed files.
    pub stale: bool,
    /// Skills with drift and/or errors. Up-to-date skills are omitted.
    pub skills: Vec<SkillDrift>,
}

/// Drift for a single skill.
#[derive(Debug, Serialize)]
pub struct SkillDrift {
    /// Skill name (`skill` field of `.sources.json`, else directory name).
    pub skill: String,
    /// Skill directory, relative to the repository root.
    pub dir: String,
    pub sources: Vec<SourceDrift>,
}

/// Drift for one upstream source of a skill.
#[derive(Debug, Serialize)]
pub struct SourceDrift {
    pub repo: String,
    pub synced: String,
    pub current: String,
    /// Upstream files changed between `synced` and `current` within `paths`.
    pub changed: Vec<String>,
    /// Populated when drift could not be determined (e.g. a missing commit).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Scans every skill for a `.sources.json` and compares its `synced` SHA against
/// the current submodule pin over the recorded paths.
pub async fn detect(skills_dir: &Path, plugins_dir: &Path) -> anyhow::Result<DriftReport> {
    let dirs = discover_skill_dirs(skills_dir, plugins_dir).await?;

    let mut skills: Vec<SkillDrift> = Vec::new();
    let mut stale = false;

    for dir in &dirs {
        let path = dir.join(SOURCES_FILE);
        let raw = match tokio::fs::read_to_string(&path).await {
            Ok(s) => s,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                tracing::debug!(path = %dir.display(), "no .sources.json; skipping");
                continue;
            }
            Err(e) => return Err(e).with_context(|| format!("reading {}", path.display())),
        };

        let parsed: SkillSources =
            serde_json::from_str(&raw).with_context(|| format!("parsing {}", path.display()))?;

        let name = parsed.skill.clone().unwrap_or_else(|| {
            dir.file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_else(|| dir.display().to_string())
        });

        let mut source_drifts: Vec<SourceDrift> = Vec::new();
        for spec in &parsed.sources {
            match source_drift(spec).await {
                Ok(Some(d)) => {
                    if !d.changed.is_empty() {
                        stale = true;
                    }
                    source_drifts.push(d);
                }
                Ok(None) => {}
                Err(e) => {
                    tracing::error!(skill = %name, repo = %spec.repo, "{e:#}");
                    source_drifts.push(SourceDrift {
                        repo: spec.repo.clone(),
                        synced: spec.synced.clone(),
                        current: String::new(),
                        changed: Vec::new(),
                        error: Some(format!("{e:#}")),
                    });
                }
            }
        }

        if !source_drifts.is_empty() {
            skills.push(SkillDrift {
                skill: name,
                dir: dir.display().to_string(),
                sources: source_drifts,
            });
        }
    }

    Ok(DriftReport { stale, skills })
}

/// Computes drift for a single source. Returns `Ok(None)` when the pin still
/// matches `synced` or no recorded path changed.
async fn source_drift(spec: &SourceSpec) -> anyhow::Result<Option<SourceDrift>> {
    let repo = PathBuf::from(&spec.repo);
    if !tokio::fs::try_exists(&repo).await.unwrap_or(false) {
        anyhow::bail!(
            "submodule {} not found (run `git submodule update --init`)",
            spec.repo
        );
    }

    let current = git(&repo, &["rev-parse", "HEAD"])
        .await
        .with_context(|| format!("resolving HEAD of {}", spec.repo))?;

    if current == spec.synced {
        return Ok(None);
    }

    let mut args: Vec<&str> = vec!["diff", "--name-only", &spec.synced, &current, "--"];
    for p in &spec.paths {
        args.push(p);
    }
    let out = git(&repo, &args)
        .await
        .with_context(|| format!("diffing {}..{} in {}", spec.synced, current, spec.repo))?;

    let changed: Vec<String> = out.lines().map(|l| l.to_string()).collect();
    if changed.is_empty() {
        return Ok(None);
    }

    Ok(Some(SourceDrift {
        repo: spec.repo.clone(),
        synced: spec.synced.clone(),
        current,
        changed,
        error: None,
    }))
}

/// Runs `git -C <repo> <args...>` and returns trimmed stdout, erroring on a
/// non-zero exit.
async fn git(repo: &Path, args: &[&str]) -> anyhow::Result<String> {
    let out = tokio::process::Command::new("git")
        .arg("-C")
        .arg(repo)
        .args(args)
        .output()
        .await
        .with_context(|| format!("running git {}", args.join(" ")))?;

    if !out.status.success() {
        let stderr = String::from_utf8_lossy(&out.stderr);
        anyhow::bail!("git {} failed: {}", args.join(" "), stderr.trim());
    }

    Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

/// Renders a human-readable summary of the report to the log.
pub fn log_summary(report: &DriftReport) {
    if !report.stale && report.skills.is_empty() {
        tracing::info!("all skills are in sync with their upstream sources");
        return;
    }

    for skill in &report.skills {
        for src in &skill.sources {
            if let Some(err) = &src.error {
                tracing::warn!(skill = %skill.skill, repo = %src.repo, "drift undetermined: {err}");
            } else {
                tracing::info!(
                    skill = %skill.skill,
                    repo = %src.repo,
                    "stale: {}..{} touched {} file(s) under tracked paths",
                    short(&src.synced),
                    short(&src.current),
                    src.changed.len(),
                );
            }
        }
    }
}

fn short(sha: &str) -> &str {
    sha.get(..7).unwrap_or(sha)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drift_report_serializes_to_expected_shape() {
        let report = DriftReport {
            stale: true,
            skills: vec![SkillDrift {
                skill: "mcp-knowledge".into(),
                dir: "plugins/ai-protocols/skills/mcp-knowledge".into(),
                sources: vec![SourceDrift {
                    repo: "submodules/modelcontextprotocol".into(),
                    synced: "aaaaaaa".into(),
                    current: "bbbbbbb".into(),
                    changed: vec!["docs/specification/draft/schema.mdx".into()],
                    error: None,
                }],
            }],
        };

        let v: serde_json::Value = serde_json::to_value(&report).unwrap();
        assert_eq!(v["stale"], serde_json::json!(true));
        assert_eq!(v["skills"][0]["skill"], "mcp-knowledge");
        assert_eq!(
            v["skills"][0]["sources"][0]["changed"][0],
            "docs/specification/draft/schema.mdx"
        );
        // `error` is omitted when absent.
        assert!(v["skills"][0]["sources"][0].get("error").is_none());
    }

    #[test]
    fn source_spec_parses() {
        let raw = r#"{
            "skill": "demo",
            "sources": [
                { "repo": "submodules/x", "synced": "abc", "paths": ["docs", "schema"] }
            ]
        }"#;
        let parsed: SkillSources = serde_json::from_str(raw).unwrap();
        assert_eq!(parsed.skill.as_deref(), Some("demo"));
        assert_eq!(parsed.sources.len(), 1);
        assert_eq!(parsed.sources[0].paths, vec!["docs", "schema"]);
    }

    #[test]
    fn short_truncates_and_tolerates_tiny_input() {
        assert_eq!(short("0123456789"), "0123456");
        assert_eq!(short("abc"), "abc");
    }
}
