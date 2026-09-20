//! `TS.260823.08`: make the first thing an adopter runs put them under the method.
//!
//! **A method is whatever its front door installs.** Everything downstream of the first hour
//! is a correction to it, and a correction an adopter has to discover on their own is one
//! most will not make.
//!
//! What this replaced was two skills that split greenfield from brownfield and both installed
//! the spine the record deleted two versions earlier: a wave directory, a sprint placeholder,
//! fifty-three overlay templates, and a generated `verify.sh` calling two probes the plugin
//! does not ship — one of them described in its own comment as hard-fail with no warn mode.
//! Neither skill mentioned the delivery graph once.
//!
//! Everything here is PURE. Adoption composes a set of files and the shell writes them, for
//! the same reason every other rule in this engine takes its facts as arguments: a core that
//! wrote to disk would be a core whose answer depends on where it ran.

use kdl::KdlDocument;

/// The four files adoption writes, as templates that ship with the plugin.
///
/// Embedded rather than read from the tree, and `include_str!` points at the shipped file so
/// there is nothing to keep in sync — the same two-carrier argument the method makes. An
/// adopter with the plugin and no binary can read them; a binary installed with no plugin
/// still writes them.
const CONFIG: &str = include_str!("../../../skills/adopt-the-method/templates/config.kdl.tmpl");
const VERIFY: &str = include_str!("../../../skills/adopt-the-method/templates/verify.sh.tmpl");
const HOOK: &str = include_str!("../../../skills/adopt-the-method/templates/pre-commit.tmpl");
const OVERLAY: &str = include_str!(
    "../../../skills/adopt-the-method/templates/capability-structure.instructions.md.tmpl"
);

/// The record of what this plugin ships as a probe.
///
/// The generated `verify.sh` is derived from THIS, never from a list maintained beside it.
/// That is the whole of `C3`: a script naming a probe the plugin does not ship is not a
/// mistake somebody made once, it is what happens to every hand-maintained list eventually.
const PROBES: &str = include_str!("../../../praxis/surfaces/SURFACES.probes.kdl");

/// What the adopter answered, and what the method is.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Binding {
    pub repository: String,
    /// `<method>@v<version>`, from the engine rather than from the caller.
    pub method: String,
    pub profile: String,
    pub stage: String,
    pub source_root: String,
    pub language: String,
    /// What `praxis evidence` runs. Declared once, in the config, never on a command line.
    pub runner: String,
    pub select_with: String,
    pub format: String,
    pub lint: String,
    pub typecheck: String,
    pub test: String,
}

impl Binding {
    /// A binding with everything unanswered left as a shell no-op and a visible placeholder.
    ///
    /// `:` rather than a guessed command. A generated step that runs the wrong linter fails
    /// for a reason the adopter did not cause, and the first thing they will do is delete
    /// the step — which loses the four beside it.
    #[must_use]
    pub fn new(repository: &str, method: &str) -> Self {
        Self {
            repository: repository.to_owned(),
            method: method.to_owned(),
            profile: "product".to_owned(),
            stage: "pre-1.0".to_owned(),
            source_root: "src/".to_owned(),
            language: "unstated".to_owned(),
            runner: ":".to_owned(),
            select_with: "--test".to_owned(),
            format: ":".to_owned(),
            lint: ":".to_owned(),
            typecheck: ":".to_owned(),
            test: ":".to_owned(),
        }
    }
}

/// One file adoption writes: where it goes, what is in it, and whether it is executable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Written {
    pub path: String,
    pub content: String,
    pub executable: bool,
}

/// Why adoption refused.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Refused {
    /// The repository is already bound. Adoption writes a binding; overwriting one silently
    /// would replace the answer to "which method governs this repository" with the last
    /// person to run a command.
    AlreadyBound { path: String },
    /// Adoption was asked for a repository with no name. Every record this method holds is
    /// attributed, and the config is where the attribution starts.
    NoRepository,
}

impl Refused {
    #[must_use]
    pub fn message(&self) -> String {
        match self {
            Self::AlreadyBound { path } => format!(
                "{path} already binds this repository to a method. Adoption writes a binding \
                 and will not replace one — read it, and if it names a method you no longer \
                 want, change `governed-by` deliberately rather than by re-running a command"
            ),
            Self::NoRepository => {
                "adoption needs the repository's name, and deliberately has no default. A git \
                 remote names whose machine this is; it does not name what the record is about"
                    .to_owned()
            }
        }
    }
}

