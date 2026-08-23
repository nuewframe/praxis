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
    Ask, Binding, Closing, Conditions, Corpus, Known, Pickup, Schema, Severity, Violation,
    Cut, Promotion, Publication, Published, Verified, assess, bind,
    check_document, close_iteration, cut, index_all, parse, pick_up, project, promote, publish,
    dashboard, guide_for, prove, review, unbind, verify, what_is_currently_true,
};
use praxis_core::check::{Facts, check_corpus_given, decision_body};
use praxis_core::schema::Extension;
use praxis_core::invariant::{Enforcement, check_invariants};
use praxis_core::surface::{audit, surfaces};
use praxis_core::ask::{Answer, Question, ask};
use praxis_core::view::{ReadModel, Section};
use praxis_core::withdraw::{Withdrawal, withdraw};
use praxis_core::cut::seal;

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
    /// Settle a claim on a run this tool watched, capturing what it exited with.
    ///
    /// The command is NOT yours to choose. The repository declares its verification once,
    /// in the config, and `--by` selects within it — an engine that ran whatever the caller
    /// named would be a command runner driven by the agent it exists to check.
    Evidence {
        /// The iteration holding the claim.
        iteration: String,
        /// The claim to settle.
        #[arg(long)]
        claim: String,
        /// Which part of the declared verification to run. Omit to run all of it.
        #[arg(long)]
        by: Option<String>,
        /// Which slice's claim, when one commitment covers several and they share an id.
        ///
        /// A claim id is unique within a SLICE, not within an iteration — an iteration over
        /// three slices carries three `C1`s. Settling "whichever comes first" is the exact
        /// failure `a-claim-id-is-unique-in-its-iteration` was written about.
        #[arg(long)]
        slice: Option<String>,
        /// The state root to read and write.
        #[arg(long, default_value = "praxis")]
        root: PathBuf,
    },
    /// Take a slice. The gate admits and opens an iteration, or refuses and records why.
    ///
    /// Selection is the commitment: choosing this is choosing not to work something else.
    PickUp {
        /// The slices to take, as ONE commitment. Vetted separately, admitted together.
        #[arg(required = true, num_args = 1..)]
        slices: Vec<String>,
        /// The state root to read and write.
        ///
        /// A flag rather than a positional: a variadic slice list followed by an optional
        /// positional is ambiguous, and clap refuses to build it (ITER.260821.19/AK1).
        #[arg(long, default_value = "praxis")]
        root: PathBuf,
        /// Who asked for this work to start. Required, and deliberately without a default.
        ///
        /// The ask IS the permission — that was never in doubt. What is refused is the TOOL
        /// deciding whose ask it was: a git identity names whose machine this is and knows
        /// nothing about who decided (`TS.260823.01`). An identity the tool supplied would
        /// prove the tool ran, which nobody doubted — the same argument `--attested-by`
        /// makes at the other end of the iteration.
        #[arg(long)]
        asked_by: String,
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
        /// Who attests this close. Required, and deliberately without a default.
        ///
        /// A close is something somebody does. An identity the tool supplied would prove the
        /// tool ran, which nobody doubted — see TS.260821.09 and ITER.260822.05/AP3.
        #[arg(long)]
        attested_by: Option<String>,
        /// The state root to read and write.
        #[arg(default_value = "praxis")]
        root: PathBuf,
    },
    /// Bind a closed iteration to a version, or refuse and say what stopped it.
    Bind {
        /// The iteration to bind.
        iteration: String,
        /// The version to bind it to.
        version: String,
        /// The state root to read and write.
        #[arg(default_value = "praxis")]
        root: PathBuf,
        /// Remove the binding instead. Allowed while the release is planned, refused after cut.
        #[arg(long)]
        undo: bool,
    },
    /// Cut a planned version, writing its index node in the same operation.
    CutRelease {
        /// The version to cut.
        version: String,
        /// The state root to read and write.
        #[arg(default_value = "praxis")]
        root: PathBuf,
        /// Confirm the proposed bump. The record proposes; choosing the version is yours.
        #[arg(long)]
        confirm: bool,
    },
    /// Regenerate the published set for a version, whole, before it is cut.
    Publish {
        /// The version to publish for.
        version: String,
        /// The state root to read.
        #[arg(default_value = "praxis")]
        root: PathBuf,
        /// Say what would be written and write nothing.
        #[arg(long)]
        dry_run: bool,
    },
    /// Prove every published document is byte-identical to what was published.
    ///
    /// Compares each release's directory against the commit its own index node names —
    /// never against the current record, which an archival document is meant to outlive.
    VerifyPublished {
        /// The state root to read.
        #[arg(default_value = "praxis")]
        root: PathBuf,
    },
    /// Fold what a cut release shipped into what each capability says it is.
    Promote {
        /// The cut version to promote.
        version: String,
        /// The state root to read and write.
        #[arg(default_value = "praxis")]
        root: PathBuf,
        /// Say what would move and move nothing.
        #[arg(long)]
        dry_run: bool,
    },
    /// Ask the record what this repository already knows, instead of re-deriving it.
    Truth {
        /// The state root to read.
        #[arg(default_value = "praxis")]
        root: PathBuf,
    },
    /// Preview what an iteration has promised and shown. Writes nothing.
    Review {
        /// The iteration to preview.
        iteration: String,
        /// The state root to read.
        #[arg(default_value = "praxis")]
        root: PathBuf,
        /// Render as Markdown into the working projection path, which is gitignored.
        #[arg(long)]
        markdown: bool,
    },
    /// Show how to use a capability at a version, or refuse if that version never shipped it.
    Guide {
        /// The capability.
        capability: String,
        /// The version whose surface to describe.
        version: String,
        /// The state root to read.
        #[arg(default_value = "praxis")]
        root: PathBuf,
    },
    /// Compose one declared read model from the record, now.
    ///
    /// The answer to "it lives in the record" for a view a release does not publish. The
    /// decisions, the architecture and the doctrine inventory stopped being published at
    /// `TS.260821.14` — they are engineering, and a reader of a release did not come for
    /// them. This is where whoever DID come for them asks, and gets the answer for the
    /// tree in front of them rather than for one frozen version.
    View {
        /// The read model to compose, by the name the record declares for it.
        name: String,
        /// Compose it as it would look for this version, rather than for now.
        #[arg(long)]
        at: Option<String>,
        /// The state root to read.
        #[arg(long, default_value = "praxis")]
        root: PathBuf,
    },
    /// Where the product stands right now. Composed on demand, never committed.
    Dashboard {
        /// The state root to read.
        #[arg(default_value = "praxis")]
        root: PathBuf,
    },
    /// Report any declared rule that nothing demonstrates refusing.
    ///
    /// A rule that has never been shown to refuse is indistinguishable from one that cannot.
    Prove {
        /// The state root to read.
        #[arg(default_value = "praxis")]
        root: PathBuf,
    },
    /// Report any doctrine this plugin ships that no record entity justifies.
    ///
    /// A skill nobody can trace to a capability, a slice or a read model is instruction an
    /// agent follows on the plugin's authority alone.
    AuditSurfaces {
        /// The state root to read.
        #[arg(long, default_value = "praxis")]
        root: PathBuf,
        /// The tree whose shipped doctrine is audited.
        #[arg(long, default_value = ".")]
        from: PathBuf,
    },
    /// Report any guarantee this plugin makes that nothing keeps.
    ///
    /// `enable-all-fail-closed` is a claim about shell scripts until something compares it
    /// to what the record says enforces each invariant.
    CheckInvariants {
        /// The state root to read.
        #[arg(long, default_value = "praxis")]
        root: PathBuf,
    },
    /// Take a cut release back, unwinding what the cut wrote elsewhere.
    ///
    /// A cut is a pointer into history. Moving it back should cost what moving it forward
    /// cost — including the promoted truth it folded into capability records.
    Withdraw {
        /// The version to withdraw.
        version: String,
        /// Why. Required: a withdrawal nobody explained is a cut nobody can account for.
        #[arg(long)]
        because: Option<String>,
        /// The state root to read and write.
        #[arg(long, default_value = "praxis")]
        root: PathBuf,
    },
    /// Answer a question about the record — any declared kind, filtered and projected.
    ///
    /// Generic by necessity: a command that knew what an `iteration` was would be the engine
    /// encoding what the record declares, and would answer nothing in a repository whose
    /// kinds are `cohort` and `experiment`.
    Ask {
        /// The kind to ask about.
        kind: String,
        /// Keep only records where this field has this value. Repeatable; all must hold.
        #[arg(long = "where", value_name = "FIELD=VALUE")]
        wheres: Vec<String>,
        /// Fields to show beside the identity. Comma-separated.
        #[arg(long, value_name = "FIELDS")]
        show: Option<String>,
        /// Print the count alone, for a loop rather than a reader.
        #[arg(long)]
        count: bool,
        /// Keep only records nothing points at, through a declared edge: `kind.field`.
        #[arg(long, value_name = "KIND.FIELD")]
        unreferenced_by: Option<String>,
        /// The state root to read.
        #[arg(long, default_value = "praxis")]
        root: PathBuf,
    },
    /// Print the method this engine carries — its vocabulary, rules, and gate.
    ///
    /// The answer to "what governs this repository", without reading anyone else's
    /// discovery folder.
    Schema {
        /// Write the method's record to stdout, whole.
        #[arg(long)]
        print: bool,
    },
    /// Seal every accepted decision and resolved symptom that carries no seal.
    ///
    /// Acceptance is an ACT. A state nobody performs is a state nobody seals.
    Accept {
        /// The state root to read and write.
        #[arg(default_value = "praxis")]
        root: PathBuf,
        /// Say what would be sealed and write nothing.
        #[arg(long)]
        dry_run: bool,
    },
    /// Show which slices could be started right now, and what would refuse each of the rest.
    ///
    /// `right now` is in the question, so this is computed on demand and never committed.
    Ready {
        /// The state root to read.
        #[arg(default_value = "praxis")]
        root: PathBuf,
    },
    /// Put a repository under the method: a binding, a verify entry point, and the
    /// engineering doctrine that survived the spine's retirement.
    ///
    /// Writes a CONFIG and nothing else about your product. The frame, the storm, the
    /// capabilities and the slices are yours — deriving them from what the repository
    /// already claims would import those claims without their evidence, which is the
    /// longest argument `adopt-the-method` makes.
    Adopt {
        /// The repository this record is about, as `owner/name`. Required, and deliberately
        /// without a default: a git remote names whose machine this is and knows nothing
        /// about what the record is about.
        #[arg(long)]
        repository: String,
        /// Where to write. Defaults to here.
        #[arg(long, default_value = ".")]
        into: PathBuf,
        /// Where this repository's capabilities live.
        #[arg(long, default_value = "src/")]
        source_root: String,
        /// The primary language, for the overlay's one project-specific paragraph.
        #[arg(long, default_value = "unstated")]
        language: String,
        /// What `praxis evidence` runs to settle a claim. Left as a shell no-op when unset,
        /// never guessed: a generated step that runs the wrong tool fails for a reason the
        /// adopter did not cause, and the first thing they do is delete it.
        #[arg(long)]
        test: Option<String>,
        #[arg(long)]
        lint: Option<String>,
        #[arg(long)]
        format: Option<String>,
        #[arg(long)]
        typecheck: Option<String>,
        /// Say what would be written and write nothing.
        #[arg(long)]
        dry_run: bool,
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
        Command::Evidence { iteration, claim, by, slice, root } => {
            match evidencing(&iteration, &claim, by.as_deref(), slice.as_deref(), &root) {
                Ok(true) => ExitCode::SUCCESS,
                Ok(false) => ExitCode::FAILURE,
                Err(report) => {
                    eprintln!("{report:?}");
                    ExitCode::FAILURE
                }
            }
        }
        Command::PickUp { slices, root, asked_by, dry_run } => match pickup(&slices, &root, &asked_by, dry_run) {
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
        Command::Close { iteration, outcome, attested_by, root } =>
            match close(&iteration, &outcome, attested_by.as_deref(), &root) {
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
        Command::Bind { iteration, version, root, undo } => {
            match binding(&iteration, &version, &root, undo) {
                Ok(bound) => {
                    if bound {
                        ExitCode::SUCCESS
                    } else {
                        ExitCode::FAILURE
                    }
                }
                Err(report) => {
                    eprintln!("{report:?}");
                    ExitCode::FAILURE
                }
            }
        }
        Command::CutRelease { version, root, confirm } => {
            match cutting(&version, &root, confirm) {
                Ok(made) => {
                    if made {
                        ExitCode::SUCCESS
                    } else {
                        ExitCode::FAILURE
                    }
                }
                Err(report) => {
                    eprintln!("{report:?}");
                    ExitCode::FAILURE
                }
            }
        }
        Command::Publish { version, root, dry_run } => {
            match publishing(&version, &root, dry_run) {
                Ok(written) => {
                    if written {
                        ExitCode::SUCCESS
                    } else {
                        ExitCode::FAILURE
                    }
                }
                Err(report) => {
                    eprintln!("{report:?}");
                    ExitCode::FAILURE
                }
            }
        }
        Command::VerifyPublished { root } => match verifying(&root) {
            Ok(0) => ExitCode::SUCCESS,
            Ok(_) => ExitCode::FAILURE,
            Err(report) => {
                eprintln!("{report:?}");
                ExitCode::FAILURE
            }
        },
        Command::Promote { version, root, dry_run } => {
            match promoting(&version, &root, dry_run) {
                Ok(moved) => {
                    if moved {
                        ExitCode::SUCCESS
                    } else {
                        ExitCode::FAILURE
                    }
                }
                Err(report) => {
                    eprintln!("{report:?}");
                    ExitCode::FAILURE
                }
            }
        }
        Command::Truth { root } => match truth(&root) {
            Ok(()) => ExitCode::SUCCESS,
            Err(report) => {
                eprintln!("{report:?}");
                ExitCode::FAILURE
            }
        },
        Command::Review { iteration, root, markdown } => {
            match reviewing(&iteration, &root, markdown) {
                Ok(()) => ExitCode::SUCCESS,
                Err(report) => {
                    eprintln!("{report:?}");
                    ExitCode::FAILURE
                }
            }
        }
        Command::Guide { capability, version, root } => {
            match guiding(&capability, &version, &root) {
                Ok(()) => ExitCode::SUCCESS,
                Err(report) => {
                    eprintln!("{report:?}");
                    ExitCode::FAILURE
                }
            }
        }
        Command::View { name, at, root } => match viewing(&name, at.as_deref(), &root) {
            Ok(true) => ExitCode::SUCCESS,
            Ok(false) => ExitCode::FAILURE,
            Err(report) => {
                eprintln!("{report:?}");
                ExitCode::FAILURE
            }
        },
        Command::Dashboard { root } => match dashboarding(&root) {
            Ok(()) => ExitCode::SUCCESS,
            Err(report) => {
                eprintln!("{report:?}");
                ExitCode::FAILURE
            }
        },
        Command::Prove { root } => match proving(&root) {
            Ok(0) => ExitCode::SUCCESS,
            Ok(_) => ExitCode::SUCCESS,
            Err(report) => {
                eprintln!("{report:?}");
                ExitCode::FAILURE
            }
        },
        Command::AuditSurfaces { root, from } => match auditing(&root, &from) {
            Ok(clean) => {
                if clean {
                    ExitCode::SUCCESS
                } else {
                    ExitCode::SUCCESS
                }
            }
            Err(report) => {
                eprintln!("{report:?}");
                ExitCode::FAILURE
            }
        },
        Command::CheckInvariants { root } => match invariants(&root) {
            Ok(()) => ExitCode::SUCCESS,
            Err(report) => {
                eprintln!("{report:?}");
                ExitCode::FAILURE
            }
        },
        Command::Withdraw { version, because, root } => {
            match withdrawing(&version, because.as_deref(), &root) {
                Ok(true) => ExitCode::SUCCESS,
                Ok(false) => ExitCode::FAILURE,
                Err(report) => {
                    eprintln!("{report:?}");
                    ExitCode::FAILURE
                }
            }
        }
        Command::Ask { kind, wheres, show, count, unreferenced_by, root } => {
            match asking(&kind, &wheres, show.as_deref(), count, unreferenced_by.as_deref(), &root)
            {
                Ok(true) => ExitCode::SUCCESS,
                Ok(false) => ExitCode::FAILURE,
                Err(report) => {
                    eprintln!("{report:?}");
                    ExitCode::FAILURE
                }
            }
        }
        Command::Schema { print } => {
            let (schema, version) = Schema::method();
            if print {
                print!("{}", praxis_core::schema::METHOD);
            } else {
                println!("praxis: governed by {version}");
                println!(
                    "praxis: {} entity kinds · {} rules · carried by the engine and shipped at \
                     praxis/method/",
                    schema.kinds().count(),
                    schema.rules().count()
                );
                println!(
                    "praxis: a repository EXTENDS this by declaring kinds it does not name, and \
                     may never redefine one. `praxis schema --print` writes it whole"
                );
            }
            ExitCode::SUCCESS
        }
        Command::Accept { root, dry_run } => match accepting(&root, dry_run) {
            Ok(()) => ExitCode::SUCCESS,
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
        Command::Adopt {
            repository,
            into,
            source_root,
            language,
            test,
            lint,
            format,
            typecheck,
            dry_run,
        } => {
            let (_, method) = Schema::method();
            let mut binding = praxis_core::adopt::Binding::new(&repository, &method);
            binding.source_root = source_root;
            binding.language = language;
            if let Some(test) = test {
                binding.runner.clone_from(&test);
                binding.test = test;
            }
            if let Some(lint) = lint {
                binding.lint = lint;
            }
            if let Some(format) = format {
                binding.format = format;
            }
            if let Some(typecheck) = typecheck {
                binding.typecheck = typecheck;
            }
            match adopting(&binding, &into, dry_run) {
                Ok(()) => ExitCode::SUCCESS,
                Err(report) => {
                    eprintln!("{report:?}");
                    ExitCode::FAILURE
                }
            }
        }
    }
}

/// `TS.260823.08`. Write the binding, the gate, and the doctrine — and nothing else.
///
/// Every refusal is recorded as an outcome and named. A front door that half-wrote and
/// stopped would leave a repository that reads as adopted and is not, which is the class of
/// fault this slice exists to close.
fn adopting(
    binding: &praxis_core::adopt::Binding,
    into: &Path,
    dry_run: bool,
) -> miette::Result<()> {
    // A dry run never refuses on an existing binding. It writes nothing, and the reason to
    // run one in an already-adopted repository is exactly to see what the current templates
    // would produce — which is how an adopter diffs a plugin upgrade against their tree.
    let bound = !dry_run && into.join("praxis/config.kdl").exists();
    let written = praxis_core::adopt::adopt(binding, bound)
        .map_err(|refused| miette::miette!("{}", refused.message()))?;

    for file in &written {
        let left = praxis_core::adopt::unresolved(&file.content);
        if !left.is_empty() {
            miette::bail!(
                "{} would ship with {} unresolved: a file that reads as configured and is not \
                 is worse than one that is obviously blank",
                file.path,
                left.join(", ")
            );
        }
    }

    if dry_run {
        for file in &written {
            println!("praxis: would write {} ({} bytes)", file.path, file.content.len());
        }
        println!(
            "praxis: and nothing else. No frame, no storm, no capability — `praxis view \
             what-you-must-declare` names the seven kinds you write next"
        );
        return Ok(());
    }

    for file in &written {
        let target = into.join(&file.path);
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| miette::miette!("cannot create {}: {e}", parent.display()))?;
        }
        fs::write(&target, &file.content)
            .map_err(|e| miette::miette!("cannot write {}: {e}", target.display()))?;
        if file.executable {
            executable(&target)?;
        }
        println!("praxis: wrote {}", file.path);
    }

    // The state root, empty. A directory with nothing in it is the honest starting state:
    // `praxis check` passes over it, and every kind that belongs there is one the adopter
    // has not needed yet.
    fs::create_dir_all(into.join("praxis/frames"))
        .map_err(|e| miette::miette!("cannot create the state root: {e}"))?;

    println!(
        "praxis: {} is bound to {}. Nothing else was written — the frame, the storm and the \
         capabilities are yours, and deriving them from what this repository already claims \
         would import those claims without their evidence",
        binding.repository, binding.method
    );
    println!("praxis: next — `praxis check`, then `praxis view what-you-must-declare`");
    Ok(())
}

