//! `praxis` — the imperative shell.
//!
//! It reads the tree, hands text to the core, and renders whatever the core refused.
//! It decides nothing: every rule applied comes from the schema the record declares.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use miette::{Diagnostic, NamedSource, SourceSpan};
use praxis_core::{
    Ask, Closing, Conditions, Corpus, Known, Pickup, Schema, Severity, Violation, assess,
    check_corpus, check_document, close_iteration, index_all, parse, pick_up, project,
};

mod render;

#[derive(Parser)]
#[command(name = "praxis", version, about = "The delivery graph engine")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Refuse any entity that does not match the shape the record declares.
    Check {
        /// The state root to read.
        #[arg(default_value = "praxis")]
        root: PathBuf,
    },
    /// Take a slice. The gate admits and opens an iteration, or refuses and records why.
    ///
    /// Selection is the commitment: choosing this is choosing not to work something else.
    PickUp {
        /// The slice to take.
        slice: String,
        /// The state root to read and write.
        #[arg(default_value = "praxis")]
        root: PathBuf,
        /// Say what would happen and write nothing.
        #[arg(long)]
        dry_run: bool,
    },
    /// Close an iteration, or refuse and name every claim it cannot account for.
    Close {
        /// The iteration to close.
        iteration: String,
        /// What this iteration concluded.
        #[arg(long, default_value = "continue")]
        outcome: String,
        /// The state root to read and write.
        #[arg(default_value = "praxis")]
        root: PathBuf,
    },
    /// Show which slices could be started right now, and what would refuse each of the rest.
    ///
    /// `right now` is in the question, so this is computed on demand and never committed.
    Ready {
        /// The state root to read.
        #[arg(default_value = "praxis")]
        root: PathBuf,
    },
}

/// A refusal, rendered. The span points at the offending node so the reader is told
/// where, not merely that.
#[derive(Debug, thiserror::Error)]
#[error("{message}")]
struct Refused {
    message: String,
    src: NamedSource<String>,
    span: SourceSpan,
    label: String,
    severity: miette::Severity,
}

impl Diagnostic for Refused {
    fn severity(&self) -> Option<miette::Severity> {
        Some(self.severity)
    }
    fn source_code(&self) -> Option<&dyn miette::SourceCode> {
        Some(&self.src)
    }
    fn labels(&self) -> Option<Box<dyn Iterator<Item = miette::LabeledSpan> + '_>> {
        Some(Box::new(std::iter::once(miette::LabeledSpan::new_with_span(
            Some(self.label.clone()),
            self.span,
        ))))
    }
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match cli.command {
        Command::Check { root } => match check(&root) {
            Ok(0) => ExitCode::SUCCESS,
            Ok(_) => ExitCode::FAILURE,
            Err(report) => {
                eprintln!("{report:?}");
                ExitCode::FAILURE
            }
        },
        Command::PickUp { slice, root, dry_run } => match pickup(&slice, &root, dry_run) {
            Ok(admitted) => {
                if admitted {
                    ExitCode::SUCCESS
                } else {
                    ExitCode::FAILURE
                }
            }
            Err(report) => {
                eprintln!("{report:?}");
                ExitCode::FAILURE
            }
        },
        Command::Close { iteration, outcome, root } => match close(&iteration, &outcome, &root) {
            Ok(closed) => {
                if closed {
                    ExitCode::SUCCESS
                } else {
                    ExitCode::FAILURE
                }
            }
            Err(report) => {
                eprintln!("{report:?}");
                ExitCode::FAILURE
            }
        },
        Command::Ready { root } => match ready(&root) {
            Ok(()) => ExitCode::SUCCESS,
            Err(report) => {
                eprintln!("{report:?}");
                ExitCode::FAILURE
            }
        },
    }
}

