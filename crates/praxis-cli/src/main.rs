//! `praxis` — the imperative shell.
//!
//! It reads the tree, hands text to the core, and renders whatever the core refused.
//! It decides nothing: every rule applied comes from the schema the record declares.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use miette::{Diagnostic, NamedSource, SourceSpan};
use praxis_core::{Known, Schema, Violation, check_document, index_all, parse};

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
}

/// A refusal, rendered. The span points at the offending node so the reader is told
/// where, not merely that.
#[derive(Debug, thiserror::Error, Diagnostic)]
#[error("{message}")]
#[diagnostic(severity(Error))]
struct Refused {
    message: String,
    #[source_code]
    src: NamedSource<String>,
    #[label("{label}")]
    span: SourceSpan,
    label: String,
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
    }
}

fn check(root: &Path) -> miette::Result<usize> {
    let files = kdl_files(root);
    if files.is_empty() {
        miette::bail!("no .kdl files under {}", root.display());
    }

    // Two reads: the first learns what the record declares, the second judges against
    // it. The schema cannot be applied while it is still being discovered.
    let mut sources = Vec::new();
    for path in files {
        let text = fs::read_to_string(&path)
            .map_err(|e| miette::miette!("{}: {e}", path.display()))?;
        let doc = parse(&text).map_err(|e| miette::Report::new(e).context(path.display().to_string()))?;
        sources.push((path, text, doc));
    }

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

    let mut refusals = 0;
    for (path, text, doc) in &sources {
        for violation in check_document(doc, &schema, &known) {
            refusals += 1;
            eprintln!("{:?}", report(path, text, &violation));
        }
    }

    if refusals == 0 {
        println!(
            "praxis: {} ok — {} entity kinds declared",
            root.display(),
            schema.kinds().count()
        );
    } else {
        eprintln!("praxis: {refusals} refusal(s) in {}", root.display());
    }
    Ok(refusals)
}

fn report(path: &Path, text: &str, violation: &Violation) -> miette::Report {
    let who = violation
        .entity_id
        .clone()
        .unwrap_or_else(|| violation.entity_kind.clone());
    miette::Report::new(Refused {
        message: format!("{who}: {}", violation.refusal.message()),
        src: NamedSource::new(path.display().to_string(), text.to_owned()),
        span: violation.span,
        label: format!("`{}`", violation.refusal.field()),
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
