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
    Conditions, Corpus, Known, Schema, Severity, Violation, assess, check_corpus, check_document,
    index_all, parse, project,
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
        Command::Ready { root } => match ready(&root) {
            Ok(()) => ExitCode::SUCCESS,
            Err(report) => {
                eprintln!("{report:?}");
                ExitCode::FAILURE
            }
        },
    }
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
        found.extend(
            corpus
                .iter()
                .filter(|v| std::ptr::eq(&docs[i], &docs[i]) && belongs(v, doc))
                .cloned(),
        );
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

/// Whether a corpus-level violation was raised by this document. Attribution is by
/// entity id, because the span is only meaningful in the file the node lives in.
fn belongs(violation: &Violation, doc: &kdl::KdlDocument) -> bool {
    doc.nodes().iter().any(|n| {
        n.name().value() == violation.entity_kind
            || n.entries()
                .iter()
                .find(|e| e.name().is_none())
                .and_then(|e| e.value().as_string())
                .is_some_and(|id| Some(id.to_owned()) == violation.entity_id)
    })
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