/// `TS.260820.05` — the one gate. Either an iteration is open, or a refusal is on the
/// record naming the condition that failed. Never both, and never neither.
fn pickup(slice: &str, root: &Path, dry_run: bool) -> miette::Result<bool> {
    let sources = load(root)?;
    let docs: Vec<_> = sources.iter().map(|(_, _, d)| d.clone()).collect();

    let mut schema = Schema::default();
    let mut conditions = Conditions::default();
    for doc in &docs {
        let found = Schema::from_document(doc);
        if !found.is_empty() {
            schema = found;
        }
        let declared = Conditions::from_document(doc);
        if !declared.is_empty() {
            conditions = declared;
        }
    }
    if conditions.is_empty() {
        miette::bail!("the record declares no admission conditions — an empty gate is not an open one");
    }

    // The signer is whoever is asking, read from the tree they are asking in. An
    // agent-signed approval is the trust-transfer problem expressed as a signature, so
    // there is nowhere here to put an agent's name.
    let signer = human()?;
    let ask = Ask {
        signer,
        at: now(),
        by: "agent:praxis".to_owned(),
    };

    let corpus = Corpus::from_documents(&docs, &schema);
    let assessment = assess(&corpus, &conditions, &ask.at);
    let taken = ids_in_use(&docs);

    // Where the record goes is a layout question, so the shell answers it — and it
    // answers it from where the SLICE lives, not from the state root. A record belongs to
    // the frame that raised the work, and only the tree knows which frame that is.
    let home = frame_of(slice, &sources).ok_or_else(|| {
        miette::miette!("cannot tell which frame {slice} belongs to, so there is nowhere to put the record")
    })?;

    match pick_up(slice, &corpus, &assessment, &ask, &taken) {
        Pickup::NoSuchSlice(why) => miette::bail!("{why}"),
        Pickup::Opened(record) => {
            let path = home.join(&record.file);
            if dry_run {
                println!("would open {} at {}\n\n{}", record.id, path.display(), record.kdl);
            } else {
                write_once(&path, &record.kdl)?;
                println!("praxis: {} opened on {slice} — {}", record.id, path.display());
            }
            Ok(true)
        }
        Pickup::Refused(record) => {
            let path = home.join(&record.file);
            for (condition, detail) in &record.failed {
                eprintln!("praxis: refused — {condition}: {detail}");
            }
            if dry_run {
                eprintln!("would record {} at {}", record.id, path.display());
            } else {
                write_once(&path, &record.kdl)?;
                eprintln!("praxis: {} recorded at {} — nothing opened", record.id, path.display());
            }
            Ok(false)
        }
    }
}

/// `TS.260820.07`. Every claim settled, or every shortfall carried forward — or it does
/// not close. This is the first command that CHANGES a record rather than adding one, so
/// it rewrites the file whole: the document model preserves what it did not touch, and a
/// failed write leaves the original exactly as it was.
fn close(iteration: &str, outcome: &str, root: &Path) -> miette::Result<bool> {
    let sources = load(root)?;
    let docs: Vec<_> = sources.iter().map(|(_, _, d)| d.clone()).collect();

    let mut schema = Schema::default();
    for doc in &docs {
        let found = Schema::from_document(doc);
        if !found.is_empty() {
            schema = found;
        }
    }
    let corpus = Corpus::from_documents(&docs, &schema);
    let ask = Ask { signer: human()?, at: now(), by: "agent:praxis".to_owned() };
    let taken = ids_in_use(&docs);

    match close_iteration(iteration, &corpus, &ask, &taken) {
        Closing::NoSuchIteration(why) | Closing::NotOpen(why) => miette::bail!("{why}"),
        Closing::Refused(record) => {
            for (claim, why) in &record.failed {
                eprintln!("praxis: close refused — {claim}: {why}");
            }
            let home = frame_of_iteration(iteration, &sources).ok_or_else(|| {
                miette::miette!("cannot tell which frame {iteration} belongs to")
            })?;
            let path = home.join(&record.file);
            write_once(&path, &record.kdl)?;
            eprintln!(
                "praxis: {} recorded at {} — {iteration} stays open",
                record.id,
                path.display()
            );
            Ok(false)
        }
        Closing::Accepted { accounting, all_met, .. } => {
            let (path, text, _) = sources
                .iter()
                .find(|(_, _, doc)| holds_iteration(doc, iteration))
                .ok_or_else(|| miette::miette!("{iteration} is in no file"))?;
            let updated = apply_close(text, iteration, outcome, &accounting, all_met, &ask)?;
            fs::write(path, updated).map_err(|e| miette::miette!("{}: {e}", path.display()))?;
            for (claim, how) in &accounting {
                println!("praxis: {claim} — {how}");
            }
            println!(
                "praxis: {iteration} closed ({outcome}){}",
                if all_met { "" } else { " — with shortfalls carried" }
            );
            Ok(true)
        }
    }
}