/// Mark a generated script executable, where the platform has such a thing.
fn executable(path: &Path) -> miette::Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(path)
            .map_err(|e| miette::miette!("cannot read {}: {e}", path.display()))?
            .permissions();
        perms.set_mode(0o755);
        fs::set_permissions(path, perms)
            .map_err(|e| miette::miette!("cannot chmod {}: {e}", path.display()))?;
    }
    #[cfg(not(unix))]
    let _ = path;
    Ok(())
}

/// `TS.260823.02` — settle a claim on a run, and hold what came back.
///
/// The shell runs it, because running is I/O; the core decides what a run means and what it
/// contributes. What is run comes from the config and never from the caller.
fn evidencing(
    iteration: &str,
    claim: &str,
    by: Option<&str>,
    slice: Option<&str>,
    root: &Path,
) -> miette::Result<bool> {
    use praxis_core::evidence::{Unwitnessed, Witnessed, command_for, digest, verification};

    let sources = load(root)?;
    let docs: Vec<_> = sources.iter().map(|(_, _, d)| d.clone()).collect();

    let Some(declared) = verification(&docs) else {
        miette::bail!(
            "this repository declares no `verification` in its config, so there is nothing to \
             run and a claim here settles on prose. Declare one — `verification {{ runner \
             \"...\" }}` — or settle the claim by hand and let the record show it was REPORTED \
             rather than witnessed"
        );
    };

    let argv = match command_for(&declared, by) {
        Ok(argv) => argv,
        Err(Unwitnessed::RunnerTakesNoSelector { runner }) => miette::bail!(
            "`{runner}` declares no `select-with`, so it cannot be pointed at one part of \
             itself. Running the whole suite and recording it as the selected one would settle \
             the claim on a run nobody asked for"
        ),
        Err(other) => miette::bail!("cannot build the verification command: {other:?}"),
    };

    let (program, rest) = argv.split_first().expect("a runner is never empty");
    let printed = argv.join(" ");
    eprintln!("praxis: running {printed}");

    let output = std::process::Command::new(program)
        .args(rest)
        .output()
        .map_err(|e| miette::miette!("cannot run `{printed}`: {e}"))?;

    let mut captured = String::from_utf8_lossy(&output.stdout).into_owned();
    captured.push_str(&String::from_utf8_lossy(&output.stderr));
    let exit = output.status.code().unwrap_or(-1);

    let run = Witnessed { command: printed, exit, digest: digest(&captured), at: now() };

    if !run.settles() {
        eprintln!(
            "praxis: {} exited {} — the claim stays pending. A red run is a fact and it is not \
             evidence the claim holds",
            run.command, run.exit
        );
        return Ok(false);
    }

    let (path, updated) = settle_claim(iteration, claim, slice, &run, &sources)?;
    fs::write(&path, updated).map_err(|e| miette::miette!("{}: {e}", path.display()))?;
    eprintln!(
        "praxis: {claim} — met, witnessed by a run exiting {} ({})",
        run.exit, run.digest
    );
    Ok(true)
}

