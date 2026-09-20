//! Verifying a published tree. `TS.260820.11`: prove a published document was never
//! hand-edited after the fact.
//!
//! Each release's directory is compared against **the commit its own index node names** —
//! never against the current record, and never by re-rendering. Both alternatives were
//! tried on paper and discarded (`WALK.260820.01/W4`, `ES.260819.01/E20`): an archival
//! document is SUPPOSED to disagree with a record that has moved on, and re-rendering a
//! past release with today's logic reports drift that never occurred.
//!
//! `C2` is the load-bearing claim. A check that fails continuously is a check everyone
//! disables, and that failure mode is likelier than the drift it exists to catch.

/// One published file, as bytes. Compared as bytes because a comparison that normalises
/// is a comparison that can be talked out of.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Published {
    pub path: String,
    pub bytes: Vec<u8>,
}

/// How one document differs from what was published.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Drift {
    /// Present in both and not the same. The hand edit this slice exists to catch.
    Edited { path: String },
    /// Published then deleted.
    Removed { path: String },
    /// In the release's directory and not in the commit that release names.
    Added { path: String },
}

impl Drift {
    pub fn path(&self) -> &str {
        match self {
            Self::Edited { path } | Self::Removed { path } | Self::Added { path } => path,
        }
    }

    pub fn message(&self) -> String {
        match self {
            Self::Edited { path } => format!(
                "{path} differs from what was published. A published document is never edited, \
                 only superseded by the next release's own document"
            ),
            Self::Removed { path } => format!(
                "{path} was published and is gone. A release's documents are what that release \
                 said, and they do not stop being that"
            ),
            Self::Added { path } => format!(
                "{path} is in this release's directory and was not in the commit it names, so \
                 nothing says what release it depicts"
            ),
        }
    }
}

/// What verifying one release concluded.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verified {
    Clean { release: String, files: usize },
    Drifted { release: String, drift: Vec<Drift> },
    /// The comparison could not be made. **Never a pass**: a check that silently skips is
    /// how verification gets disabled without anyone deciding to disable it (`C4`).
    Unverifiable { release: String, why: String },
}

impl Verified {
    /// Whether this fails the check. Unverifiable fails, deliberately.
    pub fn failed(&self) -> bool {
        !matches!(self, Self::Clean { .. })
    }
}

/// Compare a release's published directory against the commit its index names.
///
/// `at_commit` is `None` when the commit could not be read — a shallow clone, an unfetched
/// history, a missing tag. That is a failure and not a skip.
pub fn verify(
    release: &str,
    at_commit: Option<&[Published]>,
    in_tree: &[Published],
) -> Verified {
    let Some(published) = at_commit else {
        return Verified::Unverifiable {
            release: release.to_owned(),
            why: "the commit this release indexes could not be read — a shallow clone or an \
                  unfetched history. Failing rather than skipping, because a check that skips \
                  quietly is one nobody decided to switch off"
                .to_owned(),
        };
    };
    if published.is_empty() {
        return Verified::Unverifiable {
            release: release.to_owned(),
            why: "the commit this release indexes holds no published directory for it, so there \
                  is nothing to compare against"
                .to_owned(),
        };
    }

    let mut drift = Vec::new();
    for was in published {
        match in_tree.iter().find(|now| now.path == was.path) {
            None => drift.push(Drift::Removed { path: was.path.clone() }),
            Some(now) if now.bytes != was.bytes => {
                drift.push(Drift::Edited { path: was.path.clone() })
            }
            Some(_) => {}
        }
    }
    for now in in_tree {
        if !published.iter().any(|was| was.path == now.path) {
            drift.push(Drift::Added { path: now.path.clone() });
        }
    }

    if drift.is_empty() {
        Verified::Clean { release: release.to_owned(), files: published.len() }
    } else {
        Verified::Drifted { release: release.to_owned(), drift }
    }
}