fn holds_iteration(doc: &kdl::KdlDocument, id: &str) -> bool {
    doc.nodes()
        .iter()
        .any(|n| n.name().value() == "iteration" && root_id(n).as_deref() == Some(id))
}

fn frame_of_iteration(id: &str, sources: &[Source]) -> Option<PathBuf> {
    sources
        .iter()
        .find(|(_, _, doc)| holds_iteration(doc, id))
        .and_then(|(path, _, _)| path.parent()?.parent().map(Path::to_path_buf))
}

/// Rewrite the iteration in place: `state`, `closed-at`, `outcome`, an accounting block,
/// and a trail entry. Every value written here was COMPUTED — the machine states what it
/// checked and nothing about whether the work was any good.
///
/// New nodes are composed as TEXT and parsed, rather than built node by node. A node built
/// through the API carries no formatting, so it lands unindented with its values written
/// as bare identifiers — valid, and a record a human has to read. Parsing a fragment keeps
/// the formatting the fragment was written with (ITER.260821.06/V6).
fn apply_close(
    text: &str,
    iteration: &str,
    outcome: &str,
    accounting: &[(String, String)],
    all_met: bool,
    ask: &Ask,
) -> miette::Result<String> {
    let mut doc: kdl::KdlDocument = text
        .parse()
        .map_err(|e| miette::Report::new(e).context("reparsing to close"))?;
    let node = doc
        .nodes_mut()
        .iter_mut()
        .find(|n| n.name().value() == "iteration" && root_id(n).as_deref() == Some(iteration))
        .ok_or_else(|| miette::miette!("{iteration} vanished between reading and writing"))?;
    let body = node
        .children_mut()
        .as_mut()
        .ok_or_else(|| miette::miette!("{iteration} has no body"))?;

    // Swapped as a parsed fragment, not as a value: a value set through the API is
    // written as a bare identifier, so `state "open"` would become `state closed` —
    // valid KDL, and a diff that looks like the file changed shape.
    let closed_state: kdl::KdlDocument = "    state \"closed\"\n"
        .parse()
        .map_err(|e| miette::Report::new(e).context("composing the state"))?;
    if let Some(replacement) = closed_state.nodes().first() {
        for child in body.nodes_mut().iter_mut() {
            if child.name().value() == "state" {
                *child = replacement.clone();
            }
        }
    }

    let settled: String = accounting
        .iter()
        .map(|(claim, how)| format!("        claims-settled {claim:?} how={how:?}\n"))
        .collect();
    let note = concat!(
        "the accounting above was computed at close, not asserted. It says every claim is ",
        "either met or carried by a finding that names it, and nothing about whether the ",
        "work is any good"
    );
    let closing: kdl::KdlDocument = format!(
        "    closed-at {:?}\n    outcome {outcome:?}\n\n    close {{\n{settled}        all-met #{all_met}\n        note {note:?}\n    }}\n",
        ask.at
    )
    .parse()
    .map_err(|e| miette::Report::new(e).context("composing the close"))?;

    // After `opened-by`, and before the trail — where a reader looking for how it ended
    // would go first. Appending to the end is correct and unreadable.
    let at = body
        .nodes()
        .iter()
        .position(|n| n.name().value() == "trail")
        .unwrap_or(body.nodes().len());
    for (offset, new) in closing.nodes().iter().enumerate() {
        body.nodes_mut().insert(at + offset, new.clone());
    }

    let closing_note = concat!(
        "closed by `praxis close`. The accounting was computed; the conclusion is not the ",
        "machine's to draw"
    );
    let entry: kdl::KdlDocument = format!(
        "entry at={:?} by={:?} action=\"state-changed\" from=\"open\" to=\"closed\" note={closing_note:?}\n",
        ask.at, ask.by
    )
    .parse()
    .map_err(|e| miette::Report::new(e).context("composing the trail entry"))?;

    for child in body.nodes_mut().iter_mut() {
        if child.name().value() == "trail"
            && let Some(entries) = child.children_mut().as_mut()
            && let Some(first) = entry.nodes().first()
        {
            // A node's indentation is its own leading trivia, and a fragment parsed at
            // column zero has none. Inserting it without this puts it on the same line as
            // the `{` it went inside.
            let mut placed = first.clone();
            let mut format = placed.format().cloned().unwrap_or_default();
            format.leading = "\n        ".to_owned();
            placed.set_format(format);
            entries.nodes_mut().insert(0, placed);
        }
    }

    Ok(doc.to_string())
}