/// Rewrite the iteration whole, with the run folded into the named claim.
///
/// The document model preserves everything it did not touch, so a hand-written note beside
/// the claim survives — the record is edited, never regenerated.
fn settle_claim(
    iteration: &str,
    claim: &str,
    slice: Option<&str>,
    run: &praxis_core::evidence::Witnessed,
    sources: &[(PathBuf, String, kdl::KdlDocument)],
) -> miette::Result<(PathBuf, String)> {
    let (path, text, _) = sources
        .iter()
        .find(|(_, _, d)| {
            d.nodes().iter().any(|n| {
                n.name().value() == "iteration" && root_id(n).as_deref() == Some(iteration)
            })
        })
        .ok_or_else(|| miette::miette!("no iteration {iteration} in the record"))?;

    let mut doc: kdl::KdlDocument = text
        .parse()
        .map_err(|e| miette::Report::new(e).context("reparsing to settle"))?;
    let node = doc
        .nodes_mut()
        .iter_mut()
        .find(|n| n.name().value() == "iteration" && root_id(n).as_deref() == Some(iteration))
        .ok_or_else(|| miette::miette!("{iteration} vanished between reading and writing"))?;
    let body = node
        .children_mut()
        .as_mut()
        .ok_or_else(|| miette::miette!("{iteration} has no body"))?;

    // A claim id is unique within a SLICE. An iteration over several carries the same id
    // once per slice, so settling by id alone would settle whichever came first — which is
    // the failure `a-claim-id-is-unique-in-its-iteration` exists to name.
    let matching: Vec<String> = body
        .nodes()
        .iter()
        .filter(|n| n.name().value() == "claim" && root_id(n).as_deref() == Some(claim))
        .map(|n| praxis_core::schema::prop(n, "from-slice").unwrap_or_else(|| "(no slice)".to_owned()))
        .collect();
    if matching.is_empty() {
        miette::bail!("{iteration} carries no claim {claim}");
    }
    if matching.len() > 1 && slice.is_none() {
        miette::bail!(
            "{iteration} carries {claim} for {} slices ({}). Name which with `--slice` — \
             settling whichever comes first is the failure the claim-uniqueness rule is about",
            matching.len(),
            matching.join(", ")
        );
    }
    if let Some(want) = slice {
        if !matching.iter().any(|s| s == want) {
            miette::bail!("{iteration} carries no {claim} from {want} — it has {}", matching.join(", "));
        }
    }

    let target = body
        .nodes_mut()
        .iter_mut()
        .find(|n| {
            n.name().value() == "claim"
                && root_id(n).as_deref() == Some(claim)
                && slice.is_none_or(|want| praxis_core::schema::prop(n, "from-slice").as_deref() == Some(want))
        })
        .ok_or_else(|| miette::miette!("{iteration} carries no claim {claim}"))?;

    let children = target.children_mut().get_or_insert_with(kdl::KdlDocument::new);

    // Behaviour evolves, and a claim re-witnessed against new behaviour must not lose what
    // it settled on before. The superseded run is MATURED, not overwritten: the latest is
    // truth and the chain back is the maturation, which is what the record already uses in
    // place of a pointer (`TS.260821.18`, `TS.260823.02/C4`).
    let previous: Vec<(String, String)> = ["ran", "exited", "digest", "witnessed-at"]
        .iter()
        .filter_map(|field| {
            children
                .nodes()
                .iter()
                .find(|n| n.name().value() == *field)
                .and_then(|n| n.entries().first())
                .map(|e| ((*field).to_owned(), e.value().to_string()))
        })
        .collect();
    if !previous.is_empty() {
        let was: String = previous
            .iter()
            .map(|(f, v)| format!("{f} {v}"))
            .collect::<Vec<_>>()
            .join(", ");
        let matured = format!(
            "        matured \"ran\" to={:?} at={:?} \
             because=\"re-witnessed against changed behaviour; the superseded run is kept \
             because the chain back is what a pointer would have been\"\n",
            was, run.at
        );
        let block: kdl::KdlDocument = matured
            .parse()
            .map_err(|e| miette::Report::new(e).context("composing the maturation"))?;
        for node in block.nodes() {
            children.nodes_mut().push(node.clone());
        }
    }

    let block: kdl::KdlDocument = praxis_core::evidence::witnessed_kdl(run)
        .parse()
        .map_err(|e| miette::Report::new(e).context("composing the run"))?;
    children
        .nodes_mut()
        .retain(|n| !matches!(n.name().value(), "ran" | "exited" | "digest" | "witnessed-at"));
    for fresh in block.nodes() {
        children.nodes_mut().push(fresh.clone());
    }
    // The claim is MET now, and the state is a property on the claim node. Written through
    // the document model rather than by string surgery so everything else on the line —
    // `from-slice`, `text`, `settled-by`, a note somebody added — survives untouched.
    target.insert("state", "met");

    Ok((path.clone(), doc.to_string()))
}

