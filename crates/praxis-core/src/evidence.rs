//! `TS.260823.02` — a claim settles on a run the tool watched, not on a sentence about one.
//!
//! `attach-evidence` said it in its own words: *"It does not judge the evidence."* So a claim
//! settled on prose, and prose is what `S2` is about — an artifact looks identical whether
//! the agent reasoned hard or pattern-matched a template, and
//!
//! ```text
//! evidence="46 tests pass; the rule fails closed on an undeclared layer"
//! ```
//!
//! looks identical whether 46 tests pass or none exist. The validation review closed an
//! iteration clean, with three evidenced layers and a met claim, in a repository holding no
//! application code at all.
//!
//! The no-paths rule stays and is right: `the-record-cites-what-it-holds` was learned from a
//! hundred and fifty-six dead pointers. But *"a path rots"* argues for something
//! **reproducible**, not for prose — and prose is a claim by whoever produced the artifact,
//! relocated into KDL. The third option is the one this record already uses for published
//! documents: run it, and hold what came back.
//!
//! # What is NOT run
//!
//! Anything the caller names. The repository declares its verification once, in the config,
//! and the flag SELECTS within it. An arbitrary command string would make this engine a
//! general command runner driven by the agent it exists to check — the agent could name
//! `true` and settle every claim in the record.

/// How a repository verifies. Declared once, in the config, and never on the command line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Verification {
    /// The entry point, as it is written in the config.
    pub runner: String,
    /// How a selector reaches the runner, if it can take one. `None` means the runner is
    /// whole-suite only and a selector is refused rather than silently dropped.
    pub select_with: Option<String>,
}

/// One run this tool watched: what was executed, how it exited, and a digest of what it said.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Witnessed {
    pub command: String,
    pub exit: i32,
    pub digest: String,
    pub at: String,
}

impl Witnessed {
    /// Whether this run settles a claim. Only a clean exit does.
    ///
    /// A failing run is a FACT and is recorded as one — what it is not is evidence that the
    /// claim holds. The claim stays pending and the refusal names the status, so a red run
    /// is visible rather than absent.
    pub fn settles(&self) -> bool {
        self.exit == 0
    }
}

/// Why a claim could not be witnessed, when it could not.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Unwitnessed {
    /// The repository declares no verification, so there is nothing to run. The claim may
    /// still settle on prose — and the record SAYS it was reported rather than witnessed,
    /// which is the whole point of distinguishing them.
    NothingDeclared,
    /// A selector was given to a runner that takes none. Refused rather than dropped: a
    /// selector silently ignored runs the whole suite and reports it as the one test.
    RunnerTakesNoSelector { runner: String },
}

/// Read the verification a repository declares.
///
/// Absent is a legitimate answer, not an error. A repository that has not declared one keeps
/// prose evidence and is marked as reporting rather than witnessing.
pub fn verification(docs: &[kdl::KdlDocument]) -> Option<Verification> {
    for doc in docs {
        for config in doc.nodes().iter().filter(|n| n.name().value() == "config") {
            let body = config.children()?;
            let block = body.nodes().iter().find(|n| n.name().value() == "verification")?;
            let inner = block.children()?;
            let arg = |name: &str| -> Option<String> {
                inner
                    .nodes()
                    .iter()
                    .find(|n| n.name().value() == name)
                    .and_then(|n| n.entries().first())
                    .and_then(|e| e.value().as_string())
                    .map(str::to_owned)
            };
            let runner = arg("runner")?;
            return Some(Verification { runner, select_with: arg("select-with") });
        }
    }
    None
}

/// The exact command to run, as argv, for a selector or for the whole suite.
///
/// Returned as a vector rather than a string so the shell never has to re-split it — a
/// selector containing a space would otherwise become two arguments, and the run that
/// settled the claim would not be the run anybody meant.
pub fn command_for(
    verification: &Verification,
    selector: Option<&str>,
) -> Result<Vec<String>, Unwitnessed> {
    let mut argv: Vec<String> =
        verification.runner.split_whitespace().map(str::to_owned).collect();
    match (selector, verification.select_with.as_deref()) {
        (None, _) => Ok(argv),
        (Some(sel), Some(flag)) => {
            argv.extend(flag.split_whitespace().map(str::to_owned));
            argv.push(sel.to_owned());
            Ok(argv)
        }
        (Some(_), None) => {
            Err(Unwitnessed::RunnerTakesNoSelector { runner: verification.runner.clone() })
        }
    }
}

/// A digest of what a run printed.
///
/// FNV-1a, 64-bit, as `seal` uses and for the same reason: the property wanted is *a
/// different run produces a different digest*, and no dependency is worth more than that.
///
/// It is not a signature. Anyone who can edit the record can run the same function — what it
/// catches is the value written from memory, because the agent does not know it until the run
/// has happened. That is the property that matters here, and it is the one prose never had.
///
/// The digest is of the OUTPUT. Never of a file: a digest over a path is a citation the record
/// cannot follow, which is the rule `the-record-cites-what-it-holds` already refuses.
pub fn digest(output: &str) -> String {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in output.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x1000_0000_01b3);
    }
    format!("fnv1a64:{hash:016x}")
}

/// The KDL a witnessed run contributes to a claim.
pub fn witnessed_kdl(run: &Witnessed) -> String {
    format!(
        "        ran {:?}\n        exited {}\n        digest {:?}\n        witnessed-at {:?}\n",
        run.command, run.exit, run.digest, run.at
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_different_run_digests_differently() {
        assert_ne!(digest("5 passed"), digest("4 passed"));
        assert_eq!(digest("5 passed"), digest("5 passed"));
    }

    #[test]
    fn a_failing_run_does_not_settle() {
        let run = |exit| Witnessed {
            command: "cargo test".into(),
            exit,
            digest: digest(""),
            at: "now".into(),
        };
        assert!(run(0).settles());
        assert!(!run(1).settles(), "a red run is a fact, and it is not evidence the claim holds");
    }

    #[test]
    fn a_selector_reaches_the_runner_as_its_own_argument() {
        let v = Verification {
            runner: "cargo test --quiet".into(),
            select_with: Some("--test".into()),
        };
        assert_eq!(
            command_for(&v, Some("the_admission")).unwrap(),
            vec!["cargo", "test", "--quiet", "--test", "the_admission"]
        );
        assert_eq!(command_for(&v, None).unwrap(), vec!["cargo", "test", "--quiet"]);
    }

    #[test]
    fn a_selector_given_to_a_runner_that_takes_none_is_refused() {
        let v = Verification { runner: "make verify".into(), select_with: None };
        assert!(
            matches!(
                command_for(&v, Some("anything")),
                Err(Unwitnessed::RunnerTakesNoSelector { .. })
            ),
            "a dropped selector runs the whole suite and reports it as the one test"
        );
    }
}