/// Compose everything adoption writes.
///
/// `bound` is whether the target already carries a config — a fact about the world, handed
/// in. Returns the files in the order they should be written, or the refusal.
///
/// # Errors
/// Refuses a target already bound to a method, and a binding with no repository name.
pub fn adopt(binding: &Binding, bound: bool) -> Result<Vec<Written>, Refused> {
    if binding.repository.trim().is_empty() {
        return Err(Refused::NoRepository);
    }
    if bound {
        return Err(Refused::AlreadyBound { path: "praxis/config.kdl".to_owned() });
    }

    let probes = shipped_probes();
    Ok(vec![
        Written {
            path: "praxis/config.kdl".to_owned(),
            content: fill(CONFIG, &pairs(binding)),
            executable: false,
        },
        Written {
            path: "scripts/verify.sh".to_owned(),
            content: fill(
                VERIFY,
                &[
                    pairs(binding),
                    vec![
                        ("probe-steps".to_owned(), probe_steps(&probes)),
                        ("probe-pipeline".to_owned(), probe_pipeline(&probes)),
                    ],
                ]
                .concat(),
            ),
            executable: true,
        },
        Written {
            path: ".githooks/pre-commit".to_owned(),
            content: fill(HOOK, &pairs(binding)),
            executable: true,
        },
        Written {
            path: ".github/instructions/capability-structure.instructions.md".to_owned(),
            content: fill(OVERLAY, &pairs(binding)),
            executable: false,
        },
    ])
}

/// Every probe this plugin ships, as `(step name, script path)`.
///
/// Retired surfaces are excluded, which is the point: the two probes the previous front door
/// generated calls to had been retired by `TS.260821.04` and removed from the tree, and
/// nothing connected the retirement to the script that called them.
#[must_use]
pub fn shipped_probes() -> Vec<(String, String)> {
    let Ok(doc) = PROBES.parse::<KdlDocument>() else { return Vec::new() };
    let mut out = Vec::new();
    for node in doc.nodes().iter().filter(|n| n.name().value() == "doctrine-surface") {
        if crate::check::child_arg(node, "kind").as_deref() != Some("probe") {
            continue;
        }
        if crate::check::child_arg(node, "state").as_deref() == Some("retired") {
            continue;
        }
        let Some(path) = crate::check::child_arg(node, "path") else { continue };
        let name = path
            .rsplit('/')
            .next()
            .and_then(|f| f.strip_suffix(".sh"))
            .and_then(|f| f.strip_prefix("check-"))
            .unwrap_or_default()
            .to_owned();
        if !name.is_empty() {
            out.push((name, path));
        }
    }
    out
}

fn probe_steps(probes: &[(String, String)]) -> String {
    probes
        .iter()
        // No trailing `.`: every probe that takes a root defaults to the working directory,
        // and the two that do not would silently ignore the argument. An argument passed to
        // a script that never reads it is a convention nobody can check.
        .map(|(name, path)| format!("step_{}() {{ bash {path}; }}", name.replace('-', "_")))
        .collect::<Vec<_>>()
        .join("\n")
}

fn probe_pipeline(probes: &[(String, String)]) -> String {
    probes
        .iter()
        .map(|(name, _)| format!("run_step {name:<24} step_{}", name.replace('-', "_")))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Every key a template may substitute, in the order `pairs` supplies them.
///
/// The shell lint reads the same list out of the canon and refuses a template asking for
/// anything outside it; a test asserts the two agree. One declaration, two consumers — a key
/// added to the engine and not the canon fails on both sides, which is the property the
/// previous arrangement lost when it made one of the templates the authority.
#[must_use]
pub fn placeholder_keys() -> Vec<String> {
    pairs(&Binding::new("_", "_")).into_iter().map(|(key, _)| key).chain([
        "probe-steps".to_owned(),
        "probe-pipeline".to_owned(),
    ]).collect()
}

fn pairs(binding: &Binding) -> Vec<(String, String)> {
    vec![
        ("repository".to_owned(), binding.repository.clone()),
        ("method".to_owned(), binding.method.clone()),
        ("profile".to_owned(), binding.profile.clone()),
        ("stage".to_owned(), binding.stage.clone()),
        ("source-root".to_owned(), binding.source_root.clone()),
        ("language".to_owned(), binding.language.clone()),
        ("runner".to_owned(), binding.runner.clone()),
        ("select-with".to_owned(), binding.select_with.clone()),
        ("format".to_owned(), binding.format.clone()),
        ("lint".to_owned(), binding.lint.clone()),
        ("typecheck".to_owned(), binding.typecheck.clone()),
        ("test".to_owned(), binding.test.clone()),
    ]
}

/// Plain `{{key}}` substitution. No conditionals and no loops: a template language is a
/// second program nobody tests, and a template that needs branching is two templates.
fn fill(template: &str, values: &[(String, String)]) -> String {
    let mut out = template.to_owned();
    for (key, value) in values {
        out = out.replace(&format!("{{{{{key}}}}}"), value);
    }
    out
}

/// Any `{{…}}` a fill left behind.
///
/// A placeholder that reached an adopter's tree is a file that reads as configured and is
/// not, which is the failure the old overlay's placeholder-parity lint existed to catch.
#[must_use]
pub fn unresolved(content: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = content;
    while let Some(start) = rest.find("{{") {
        let Some(end) = rest[start..].find("}}") else { break };
        out.push(rest[start..start + end + 2].to_owned());
        rest = &rest[start + end + 2..];
    }
    out.sort();
    out.dedup();
    out
}