/// `TS.260820.05` — the one gate. Either an iteration is open, or a refusal is on the
/// record naming the condition that failed. Never both, and never neither.
fn pickup(slices: &[String], root: &Path, asked_by: &str, dry_run: bool) -> miette::Result<bool> {
    let sources = load(root)?;
    let docs: Vec<_> = sources.iter().map(|(_, _, d)| d.clone()).collect();

    let (schema, _) = governing(docs.iter());
    let conditions = admission(docs.iter());
    if conditions.is_empty() {
        miette::bail!("the record declares no admission conditions — an empty gate is not an open one");
    }

    // The asker is named by whoever asks. It is NOT read from the tree.
    //
    // It used to be `git config user.email`, stripped and prefixed `human:` — which
    // reports whose machine the command ran on, and the question an admission answers is
    // who decided the work should start. The two are only ever equal by coincidence, and
    // an agent running in the maintainer's shell produced a signed human approval nobody
    // typed. `TS.260823.01`.
    let signer = asker(asked_by)?;
    let ask = Ask {
        signer,
        at: now(),
        by: "agent:praxis".to_owned(),
        // Pick-up is not an attestation. Whoever asks for work is not thereby vouching for it.
        attested_by: None,
    };

    let corpus = Corpus::from_documents(&docs, &schema);
    let assessment = assess(&corpus, &conditions, &ask.at);
    let taken = ids_in_use(&docs);

    // Where the record goes is a layout question, so the shell answers it — and it
    // answers it from where the SLICE lives, not from the state root. A record belongs to
    // the frame that raised the work, and only the tree knows which frame that is.
    // Every slice in one commitment must belong to one frame — the record has nowhere to
    // put a commitment that spans two.
    let mut homes: Vec<PathBuf> = Vec::new();
    for slice in slices {
        let home = frame_of(slice, &sources).ok_or_else(|| {
            miette::miette!("cannot tell which frame {slice} belongs to, so there is nowhere to put the record")
        })?;
        if !homes.contains(&home) {
            homes.push(home);
        }
    }
    if homes.len() > 1 {
        miette::bail!(
            "these slices belong to {} different frames. A commitment is one ask against one \
             problem, and the record has nowhere to put one that spans two",
            homes.len()
        );
    }
    let home = homes.remove(0);

    match pick_up(slices, &corpus, &assessment, &ask, &taken) {
        Pickup::NoSuchSlice(why) => miette::bail!("{why}"),
        Pickup::Opened(record) => {
            let path = home.join(&record.file);
            if dry_run {
                println!("would open {} at {}\n\n{}", record.id, path.display(), record.kdl);
            } else {
                write_once(&path, &record.kdl)?;
                println!("praxis: {} opened on {} — {}", record.id, slices.join(" · "), path.display());
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
fn close(
    iteration: &str,
    outcome: &str,
    attested_by: Option<&str>,
    root: &Path,
) -> miette::Result<bool> {
    let sources = load(root)?;
    let docs: Vec<_> = sources.iter().map(|(_, _, d)| d.clone()).collect();

    let (schema, _) = governing(docs.iter());
    let corpus = Corpus::from_documents(&docs, &schema);
    let ask = Ask {
        signer: human()?,
        at: now(),
        // What RAN and who ATTESTED are different facts. The trail keeps recording the tool;
        // the attestation carries the identity somebody typed (TS.260821.09/C3).
        by: "agent:praxis".to_owned(),
        attested_by: attested_by.map(str::to_owned),
    };
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
    // The attester is written as its own fact, beside the accounting and separate from the
    // trail's `by`. What RAN and who ATTESTED are different things, and conflating them is
    // the defect this records the fix for (TS.260821.09/C3).
    let attester = ask.attested_by.as_deref().unwrap_or_default();
    let closing: kdl::KdlDocument = format!(
        "    closed-at {:?}\n    outcome {outcome:?}\n\n    close {{\n{settled}        all-met #{all_met}\n        attested-by {attester:?}\n        note {note:?}\n    }}\n",
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

/// `TS.260820.16`. What a version contains stops being a list somebody keeps alongside
/// the record and becomes a fact about it.
fn binding(iteration: &str, version: &str, root: &Path, undo: bool) -> miette::Result<bool> {
    let sources = load(root)?;
    let docs: Vec<_> = sources.iter().map(|(_, _, d)| d.clone()).collect();
    let (schema, _) = governing(docs.iter());
    let corpus = Corpus::from_documents(&docs, &schema);
    let ask = Ask { signer: human()?, at: now(), by: "agent:praxis".to_owned(), attested_by: None };
    let taken = ids_in_use(&docs);

    let outcome = if undo {
        unbind(iteration, version, &corpus, &ask, &taken)
    } else {
        bind(iteration, version, &corpus, &ask, &taken)
    };

    match outcome {
        Binding::NotFound(why) => miette::bail!("{why}"),
        Binding::Refused(record) => {
            for (who, why) in &record.failed {
                eprintln!("praxis: bind refused — {who}: {why}");
            }
            let path = root.join(&record.file);
            write_once(&path, &record.kdl)?;
            eprintln!("praxis: {} recorded at {} — nothing bound", record.id, path.display());
            Ok(false)
        }
        Binding::Bound { release, proposal } => {
            // The release index is machine-owned, so it is written whole. An iteration is
            // a human's record, and that one is edited in place.
            let path = root.join(&release.file);
            if let Some(dir) = path.parent() {
                fs::create_dir_all(dir).map_err(|e| miette::miette!("{}: {e}", dir.display()))?;
            }
            fs::write(&path, &release.kdl)
                .map_err(|e| miette::miette!("{}: {e}", path.display()))?;
            println!(
                "praxis: {iteration} {} {version} — {}",
                if undo { "unbound from" } else { "bound to" },
                path.display()
            );
            println!("praxis: proposed bump {} — {}", proposal.position, proposal.because);
            for silent in &proposal.silent {
                println!("praxis: {silent} declares no `contributes`, so it informed nothing");
            }
            Ok(true)
        }
    }
}

/// `TS.260820.09`. A point on the version line, indexed to the commit it was cut at.
fn cutting(version: &str, root: &Path, confirm: bool) -> miette::Result<bool> {
    let sources = load(root)?;
    let docs: Vec<_> = sources.iter().map(|(_, _, d)| d.clone()).collect();
    let (schema, _) = governing(docs.iter());
    let corpus = Corpus::from_documents(&docs, &schema);
    let ask = Ask { signer: human()?, at: now(), by: "agent:praxis".to_owned(), attested_by: None };
    let commit = head_commit()?;

    // TS.260820.09/C3: the recorded commit must CONTAIN this release's published
    // directory. A commit taken at cut time cannot contain documents written afterwards,
    // so publishing comes first and this refuses until it has — which makes the
    // containment true by construction rather than by hoping the steps ran in order.
    published_and_committed(version)?;

    match cut(version, &commit, confirm, &corpus, &ask, &ids_in_use(&docs)) {
        Cut::NotFound(why) => miette::bail!("{why}"),
        Cut::Refused(record) => {
            for (condition, why) in &record.failed {
                eprintln!("praxis: cut refused — {condition}: {why}");
            }
            let path = root.join(&record.file);
            write_once(&path, &record.kdl)?;
            eprintln!("praxis: {} recorded at {} — {version} is still planned", record.id, path.display());
            Ok(false)
        }
        Cut::Made(record) => {
            // The release and its index are one write. There is no half-cut state to
            // recover from, because they were never two operations.
            let path = root.join(&record.file);
            let staging = path.with_extension("kdl.cutting");
            fs::write(&staging, &record.kdl)
                .map_err(|e| miette::miette!("{}: {e}", staging.display()))?;
            fs::rename(&staging, &path).map_err(|e| {
                let _ = fs::remove_file(&staging);
                miette::miette!("{}: {e}", path.display())
            })?;
            println!("praxis: {version} cut at {commit} — {}", path.display());
            println!("praxis: the index node is written. Nothing rebinds to it, and editing it is refused");
            Ok(true)
        }
    }
}

/// Whether this version's published set exists and is in the commit about to be indexed.
/// Both halves are needed: a directory that exists but is uncommitted is not in HEAD, and
/// an index pointing at a commit that lacks the documents is exactly what C3 forbids.
fn published_and_committed(version: &str) -> miette::Result<()> {
    let dir = format!("docs/releases/{version}");
    if !Path::new(&dir).is_dir() {
        miette::bail!(
            "{dir} does not exist. The index names a commit that must CONTAIN this release's \
             published set, so publish first: `praxis publish {version}`"
        );
    }
    let out = std::process::Command::new("git")
        .args(["status", "--porcelain", "--", &dir])
        .output()
        .map_err(|e| miette::miette!("cannot read the tree state: {e}"))?;
    let dirty = String::from_utf8_lossy(&out.stdout);
    if !dirty.trim().is_empty() {
        miette::bail!(
            "{dir} has uncommitted changes, so the commit about to be indexed does not contain \
             the published set as it stands:\n{}",
            dirty.trim()
        );
    }
    Ok(())
}

/// Where the tree is. What HEAD is is a fact about the tree, not about the record, so the
/// shell answers it — and a detached or absent git is a refusal rather than a guess.
fn head_commit() -> miette::Result<String> {
    let out = std::process::Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .map_err(|e| miette::miette!("cannot read the commit: {e}"))?;
    let commit = String::from_utf8_lossy(&out.stdout).trim().to_owned();
    if commit.is_empty() {
        miette::bail!(
            "no commit to index. A release without an index is a release nothing can be \
             verified against"
        );
    }
    Ok(commit)
}

/// `TS.260820.10`. Every document generated afresh and whole, stamped with the release it
/// depicts, under that release's own directory. There is no in-place edit anywhere here
/// and no substitution path to find.
fn publishing(version: &str, root: &Path, dry_run: bool) -> miette::Result<bool> {
    let sources = load(root)?;
    let docs: Vec<_> = sources.iter().map(|(_, _, d)| d.clone()).collect();
    let (schema, _) = governing(docs.iter());
    let corpus = Corpus::from_documents(&docs, &schema);

    match publish(version, &corpus) {
        Publication::Refused(reasons) => {
            for why in &reasons {
                eprintln!("praxis: publish refused — {why}");
            }
            Ok(false)
        }
        Publication::Ready { documents, .. } => {
            // Composed first, written second. A document is replaced whole or not at all,
            // and nothing outside this release's directory is touched.
            let mut rendered = Vec::new();
            for document in &documents {
                // Every chapter, not only the document's own result. A story whose index
                // satisfies the seam and whose fourth part does not is a document that
                // passes on its cover.
                for model in &document.models {
                    let flaws = model.flaws();
                    if !flaws.is_empty() {
                        for flaw in &flaws {
                            eprintln!(
                                "praxis: read-model@v1 violated in {} ({}) — {flaw}",
                                document.file, model.view
                            );
                        }
                        miette::bail!("nothing published: a result does not satisfy read-model@v1");
                    }
                }
                rendered.push((
                    PathBuf::from(&document.file),
                    render::render_story(&document.models, version),
                ));
            }

            for (path, text) in &rendered {
                if dry_run {
                    println!("would write {} ({} bytes)", path.display(), text.len());
                    continue;
                }
                if let Some(dir) = path.parent() {
                    fs::create_dir_all(dir)
                        .map_err(|e| miette::miette!("{}: {e}", dir.display()))?;
                }
                fs::write(path, text).map_err(|e| miette::miette!("{}: {e}", path.display()))?;
                println!("praxis: wrote {}", path.display());
            }
            Ok(true)
        }
    }
}

/// `TS.260820.11`. Either every published document is byte-identical to what was
/// published, or the edited one is named.
fn verifying(root: &Path) -> miette::Result<usize> {
    let sources = load(root)?;
    let docs: Vec<_> = sources.iter().map(|(_, _, d)| d.clone()).collect();
    let (schema, _) = governing(docs.iter());
    let corpus = Corpus::from_documents(&docs, &schema);

    let cut_releases: Vec<_> = corpus.releases.iter().filter(|r| r.cut()).collect();
    if cut_releases.is_empty() {
        // Not a pass and not a failure: there is genuinely nothing published yet. Said
        // out loud, because "verified" and "nothing to verify" must not read alike.
        println!("praxis: no cut release yet — nothing has been published to verify");
        return Ok(0);
    }

    let mut failures = 0;
    for release in cut_releases {
        let commit = index_commit(&docs, &release.id);
        let dir = format!("docs/releases/{}", release.version);
        let at_commit = commit.as_deref().and_then(|c| files_at(c, &dir));
        let in_tree = files_in_tree(&dir);

        match verify(&release.version, at_commit.as_deref(), &in_tree) {
            Verified::Clean { files, .. } => {
                println!("praxis: {} verified — {files} document(s) unchanged since publication", release.version);
            }
            Verified::Drifted { drift, .. } => {
                failures += 1;
                for item in &drift {
                    eprintln!("praxis: drift in {} — {}", release.version, item.message());
                }
            }
            Verified::Unverifiable { why, .. } => {
                failures += 1;
                eprintln!("praxis: {} cannot be verified — {why}", release.version);
            }
        }
    }
    if failures > 0 {
        eprintln!("praxis: {failures} release(s) failed verification");
    }
    Ok(failures)
}

/// The commit a release's index node names.
fn index_commit(docs: &[kdl::KdlDocument], release_id: &str) -> Option<String> {
    for doc in docs {
        for node in doc.nodes() {
            if node.name().value() != "release" || root_id(node).as_deref() != Some(release_id) {
                continue;
            }
            let index = node.iter_children().find(|c| c.name().value() == "index")?;
            let commit = index.iter_children().find(|c| c.name().value() == "commit")?;
            return commit
                .entries()
                .iter()
                .find(|e| e.name().is_none())
                .and_then(|e| e.value().as_string())
                .map(str::to_owned);
        }
    }
    None
}

/// Every file under `dir` at `commit`. `None` when the commit itself cannot be read —
/// which is a failure, never a skip.
fn files_at(commit: &str, dir: &str) -> Option<Vec<Published>> {
    let listed = std::process::Command::new("git")
        .args(["ls-tree", "-r", "--name-only", commit, "--", dir])
        .output()
        .ok()?;
    if !listed.status.success() {
        return None;
    }
    let mut out = Vec::new();
    for path in String::from_utf8_lossy(&listed.stdout).lines() {
        let blob = std::process::Command::new("git")
            .arg("show")
            .arg(format!("{commit}:{path}"))
            .output()
            .ok()?;
        if !blob.status.success() {
            return None;
        }
        out.push(Published { path: path.to_owned(), bytes: blob.stdout });
    }
    Some(out)
}

fn files_in_tree(dir: &str) -> Vec<Published> {
    let mut paths = Vec::new();
    walk_all(Path::new(dir), &mut paths);
    paths.sort();
    paths
        .into_iter()
        .filter_map(|path| {
            let bytes = fs::read(&path).ok()?;
            Some(Published { path: path.to_string_lossy().into_owned(), bytes })
        })
        .collect()
}

/// `TS.260820.17`. Current truth moves as a consequence of releasing, in one operation.
///
/// A failure part-way through leaves nothing moved: a half-promoted record is worse than
/// an unpromoted one, because it is wrong in a way nobody can see. Every file's new
/// content is composed first, every original is held, and any failure restores all of them.
fn promoting(version: &str, root: &Path, dry_run: bool) -> miette::Result<bool> {
    let sources = load(root)?;
    let docs: Vec<_> = sources.iter().map(|(_, _, d)| d.clone()).collect();
    let (schema, _) = governing(docs.iter());
    let corpus = Corpus::from_documents(&docs, &schema);

    let changes = match promote(version, &corpus) {
        Promotion::Refused(reasons) => {
            for why in &reasons {
                eprintln!("praxis: promotion refused — {why}");
            }
            return Ok(false);
        }
        Promotion::AlreadyPromoted { .. } => {
            println!("praxis: {version} is already promoted — nothing moved");
            return Ok(true);
        }
        Promotion::Ready { changes, .. } => changes,
    };

    // Compose everything before touching anything.
    let mut staged: Vec<(PathBuf, String, String)> = Vec::new();
    for change in &changes {
        let (path, text, _) = sources
            .iter()
            .find(|(_, _, doc)| holds_capability(doc, &change.capability))
            .ok_or_else(|| {
                miette::miette!("{} is in no file, so its truth has nowhere to move", change.capability)
            })?;
        let updated = apply_promotion(text, change)?;
        staged.push((path.clone(), text.clone(), updated));
    }

    if dry_run {
        for (path, _, _) in &staged {
            println!("would move {}", path.display());
        }
        return Ok(true);
    }

    let mut written: Vec<(PathBuf, String)> = Vec::new();
    for (path, original, updated) in &staged {
        match fs::write(path, updated) {
            Ok(()) => written.push((path.clone(), original.clone())),
            Err(e) => {
                // Move nothing. Restore every file already written, in reverse.
                for (done, was) in written.iter().rev() {
                    let _ = fs::write(done, was);
                }
                miette::bail!(
                    "{}: {e} — nothing moved. {} file(s) restored",
                    path.display(),
                    written.len()
                );
            }
        }
    }

    for change in &changes {
        println!(
            "praxis: {} — {} shipped entr{}{}",
            change.capability,
            change.shipped.len(),
            if change.shipped.len() == 1 { "y" } else { "ies" },
            if change.becomes_active { ", and now active" } else { "" }
        );
    }
    println!("praxis: {version} promoted into {} capability record(s)", changes.len());
    Ok(true)
}

fn holds_capability(doc: &kdl::KdlDocument, id: &str) -> bool {
    doc.nodes()
        .iter()
        .any(|n| n.name().value() == "capability" && root_id(n).as_deref() == Some(id))
}

/// Replace a capability's promoted block with the derived one, and move it out of
/// `sought`. Composed as parsed text so the file stays a file a person can read.
fn apply_promotion(text: &str, change: &praxis_core::Change) -> miette::Result<String> {
    let mut doc: kdl::KdlDocument = text
        .parse()
        .map_err(|e| miette::Report::new(e).context("reparsing to promote"))?;
    let node = doc
        .nodes_mut()
        .iter_mut()
        .find(|n| n.name().value() == "capability" && root_id(n).as_deref() == Some(&change.capability))
        .ok_or_else(|| miette::miette!("{} vanished between reading and writing", change.capability))?;
    let body = node
        .children_mut()
        .as_mut()
        .ok_or_else(|| miette::miette!("{} has no body", change.capability))?;

    if change.becomes_active {
        let active: kdl::KdlDocument = "    state \"active\"\n"
            .parse()
            .map_err(|e| miette::Report::new(e).context("composing the state"))?;
        if let Some(replacement) = active.nodes().first() {
            for child in body.nodes_mut().iter_mut() {
                if child.name().value() == "state" {
                    *child = replacement.clone();
                }
            }
        }
    }

    body.nodes_mut().retain(|n| n.name().value() != "shipped");
    let lines: String = change
        .shipped
        .iter()
        .map(|s| {
            format!(
                "    shipped {:?} by={:?} slice={:?}\n",
                s.version, s.iteration, s.slice
            )
        })
        .collect();
    let block: kdl::KdlDocument = format!("\n{lines}")
        .parse()
        .map_err(|e| miette::Report::new(e).context("composing the promoted truth"))?;

    let at = body
        .nodes()
        .iter()
        .position(|n| n.name().value() == "trail")
        .unwrap_or(body.nodes().len());
    for (offset, new) in block.nodes().iter().enumerate() {
        body.nodes_mut().insert(at + offset, new.clone());
    }
    Ok(doc.to_string())
}

/// `TS.260820.03`. An arriving agent asks instead of reconstructing.
fn truth(root: &Path) -> miette::Result<()> {
    let sources = load(root)?;
    let docs: Vec<_> = sources.iter().map(|(_, _, d)| d.clone()).collect();
    let (schema, _) = governing(docs.iter());
    let corpus = Corpus::from_documents(&docs, &schema);
    let model = what_is_currently_true(&corpus, &now());

    let flaws = model.flaws();
    if !flaws.is_empty() {
        for flaw in &flaws {
            eprintln!("praxis: read-model@v1 violated — {flaw}");
        }
        miette::bail!("the answer does not satisfy read-model@v1");
    }
    print!("{}", render::render(&model));
    Ok(())
}

/// `TS.260820.08`. A reviewer sees what an iteration promised and what it has shown, in
/// one answer, and reviewing leaves no residue.
///
/// There is no path under the archival root anywhere in this function. That is what makes
/// C2 structural: composing a preview cannot touch the published tree, because it has
/// nowhere to write to it.
fn reviewing(iteration: &str, root: &Path, markdown: bool) -> miette::Result<()> {
    let sources = load(root)?;
    let docs: Vec<_> = sources.iter().map(|(_, _, d)| d.clone()).collect();
    let (schema, _) = governing(docs.iter());
    let corpus = Corpus::from_documents(&docs, &schema);
    let model = review(iteration, &corpus, &now())
        .ok_or_else(|| miette::miette!("{iteration} is no iteration the record holds"))?;

    let flaws = model.flaws();
    if !flaws.is_empty() {
        for flaw in &flaws {
            eprintln!("praxis: read-model@v1 violated — {flaw}");
        }
        miette::bail!("the preview does not satisfy read-model@v1");
    }

    if !markdown {
        print!("{}", render::render(&model));
        return Ok(());
    }

    // The working projection path, which config.kdl declares and .gitignore excludes. A
    // preview cannot be committed by accident because the only place it lands is ignored.
    let path = root.join(".render").join(format!("{iteration}.preview.md"));
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir).map_err(|e| miette::miette!("{}: {e}", dir.display()))?;
    }
    fs::write(&path, render::render_markdown(&model, iteration))
        .map_err(|e| miette::miette!("{}: {e}", path.display()))?;
    println!("praxis: preview at {} — gitignored, and safe to delete", path.display());
    Ok(())
}

/// `TS.260820.15`. A guide describes what a version actually shipped, or it is refused.
fn guiding(capability: &str, version: &str, root: &Path) -> miette::Result<()> {
    let sources = load(root)?;
    let docs: Vec<_> = sources.iter().map(|(_, _, d)| d.clone()).collect();
    let (schema, _) = governing(docs.iter());
    let corpus = Corpus::from_documents(&docs, &schema);

    match guide_for(capability, version, &corpus) {
        Err(why) => miette::bail!("{}", why.message()),
        Ok(lines) => {
            println!("{capability} at {version}\n");
            for line in lines {
                println!("  {line}");
            }
            Ok(())
        }
    }
}

/// `TS.260820.13`. One document from several read models, each still singly owned, thrown
/// away after reading. There is no path under the archival root anywhere in this function.
/// `praxis view` — one declared read model, composed from the record now.
///
/// It refuses a name the record does not declare rather than composing it anyway: a view
/// nobody declared is a question nobody agreed was worth answering, and answering it here
/// would put the decision in the engine.
fn viewing(name: &str, at: Option<&str>, root: &Path) -> miette::Result<bool> {
    let sources = load(root)?;
    let docs: Vec<_> = sources.iter().map(|(_, _, d)| d.clone()).collect();
    let (schema, _) = governing(docs.iter());
    let corpus = Corpus::from_documents(&docs, &schema);

    let Some(view) = corpus.views.iter().find(|v| v.name == name) else {
        let declared: Vec<&str> = corpus.views.iter().map(|v| v.name.as_str()).collect();
        eprintln!(
            "praxis: `{name}` is no read model this record declares. It holds {}",
            declared.join(" · ")
        );
        return Ok(false);
    };

    // The version a composer needs. A view that depicts a release wants one; a view about
    // the tree does not, and the moment is what it says instead.
    let version = at.map(str::to_owned).unwrap_or_else(|| {
        corpus
            .releases
            .iter()
            .rev()
            .find(|r| r.cut())
            .map(|r| r.version.clone())
            .unwrap_or_else(|| "unreleased".to_owned())
    });

    let Some(mut model) = praxis_core::compose_view(&view.name, &version, &corpus) else {
        eprintln!(
            "praxis: `{name}` is declared and this engine has no composer for it — the record \
             asks a question the tool cannot yet answer"
        );
        return Ok(false);
    };
    // Composed on demand, so it says WHEN rather than which version — even for a view that
    // was archival until this release stopped publishing it. A result that claims to depict
    // a version it was not frozen at is the second copy this whole frame distrusts.
    model.publishable = false;
    model.as_of = now();

    let flaws = model.flaws();
    if !flaws.is_empty() {
        for flaw in &flaws {
            eprintln!("praxis: read-model@v1 violated in {name} — {flaw}");
        }
        miette::bail!("the view does not satisfy read-model@v1");
    }
    print!("{}", render::render(&model));
    Ok(true)
}

fn dashboarding(root: &Path) -> miette::Result<()> {
    let sources = load(root)?;
    let docs: Vec<_> = sources.iter().map(|(_, _, d)| d.clone()).collect();
    let (schema, _) = governing(docs.iter());
    let conditions = admission(docs.iter());
    let corpus = Corpus::from_documents(&docs, &schema);
    let document = dashboard(&corpus, &conditions, &now());

    if !document.owners_are_distinct() {
        miette::bail!(
            "a capability contributed more than one part. A DOCUMENT composes several read \
             models; a READ MODEL still has exactly one owner"
        );
    }
    for part in &document.parts {
        let flaws = part.model.flaws();
        if !flaws.is_empty() {
            for flaw in &flaws {
                eprintln!("praxis: read-model@v1 violated in {} — {flaw}", part.owner);
            }
            miette::bail!("the dashboard does not satisfy read-model@v1");
        }
    }
    print!("{}", render::render_composed(&document));
    Ok(())
}

/// `TS.260821.02`. Which of our gates has never fired.
/// Every instruction file this plugin ships, as the shell finds them.
///
/// This is the ONE place the tree is read. `TS.260821.03`/C4: the audit is pure and total
/// over its input, so what ships is a fact handed in — a core that walked a directory would
/// give a different answer depending on where it ran.
///
/// The four shapes are the four things this plugin ships as instruction. A file outside them
/// is not doctrine: `scripts/gen-*.sh` generates, `hooks/` wires, and neither tells an agent
/// what to do.
/// Every file this repository ships as instruction, as the RECORD declares its shapes.
///
/// `TS.260823.05`. This used to be a glob in the engine naming four shapes, and it reported
/// "55 of 55 anchored" — true of that set, and silent about fifty-three further files under
/// `skills/provision-project-overlay/templates/`, the tree copied INTO an adopter's
/// repository. A denominator chosen by the thing being measured is the trust-transfer
/// problem with a percentage sign after it.
///
/// The glob was the fault, not its width: A4 says the engine may not hold what the record
/// declares, and *which shapes are instruction* is a fact about this repository. Widening it
/// in place would have fixed today's number and left the next shape to be found the same way.
fn shipped_doctrine(root: &Path, shapes: &[(String, String)]) -> Vec<String> {
    let mut out = Vec::new();
    for (dir, pattern) in shapes {
        collect_matching(&root.join(dir), root, pattern, &mut out);
    }
    out.sort();
    out.dedup();
    out
}

/// Walk a directory, collecting every file whose name matches the pattern.
///
/// Recursive, because a template three directories down is read by an agent exactly as a
/// top-level skill is. The pattern is a filename shape — `*SKILL.md`, `check-*.sh` — never
/// a path, which is why it is exempt from the citation rule.
fn collect_matching(dir: &Path, root: &Path, pattern: &str, out: &mut Vec<String>) {
    let Ok(entries) = fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_matching(&path, root, pattern, out);
        } else if praxis_core::surface::matches_shape(&entry.file_name().to_string_lossy(), pattern)
            && let Ok(rel) = path.strip_prefix(root)
        {
            out.push(rel.to_string_lossy().replace('\\', "/"));
        }
    }
}

/// What every shipped instruction file SAYS, as `(path, contents)`.
///
/// `TS.260823.06`. The core reasons about the words and never reads them: which files ship
/// is already a fact the shell hands in, and what they contain is the same kind of fact one
/// level in. A file that cannot be read is omitted rather than reported — the audit above
/// already found it, and a second complaint about the same file names one fault twice.
fn shipped_text(root: &Path, shipped: &[String]) -> Vec<(String, String)> {
    shipped
        .iter()
        .filter_map(|path| Some((path.clone(), fs::read_to_string(root.join(path)).ok()?)))
        .collect()
}

/// The shipped files whose ANCHORING a named slice owes, as `(path, slice)`.
fn owed_anchors(root: &Path, docs: &[kdl::KdlDocument]) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for doc in docs {
        for config in doc.nodes().iter().filter(|n| n.name().value() == "config") {
            let Some(body) = config.children() else { continue };
            let Some(block) = body.nodes().iter().find(|n| n.name().value() == "ships-doctrine")
            else {
                continue;
            };
            let Some(inner) = block.children() else { continue };
            for shape in inner.nodes().iter().filter(|n| n.name().value() == "from") {
                let Some(owed_to) = praxis_core::schema::prop(shape, "owed-to") else { continue };
                let Some(dir) = shape.entries().first().and_then(|e| e.value().as_string()) else {
                    continue;
                };
                let Some(named) = praxis_core::schema::prop(shape, "named") else { continue };
                let mut found = Vec::new();
                collect_matching(&root.join(dir), root, &named, &mut found);
                out.extend(found.into_iter().map(|p| (p, owed_to.clone())));
            }
        }
    }
    out
}

/// `TS.260821.03`. Audit what the plugin ships against what the record declares.
fn auditing(root: &Path, from: &Path) -> miette::Result<bool> {
    let sources = load(root)?;
    let (schema, _) = governing(sources.iter().map(|(_, _, d)| d));
    if schema.is_empty() {
        miette::bail!("the record declares no schema, so nothing declares what a surface is");
    }
    if schema.entity("doctrine-surface").is_none() {
        miette::bail!(
            "the record does not declare `doctrine-surface`, so it has no way to say what this \
             plugin ships. See TS.260821.03"
        );
    }

    let docs: Vec<_> = sources.into_iter().map(|(_, _, doc)| doc).collect();
    let corpus = Corpus::from_documents(&docs, &schema);
    let shapes = praxis_core::surface::declared_shapes(&docs);
    if shapes.is_empty() {
        miette::bail!(
            "this repository declares no `ships-doctrine` in its config, so what counts as \
             shipped instruction is undeclared. An audit whose denominator lives in the engine \
             measures whatever the engine chose to look at"
        );
    }
    let shipped = shipped_doctrine(from, &shapes);
    let audit = audit(surfaces(&corpus), &shipped);

    for (id, path) in &audit.absent {
        println!("praxis: {id} declares {path} — REFUSED, the plugin does not ship it");
    }
    for (id, path) in &audit.still_shipped {
        println!("praxis: {id} is retired and {path} is still in the tree — REFUSED");
    }
    for path in &audit.unanchored {
        println!("praxis: {path} — UNANCHORED, no doctrine-surface declares it");
    }

    println!(
        "praxis: {} of {} shipped surfaces are anchored",
        audit.anchored.len(),
        shipped.len()
    );
    Ok(audit.clean())
}

/// `TS.260821.05`. What this plugin guarantees, and what keeps each guarantee.
/// `TS.260821.13`. The pointer and everything the cut wrote elsewhere move together.
fn withdrawing(version: &str, because: Option<&str>, root: &Path) -> miette::Result<bool> {
    let sources = load(root)?;
    let docs: Vec<_> = sources.iter().map(|(_, _, d)| d.clone()).collect();
    let (schema, _) = governing(docs.iter());
    let corpus = Corpus::from_documents(&docs, &schema);

    let taken = match withdraw(version, because, &corpus) {
        Withdrawal::Refused(why) => {
            eprintln!("praxis: withdrawal refused — {why}");
            return Ok(false);
        }
        Withdrawal::Taken { unwound, .. } => unwound,
    };
    let because = because.unwrap_or_default();

    // Both halves, or neither. A withdrawal that flips the marker and leaves capability
    // records asserting the release has moved the problem rather than undone it.
    let mut staged: Vec<(PathBuf, String)> = Vec::new();
    for (path, text, doc) in &sources {
        let holds_release = doc.nodes().iter().any(|n| {
            n.name().value() == "release" && child_of(n, "version").as_deref() == Some(version)
        });
        if holds_release {
            staged.push((path.clone(), mark_withdrawn(text, version, because)?));
            continue;
        }
        let touched = doc.nodes().iter().any(|n| {
            n.name().value() == "capability"
                && string_id(n).is_some_and(|id| taken.iter().any(|t| t == &id))
        });
        if touched {
            staged.push((path.clone(), unwind_promotion(text, version)));
        }
    }

    // Every file, or none. `write_once` is for records the gate CREATES; these already
    // exist, and the point of the command is that both halves move together.
    for (path, text) in &staged {
        fs::write(path, text).map_err(|e| miette::miette!("{}: {e}", path.display()))?;
    }

    println!("praxis: {version} withdrawn — the pointer is back, and it says why");
    if taken.is_empty() {
        println!("praxis: nothing had been promoted from it");
    } else {
        println!("praxis: promoted truth recomputed away from {}", taken.join(" · "));
    }
    Ok(true)
}

/// Turn a cut release into a withdrawn one: the state, the reason, and the index kept as
/// what WAS cut so that this and a version never cut do not read alike.
fn mark_withdrawn(text: &str, version: &str, because: &str) -> miette::Result<String> {
    let mut out = String::new();
    let mut inside = false;
    let mut dropping = false;
    let mut depth = 0usize;
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("release ") {
            inside = true;
        }
        if inside {
            depth += line.matches('{').count();
            depth = depth.saturating_sub(line.matches('}').count());
        }
        if inside && trimmed == "state \"released\"" {
            out.push_str("    state \"withdrawn\"\n");
            out.push_str("    withdrawn-because #\"\"\"\n");
            for sentence in because.split(". ") {
                out.push_str(&format!("        {}\n", sentence.trim()));
            }
            out.push_str("        \"\"\"#\n");
            continue;
        }
        // The index becomes what WAS cut. Re-cutting carries it forward.
        if inside && trimmed == "index {" {
            out.push_str(&line.replace("index {", "previously-cut {"));
            out.push('\n');
            continue;
        }
        // The seal covered a cut release's content, and there is no longer a cut to seal.
        // Dropped WITH its continuation lines: a KDL node ending in `\` carries on, and
        // removing only the first line orphans the rest into a parse error.
        if dropping || (inside && trimmed.starts_with("seal ")) {
            dropping = line.trim_end().ends_with('\\');
            continue;
        }
        out.push_str(line);
        out.push('\n');
        if inside && depth == 0 && trimmed == "}" {
            inside = false;
        }
    }
    let _ = version;
    Ok(out)
}