/// The frame directory a slice lives under. Discovery sits one level below the frame, so
/// the frame is the grandparent of the file the slice was found in.
fn frame_of(slice: &str, sources: &[Source]) -> Option<PathBuf> {
    let holder = sources.iter().find(|(_, _, doc)| {
        doc.nodes().iter().any(|n| {
            n.name().value() == "thin-slice"
                && n.entries()
                    .iter()
                    .find(|e| e.name().is_none())
                    .and_then(|e| e.value().as_string())
                    == Some(slice)
        })
    })?;
    holder.0.parent()?.parent().map(Path::to_path_buf)
}

/// Every id the record already uses, so a new one cannot collide.
fn ids_in_use(docs: &[kdl::KdlDocument]) -> Vec<String> {
    let mut out = Vec::new();
    for doc in docs {
        for node in doc.nodes() {
            if let Some(id) = root_id(node) {
                out.push(id);
            }
        }
    }
    out
}

/// Whoever is asking. `human:<name>`, from the tree's own git identity — the record
/// refuses a signature outside the human namespace, and there is no way to spell an
/// agent's name here.
fn human() -> miette::Result<String> {
    let out = std::process::Command::new("git")
        .args(["config", "user.email"])
        .output()
        .map_err(|e| miette::miette!("cannot read the git identity: {e}"))?;
    let email = String::from_utf8_lossy(&out.stdout).trim().to_owned();
    let name = email.split('@').next().unwrap_or_default().to_owned();
    if name.is_empty() {
        miette::bail!(
            "no git identity, so there is nobody to sign the admission. An unsigned admission \
             is refused by the record, and an agent may not sign one"
        );
    }
    Ok(format!("human:{name}"))
}

/// `change-set@v1`: whole or nothing, leaving the record unchanged on failure. The file
/// is written complete under a temporary name and moved into place, so a reader never
/// sees half a record and a crash leaves nothing behind.
fn write_once(path: &Path, content: &str) -> miette::Result<()> {
    if path.exists() {
        miette::bail!("{} already exists — the gate never overwrites a record", path.display());
    }
    let dir = path.parent().unwrap_or(Path::new("."));
    fs::create_dir_all(dir).map_err(|e| miette::miette!("{}: {e}", dir.display()))?;
    let staging = path.with_extension("kdl.writing");
    fs::write(&staging, content).map_err(|e| miette::miette!("{}: {e}", staging.display()))?;
    fs::rename(&staging, path).map_err(|e| {
        let _ = fs::remove_file(&staging);
        miette::miette!("{}: {e}", path.display())
    })
}

/// `TS.260820.04`. The conditions come from the record, the verdicts from the core, and
/// the composition from a renderer that does not know what any of it means.
fn ready(root: &Path) -> miette::Result<()> {
    let sources = load(root)?;
    let docs: Vec<_> = sources.iter().map(|(_, _, d)| d.clone()).collect();

    let mut schema = Schema::default();
    let mut conditions = Conditions::default();
    for doc in &docs {
        let found = Schema::from_document(doc);
        if !found.is_empty() {
            schema = found;
        }
        let declared = Conditions::from_document(doc);
        if !declared.is_empty() {
            conditions = declared;
        }
    }
    if conditions.is_empty() {
        miette::bail!(
            "the record declares no admission conditions — there is nothing to evaluate, \
             and an empty gate must not be reported as an open one"
        );
    }

    let corpus = Corpus::from_documents(&docs, &schema);
    let assessment = assess(&corpus, &conditions, &now());
    let model = project(&assessment, &corpus);

    // A result that breaks its own seam is a refusal like any other.
    let flaws = model.flaws();
    if !flaws.is_empty() {
        for flaw in &flaws {
            eprintln!("praxis: read-model@v1 violated — {flaw}");
        }
        miette::bail!("the result does not satisfy read-model@v1");
    }

    print!("{}", render::render(&model));
    Ok(())
}

/// The moment, in UTC. A view whose value is being current must say when it was computed.
fn now() -> String {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_secs());
    let (days, rem) = ((secs / 86_400) as i64, secs % 86_400);
    let (y, m, d) = civil_from_days(days);
    format!(
        "{y:04}-{m:02}-{d:02}T{:02}:{:02}:{:02}Z",
        rem / 3600,
        (rem % 3600) / 60,
        rem % 60
    )
}

