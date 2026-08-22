//! `TS.260821.03`: report any doctrine this plugin ships that no record entity justifies.
//!
//! Every other module here reasons about the WORK — slices, commitments, claims, releases.
//! This one reasons about the INSTRUCTION shipped to do the work, which the record has never
//! held. That gap is why twenty-five of forty-seven skills could stop corresponding to
//! anything without a single edge going dangling: there were no edges, because there was no
//! node.
//!
//! The audit is total over its input and reads nothing. Which files ship is a fact handed in
//! by the shell (`fact-set@v1`) — a core that walks a directory is a core whose answer
//! depends on where it was run, and every rule in this engine would then be conditional on
//! a working tree.

use crate::admission::Corpus;

/// One file the plugin ships as instruction, and what in the record asks for it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Surface {
    pub id: String,
    pub path: String,
    pub kind: String,
    /// What this surface exists to serve — a slice, a capability, a read model. Never empty:
    /// the schema requires `1..n`, because a surface serving nothing is the whole rule.
    pub serves: Vec<String>,
    pub state: String,
    pub stands_at: Option<String>,
}

impl Surface {
    pub fn retired(&self) -> bool {
        self.state == "retired"
    }
}

/// What the audit found. Both directions, because they mean opposite things: a shipped file
/// nobody declared is doctrine on the plugin's authority, and a declared file that does not
/// ship is a promise the record cannot keep.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Audit {
    /// Shipped, and no surface declares it. REPORTED — see `every-shipped-surface-is-anchored`.
    pub unanchored: Vec<String>,
    /// Declared, and the plugin does not ship it. REFUSED — see `a-declared-surface-ships`.
    pub absent: Vec<(String, String)>,
    /// Declared retired, and the file is still there. The retirement did not happen.
    pub still_shipped: Vec<(String, String)>,
    /// Shipped and anchored to something the record holds.
    pub anchored: Vec<String>,
}

impl Audit {
    /// Nothing shipped without an anchor. The condition `TS.260821.04` exists to reach, and
    /// the one that licenses flipping the rule from report to refuse.
    pub fn clean(&self) -> bool {
        self.unanchored.is_empty() && self.absent.is_empty() && self.still_shipped.is_empty()
    }
}

/// Audit the shipped set against the declared set.
///
/// `shipped` is every instruction file the plugin ships, as the shell found them. The order
/// of the answer follows the order of the input so a report is stable between runs.
pub fn audit(surfaces: &[Surface], shipped: &[String]) -> Audit {
    let mut audit = Audit::default();

    for path in shipped {
        // A retired surface does not anchor: declaring a file retired and leaving it in the
        // tree is the failure mode the retirement was supposed to fix, so it is called out
        // as its own thing rather than quietly counting as anchored.
        match surfaces.iter().find(|s| &s.path == path) {
            Some(surface) if surface.retired() => {
                audit.still_shipped.push((surface.id.clone(), path.clone()));
            }
            Some(surface) => audit.anchored.push(surface.id.clone()),
            None => audit.unanchored.push(path.clone()),
        }
    }

    for surface in surfaces {
        if !surface.retired() && !shipped.iter().any(|p| p == &surface.path) {
            audit.absent.push((surface.id.clone(), surface.path.clone()));
        }
    }

    audit
}

/// Read every `doctrine-surface` the corpus holds.
///
/// The set comes from the record. An engine carrying its own list of what the plugin ships
/// would be the engine encoding what the record declares, which A4 forbids for exactly the
/// reason this module exists.
pub fn surfaces(corpus: &Corpus) -> &[Surface] {
    &corpus.surfaces
}