/// Recompute promoted truth away from a withdrawn release.
///
/// Line-oriented rather than a document rewrite, so the surrounding prose — which a human
/// wrote and no rule derives — is left exactly as it is.
fn unwind_promotion(text: &str, version: &str) -> String {
    let marker = format!("shipped \"{version}\"");
    let kept: Vec<&str> = text.lines().filter(|l| !l.trim_start().starts_with(&marker)).collect();
    let mut out = kept.join("\n");
    if !text.lines().any(|l| l.trim_start().starts_with("shipped \"")) || !out.contains("shipped \"")
    {
        // Nothing shipped any more, so the capability is sought again — promotion is what
        // made it active, and it has been recomputed away.
        out = out.replacen("    state \"active\"", "    state \"sought\"", 1);
    }
    if text.ends_with('\n') && !out.ends_with('\n') {
        out.push('\n');
    }
    out
}

fn child_of(node: &kdl::KdlNode, field: &str) -> Option<String> {
    node.children()?
        .nodes()
        .iter()
        .find(|n| n.name().value() == field)
        .and_then(|n| n.entries().first())
        .and_then(|e| e.value().as_string())
        .map(str::to_owned)
}

fn string_id(node: &kdl::KdlNode) -> Option<String> {
    node.entries().first()?.value().as_string().map(str::to_owned)
}