/// Howard Hinnant's `civil_from_days`, era-based. Days since 1970-01-01 to a civil date.
fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (if m <= 2 { y + 1 } else { y }, m, d)
}

/// Read every `.kdl` file under a root, parsed. The only place the engine touches a disk.
type Source = (PathBuf, String, kdl::KdlDocument);

fn load(root: &Path) -> miette::Result<Vec<Source>> {
    let files = kdl_files(root);
    if files.is_empty() {
        miette::bail!("no .kdl files under {}", root.display());
    }
    let mut sources = Vec::new();
    for path in files {
        let text =
            fs::read_to_string(&path).map_err(|e| miette::miette!("{}: {e}", path.display()))?;
        let doc = parse(&text)
            .map_err(|e| miette::Report::new(e).context(path.display().to_string()))?;
        sources.push((path, text, doc));
    }
    Ok(sources)
}

fn check(root: &Path) -> miette::Result<usize> {
    // Two reads: the first learns what the record declares, the second judges against
    // it. The schema cannot be applied while it is still being discovered.
    let sources = load(root)?;

    let mut schema = Schema::default();
    let mut known = Known::default();
    for (_, _, doc) in &sources {
        let found = Schema::from_document(doc);
        if !found.is_empty() {
            schema = found;
        }
        index_all(doc, &mut known);
    }
    if schema.is_empty() {
        miette::bail!("the record declares no schema — nothing to check against");
    }

    // Rules that need the whole record at once — uniqueness and coverage — are decided
    // over the corpus and attributed back to the file that raised them.
    let docs: Vec<_> = sources.iter().map(|(_, _, d)| d.clone()).collect();
    let corpus = check_corpus(&docs, &schema);

    let (mut refusals, mut reports) = (0, 0);
    for (i, (path, text, doc)) in sources.iter().enumerate() {
        let mut found = check_document(doc, &schema, &known);
        let _ = i;
        found.extend(corpus.iter().filter(|v| belongs(v, doc)).cloned());
        for violation in found {
            match violation.severity() {
                Severity::Refuse => refusals += 1,
                Severity::Report => reports += 1,
            }
            eprintln!("{:?}", report(path, text, &violation));
        }
    }

    let noted = if reports > 0 { format!(" · {reports} report(s)") } else { String::new() };
    if refusals == 0 {
        println!(
            "praxis: {} ok — {} entity kinds declared{noted}",
            root.display(),
            schema.kinds().count()
        );
    } else {
        eprintln!("praxis: {refusals} refusal(s) in {}{noted}", root.display());
    }
    Ok(refusals)
}

/// Whether a corpus-level violation was raised by this document. Attribution is by entity
/// ID, because the span is only meaningful in the file the node lives in — and matching on
/// KIND as well would report one violation once per file holding any record of that kind
/// (ITER.260821.03/S7).
fn belongs(violation: &Violation, doc: &kdl::KdlDocument) -> bool {
    match &violation.entity_id {
        Some(id) => doc.nodes().iter().any(|n| root_id(n).as_deref() == Some(id.as_str())),
        // An anonymous violation can only be placed by kind, which is imprecise — so a
        // corpus rule that cannot name the entity it is about is a rule to reconsider.
        None => doc.nodes().iter().any(|n| n.name().value() == violation.entity_kind),
    }
}

fn root_id(node: &kdl::KdlNode) -> Option<String> {
    node.entries()
        .iter()
        .find(|e| e.name().is_none())
        .and_then(|e| e.value().as_string())
        .map(str::to_owned)
}

fn report(path: &Path, text: &str, violation: &Violation) -> miette::Report {
    let severity = match violation.severity() {
        Severity::Refuse => miette::Severity::Error,
        Severity::Report => miette::Severity::Warning,
    };
    let who = violation
        .entity_id
        .clone()
        .unwrap_or_else(|| violation.entity_kind.clone());
    miette::Report::new(Refused {
        message: format!("{who}: {}", violation.refusal.message()),
        src: NamedSource::new(path.display().to_string(), text.to_owned()),
        span: violation.span,
        label: format!("`{}`", violation.refusal.field()),
        severity,
    })
}

fn kdl_files(root: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    walk(root, &mut out);
    out.sort();
    out
}

fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            walk(&path, out);
        } else if path.extension().is_some_and(|e| e == "kdl") {
            out.push(path);
        }
    }
}
