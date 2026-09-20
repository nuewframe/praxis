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

/// Whether a filename matches a shape. One `*`, anywhere, or an exact name.
///
/// `TS.260823.05`. The shapes come from the record — `*SKILL.md`, `check-*.sh` — so this is
/// the only place that knows what a shape MEANS, and it is pure: the shell walks the tree
/// and asks.
#[must_use]
pub fn matches_shape(name: &str, pattern: &str) -> bool {
    match pattern.split_once('*') {
        Some((before, after)) => {
            name.len() >= before.len() + after.len()
                && name.starts_with(before)
                && name.ends_with(after)
        }
        None => name == pattern,
    }
}

/// The shapes the record says count as shipped instruction, as `(directory, pattern)`.
///
/// Empty is a repository that declares none, and the caller refuses rather than falling back
/// to a set the engine chose. That fallback was the fault: four shapes globbed in the shell,
/// reporting "55 of 55 anchored" while fifty-three files an adopter receives sat outside the
/// denominator entirely.
#[must_use]
pub fn declared_shapes(docs: &[kdl::KdlDocument]) -> Vec<(String, String)> {
    shapes_with(docs, None)
}

/// The shapes whose disposal is owed to a named slice, as `(directory, pattern, slice)`.
#[must_use]
pub fn owed_shapes(docs: &[kdl::KdlDocument]) -> Vec<(String, String, String)> {
    shapes_with(docs, Some("owed-to"))
        .into_iter()
        .filter_map(|(dir, named)| {
            owner_for(docs, &dir, &named).map(|owed| (dir, named, owed))
        })
        .collect()
}

fn owner_for(docs: &[kdl::KdlDocument], dir: &str, named: &str) -> Option<String> {
    for shape in ships_doctrine_nodes(docs) {
        let this_dir = shape.entries().first().and_then(|e| e.value().as_string());
        if this_dir == Some(dir) && crate::schema::prop(&shape, "named").as_deref() == Some(named) {
            return crate::schema::prop(&shape, "owed-to");
        }
    }
    None
}

fn ships_doctrine_nodes(docs: &[kdl::KdlDocument]) -> Vec<kdl::KdlNode> {
    let mut out = Vec::new();
    for doc in docs {
        for config in doc.nodes().iter().filter(|n| n.name().value() == "config") {
            let Some(body) = config.children() else { continue };
            let Some(block) = body.nodes().iter().find(|n| n.name().value() == "ships-doctrine")
            else {
                continue;
            };
            let Some(inner) = block.children() else { continue };
            out.extend(inner.nodes().iter().filter(|n| n.name().value() == "from").cloned());
        }
    }
    out
}

fn shapes_with(docs: &[kdl::KdlDocument], requiring: Option<&str>) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for shape in ships_doctrine_nodes(docs) {
        if let Some(key) = requiring
            && crate::schema::prop(&shape, key).is_none()
        {
            continue;
        }
        let Some(dir) = shape.entries().first().and_then(|e| e.value().as_string()) else {
            continue;
        };
        let Some(named) = crate::schema::prop(&shape, "named") else { continue };
        out.push((dir.to_owned(), named));
    }
    out
}

/// One line of shipped doctrine that instructs an agent in a word the method retired.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Teaching {
    pub path: String,
    /// 1-indexed, because the reader of a refusal opens the file at it.
    pub line: usize,
    pub word: String,
    pub instead: Option<String>,
}

/// Every line of `text` that names a retired word without saying it was retired.
///
/// `TS.260823.06`. The unit is the LINE, and that is the whole distinction the rule can
/// draw: a line that names a retired kind and does not say it was retired is an
/// instruction, and a line that names it beside a retirement marker is an explanation. A
/// surface must be able to explain what changed — a rule that refused every mention would
/// make the history unwritable, and an agent reading a method with a silent hole in its past
/// will fill the hole.
///
/// The markers come from the record, never from here. Which words mark an explanation is a
/// fact about a method's prose, and A4 forbids the engine holding one.
///
/// One report per line, not per occurrence: a line saying "create a sprint, then close the
/// sprint" is one thing to fix.
#[must_use]
pub fn teaching(path: &str, text: &str, words: &[&crate::schema::RetiredWord], markers: &[String]) -> Vec<Teaching> {
    let mut out = Vec::new();
    for (n, line) in text.lines().enumerate() {
        let lowered = line.to_lowercase();
        if markers.iter().any(|m| lowered.contains(m.as_str())) {
            continue;
        }
        // Tokenised once per line, not once per word. Doing it inside the `find` made the
        // scan quadratic in the retired vocabulary and re-allocated every token for every
        // word — `praxis check` went from under a second to over two minutes across a
        // hundred and seven files.
        let tokens = tokens(&lowered);
        if let Some(word) = words.iter().find(|w| tokens.iter().any(|t| w.matches(t))) {
            out.push(Teaching {
                path: path.to_owned(),
                line: n + 1,
                word: word.word.clone(),
                instead: word.instead.clone(),
            });
        }
    }
    out
}

/// The words of a line, hyphenated compounds counted both whole and in parts.
///
/// `wave-based` has to yield `wave` — it is the exact phrase five manifests carry — and
/// `TS-041` has to survive whole, because `TS-` is the head of an id form rather than a
/// word. Splitting on the hyphen loses the second; keeping it loses the first; so both.
fn tokens(lowered: &str) -> Vec<String> {
    let mut out = Vec::new();
    for raw in lowered.split(|c: char| !c.is_alphanumeric() && c != '-') {
        if raw.is_empty() {
            continue;
        }
        out.push(raw.to_owned());
        if raw.contains('-') {
            out.extend(raw.split('-').filter(|p| !p.is_empty()).map(str::to_owned));
        }
    }
    out
}