/// `TS.260821.11`. Ask the record a question and print the answer.
fn asking(
    kind: &str,
    wheres: &[String],
    show: Option<&str>,
    count: bool,
    unreferenced_by: Option<&str>,
    root: &Path,
) -> miette::Result<bool> {
    let sources = load(root)?;
    let docs: Vec<_> = sources.into_iter().map(|(_, _, doc)| doc).collect();
    let (schema, _) = governing(docs.iter());

    let mut question = Question {
        kind: kind.to_owned(),
        show: show
            .map(|s| s.split(',').map(str::trim).filter(|s| !s.is_empty()).map(str::to_owned).collect())
            .unwrap_or_default(),
        ..Question::default()
    };
    for predicate in wheres {
        let Some((field, value)) = predicate.split_once('=') else {
            eprintln!("praxis: `{predicate}` is not a predicate — it reads `--where field=value`");
            return Ok(false);
        };
        question.where_.push((field.trim().to_owned(), value.trim().to_owned()));
    }
    if let Some(edge) = unreferenced_by {
        let Some((k, f)) = edge.split_once('.') else {
            eprintln!("praxis: `{edge}` is not an edge — it reads `--unreferenced-by kind.field`");
            return Ok(false);
        };
        question.unreferenced_by = Some((k.to_owned(), f.to_owned()));
    }

    match ask(&docs, &schema, &question) {
        Answer::Refused(why) => {
            eprintln!("praxis: {why}");
            Ok(false)
        }
        // The count alone, on stdout, with no prose around it. The usage log says four
        // commands were run twenty-five times to read four integers, and two of them had
        // their framing stripped by `sed` on the way past.
        Answer::Rows { rows, .. } if count => {
            println!("{}", rows.len());
            Ok(true)
        }
        Answer::Rows { columns, rows } => {
            let model = ReadModel::new(
                format!("{kind}s the record holds"),
                format!("which {kind} records match?"),
                &now(),
            );
            let mut section = Section::new(kind, &columns.iter().map(String::as_str).collect::<Vec<_>>())
                .empty_because("no record of that kind matches — which is an answer, not a gap");
            for row in rows {
                section.push(row);
            }
            print!("{}", render::render(&model.section(section)));
            Ok(true)
        }
    }
}

fn invariants(root: &Path) -> miette::Result<()> {
    let sources = load(root)?;
    let (schema, _) = governing(sources.iter().map(|(_, _, d)| d));
    if schema.is_empty() {
        miette::bail!("the record declares no schema, so nothing declares what an invariant is");
    }
    if schema.entity("invariant").is_none() {
        miette::bail!(
            "the record does not declare `invariant`, so what this plugin guarantees is not \
             something it can be asked about. See TS.260821.05"
        );
    }

    let docs: Vec<_> = sources.into_iter().map(|(_, _, doc)| doc).collect();
    let corpus = Corpus::from_documents(&docs, &schema);
    let found = check_invariants(&corpus);

    let (mut kept, mut taught, mut omitted, mut unkept) = (0, 0, 0, 0);
    for enforcement in &found {
        match enforcement {
            Enforcement::Kept { invariant, by } => {
                kept += 1;
                println!("praxis: {invariant} — kept by {}", by.join(" · "));
            }
            Enforcement::Omitted { invariant, because } => {
                omitted += 1;
                println!("praxis: {invariant} — {because}, so it is not owed");
            }
            Enforcement::Taught { invariant, by } => {
                taught += 1;
                println!(
                    "praxis: {invariant} — TAUGHT by {}, and no probe checks it",
                    by.join(" · ")
                );
            }
            Enforcement::Unkept { invariant } => {
                unkept += 1;
                println!("praxis: {invariant} — UNKEPT, nothing the record holds enforces it");
            }
        }
    }
    println!(
        "praxis: {kept} of {} declared invariants have a probe · {taught} taught and unchecked · \
         {omitted} omitted by this profile · {unkept} neither",
        found.len()
    );
    Ok(())
}

fn proving(root: &Path) -> miette::Result<usize> {
    let sources = load(root)?;
    let (schema, _) = governing(sources.iter().map(|(_, _, d)| d));
    if schema.is_empty() {
        miette::bail!("the record declares no schema, so there are no rules to prove");
    }

    let proofs = prove(&schema);
    let (witnessed, unwitnessed): (Vec<_>, Vec<_>) = proofs.iter().partition(|p| p.proven());

    for proof in &witnessed {
        if let praxis_core::Proof::Witnessed { rule, refusals } = proof {
            println!("praxis: {rule} — witnessed, {refusals} refusal(s)");
        }
    }
    for proof in &unwitnessed {
        if let praxis_core::Proof::Unwitnessed { rule, why } = proof {
            eprintln!("praxis: {rule} — UNWITNESSED, {why}");
        }
    }
    println!(
        "praxis: {} of {} declared rules are witnessed",
        witnessed.len(),
        proofs.len()
    );
    Ok(unwitnessed.len())
}

/// `TS.260820.14`/AE3 and `TS.260820.18`/C3. Seal what has become immutable.
///
/// The seal is computed from the record's own content by the same function the rule uses
/// to check it, so sealing and checking cannot disagree about what was sealed.
fn accepting(root: &Path, dry_run: bool) -> miette::Result<()> {
    let sources = load(root)?;
    let mut sealed = 0;

    for (path, text, _) in &sources {
        let mut doc: kdl::KdlDocument = match text.parse() {
            Ok(doc) => doc,
            Err(_) => continue,
        };
        let mut touched = false;

        for node in doc.nodes_mut().iter_mut() {
            let kind = node.name().value().to_owned();
            let Some(body) = node.children_mut().as_mut() else { continue };
            for child in body.nodes_mut().iter_mut() {
                let name = child.name().value().to_owned();
                let (is_target, material) = match (kind.as_str(), name.as_str()) {
                    ("iteration", "decision") => {
                        let accepted = child
                            .get("state")
                            .and_then(|v| v.as_string())
                            .unwrap_or("accepted")
                            == "accepted";
                        let title = child
                            .entries()
                            .iter()
                            .find(|e| e.name().is_none())
                            .and_then(|e| e.value().as_string())
                            .unwrap_or_default()
                            .to_owned();
                        (accepted, decision_body(child, &title))
                    }
                    ("frame", "symptom") => {
                        let state =
                            child.get("state").and_then(|v| v.as_string()).unwrap_or_default();
                        let id = child
                            .entries()
                            .iter()
                            .find(|e| e.name().is_none())
                            .and_then(|e| e.value().as_string())
                            .unwrap_or_default()
                            .to_owned();
                        let resolved = matches!(state, "resolved" | "partially-resolved");
                        let by = child.get("resolved-by").and_then(|v| v.as_string()).unwrap_or("");
                        (resolved, format!("{id}\u{1f}{state}\u{1f}{by}"))
                    }
                    _ => (false, String::new()),
                };
                if !is_target || child.get("seal").is_some() {
                    continue;
                }
                let computed = seal(&material, "", &[], &[]);
                if dry_run {
                    println!("would seal {name} in {}", path.display());
                } else {
                    child.push(("seal", computed.as_str()));
                    touched = true;
                }
                sealed += 1;
            }
        }

        if touched && !dry_run {
            fs::write(path, doc.to_string())
                .map_err(|e| miette::miette!("{}: {e}", path.display()))?;
            println!("praxis: sealed {}", path.display());
        }
    }

    println!(
        "praxis: {sealed} record(s) {}",
        if dry_run { "would be sealed" } else { "sealed" }
    );
    Ok(())
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
/// The schema governing a tree: the method the engine carries, extended by whatever the
/// repository declares.
///
/// `TS.260821.10`. Before this, every command took the LAST schema it found — so a project
/// declaring its own would have REPLACED the method rather than extended it, and nothing
/// would have said so. The method is the floor now, and it is not removable.
fn governing<'a>(docs: impl Iterator<Item = &'a kdl::KdlDocument>) -> (Schema, Vec<Extension>) {
    let (mut schema, _) = Schema::method();
    let mut extensions = Vec::new();
    for doc in docs {
        // The method's own record is in the tree — it ships there, so an agent with the
        // plugin and no binary can read it. Extending the method with itself would report
        // every kind it declares as a redefinition of every kind it declares.
        if doc.nodes().iter().any(|n| n.name().value() == "method") {
            continue;
        }
        let local = Schema::from_document(doc);
        if !local.is_empty() {
            extensions.extend(schema.extend(local));
        }
    }
    (schema, extensions)
}

/// The gate, from the method — and from the repository only if the method carries none.
///
/// Admission conditions are NORMATIVE: a repository that could rewrite its own has no gate.
/// The fallback exists for a tree whose method predates TS.260821.10, not as a licence.
fn admission<'a>(docs: impl Iterator<Item = &'a kdl::KdlDocument>) -> Conditions {
    let method: kdl::KdlDocument = praxis_core::schema::METHOD.parse().expect("the method parses");
    let from_method = Conditions::from_document(&method);
    if !from_method.is_empty() {
        return from_method;
    }
    let mut conditions = Conditions::default();
    for doc in docs {
        let declared = Conditions::from_document(doc);
        if !declared.is_empty() {
            conditions = declared;
        }
    }
    conditions
}

/// The identity of whoever asked for work to start, as they gave it.
///
/// `TS.260823.01`. Two things are refused here and nothing else is checked, because
/// nothing else CAN be: the record can hold who was named, and never whether they meant
/// it. Pretending otherwise would repeat one level up the fault this function removes.
///
/// - an empty ask, because an admission with no asker admits on nobody's word
/// - an `agent:` namespace, because an agent admitting its own work is the whole subject
///
/// A bare name is taken as a human. Requiring the prefix would teach people to type
/// `human:` as a formality, and a formality is what the git identity had become.
fn asker(given: &str) -> miette::Result<String> {
    let given = given.trim();
    if given.is_empty() {
        miette::bail!(
            "`--asked-by` is empty, so there is nobody the admission is on the word of. The \
             ask is the permission and it needs somebody to have made it"
        );
    }
    if let Some(rest) = given.strip_prefix("agent:") {
        miette::bail!(
            "`{rest}` is in the agent namespace, and an agent may not admit its own work. \
             Name whoever asked for it — the ask through a prompt is the permission, and \
             this records whose ask it was"
        );
    }
    Ok(match given.strip_prefix("human:") {
        Some(name) => format!("human:{}", name.trim()),
        None => format!("human:{given}"),
    })
}

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

    let (schema, _) = governing(docs.iter());
    let conditions = admission(docs.iter());
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

    let (schema, extensions) = governing(sources.iter().map(|(_, _, d)| d));
    let mut known = Known::default();
    for (_, _, doc) in &sources {
        index_all(doc, &mut known);
    }
    // A repository extends the method; it never redefines it. Reported here rather than
    // inside `governing` because `check` is the command whose job is saying what is wrong,
    // and a silent composition is how the engine used to take the LAST schema it found.
    let mut redefinitions = 0;
    for extension in &extensions {
        if let Extension::Redefines(what) = extension {
            redefinitions += 1;
            eprintln!(
                "praxis: {what} is declared by the method and redeclared here. A repository \
                 EXTENDS the method and never redefines it — rename it, or drop it and use \
                 what the method declares"
            );
        }
    }
    if schema.is_empty() {
        miette::bail!("the record declares no schema — nothing to check against");
    }

    // Rules that need the whole record at once — uniqueness and coverage — are decided
    // over the corpus and attributed back to the file that raised them.
    let docs: Vec<_> = sources.iter().map(|(_, _, d)| d.clone()).collect();
    // What the tree holds, for the rules the record alone cannot decide (TS.260821.03).
    // The shapes come from the record here too. A repository that declares none audits
    // nothing rather than falling back to a set the engine chose (`TS.260823.05`).
    let shipped = shipped_doctrine(Path::new("."), &praxis_core::surface::declared_shapes(&docs));
    let facts = Facts {
        text: shipped_text(Path::new("."), &shipped),
        shipped,
        owed: owed_anchors(Path::new("."), &docs),
    };
    let corpus = check_corpus_given(&docs, &schema, &facts);

    // Redefinitions count as refusals: a repository running a variant of the method is not
    // running the method, and the whole of TS.260821.10 is that nobody could tell.
    let (mut refusals, mut reports) = (redefinitions, 0);
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

    // Corpus violations that belong to no document. A rule about what is ABSENT from the
    // record — or absent from the tree — has no file to be attributed to, and the per-file
    // loop above would drop it silently. That is AG1 exactly: a correct rule computing a
    // refusal that never reaches the output (ITER.260821.17).
    for violation in corpus.iter().filter(|v| !docs.iter().any(|d| belongs(v, d))) {
        match violation.severity() {
            Severity::Refuse => refusals += 1,
            Severity::Report => reports += 1,
        }
        eprintln!("praxis: {} — {}", violation.refusal.rule(), violation.refusal.message());
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
        // Nested too. A symptom lives inside its frame and a decision inside its
        // iteration, so a search of root nodes alone COMPUTES the refusal and then drops
        // it — which is the shape S3 warns about, one layer down (ITER.260821.17/AG1).
        Some(id) => holds(doc.nodes(), &violation.entity_kind, id),
        // An anonymous violation can only be placed by kind, which is imprecise — so a
        // corpus rule that cannot name the entity it is about is a rule to reconsider.
        None => doc.nodes().iter().any(|n| n.name().value() == violation.entity_kind),
    }
}

/// Whether these nodes, at any depth, hold an entity of this KIND with this id. Both
/// halves are needed: an id alone matches `attacks "S1"` as readily as `symptom "S1"`, and
/// a kind alone matches every record of that kind in the tree.
fn holds(nodes: &[kdl::KdlNode], kind: &str, id: &str) -> bool {
    nodes.iter().any(|n| {
        (n.name().value() == kind && root_id(n).as_deref() == Some(id))
            || n.children().is_some_and(|kids| holds(kids.nodes(), kind, id))
    })
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
    collect(dir, out, Some("kdl"));
}

/// Every file under a directory. Verification compares what is THERE, not what it
/// expected to find — a filter would let an added file hide behind its extension.
fn walk_all(dir: &Path, out: &mut Vec<PathBuf>) {
    collect(dir, out, None);
}

fn collect(dir: &Path, out: &mut Vec<PathBuf>, extension: Option<&str>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect(&path, out, extension);
        } else if extension.is_none_or(|want| path.extension().is_some_and(|e| e == want)) {
            out.push(path);
        }
    }
}
