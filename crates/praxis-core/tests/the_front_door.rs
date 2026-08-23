//! `TS.260823.08` — make the first thing an adopter runs put them under the method.
//!
//! The two skills the index named first installed the spine the record had deleted: a wave
//! directory, a sprint placeholder, fifty-one overlay templates, and a generated `verify.sh`
//! calling two probes the plugin does not ship — one of them described in its own comment as
//! "HARD-FAIL by design, no warn mode". `adopt-the-method` was correct and third, so nobody
//! reached it.
//!
//! A method is whatever its front door installs. These four claims are about what it installs
//! now, and C4 is the one that says what it deliberately does not.

use praxis_core::adopt::{Binding, Refused, adopt, shipped_probes, unresolved};

const METHOD: &str = "delivery-graph@v1";

fn written() -> Vec<praxis_core::adopt::Written> {
    let mut binding = Binding::new("acme/checkout", METHOD);
    binding.language = "typescript".to_owned();
    binding.test = "npm test".to_owned();
    binding.runner = "npm test".to_owned();
    adopt(&binding, false).expect("an unbound tree adopts")
}

fn file(name: &str) -> String {
    written()
        .into_iter()
        .find(|f| f.path == name)
        .unwrap_or_else(|| panic!("adoption writes {name}"))
        .content
}

/// C1, first half — adoption writes a config bound to the method the engine carries.
///
/// The version comes from the engine, never from the caller. A binding to a method the
/// engine does not carry is a repository checked against rules nobody can see, which is
/// worse than no binding because it reads as one.
#[test]
fn adoption_writes_a_binding_to_the_method_the_engine_carries() {
    let config = file("praxis/config.kdl");
    assert!(config.contains(&format!("governed-by \"{METHOD}\"")), "{config}");
    assert!(config.contains("repository \"acme/checkout\""));
    assert!(config.contains("verification"), "how this repository verifies is declared once");
    assert!(config.contains("runner \"npm test\""));
}

/// C1, second half — what adoption wrote checks clean.
///
/// Run against a real empty tree through the real binary, because the claim is about a
/// repository and not about a fixture. One report is expected and is the honest answer: a
/// record holding only a binding names nobody it is for yet.
#[test]
fn what_adoption_writes_passes_praxis_check() {
    let tmp = temp_dir("front-door-check");
    let out = std::process::Command::new(binary())
        .current_dir(&tmp)
        .args(["adopt", "--repository", "acme/checkout", "--test", "npm test"])
        .output()
        .expect("the engine runs");
    assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));

    let checked = std::process::Command::new(binary())
        .current_dir(&tmp)
        .arg("check")
        .output()
        .expect("the engine runs");
    let said = String::from_utf8_lossy(&checked.stderr);
    assert!(
        checked.status.success(),
        "a freshly adopted tree must check clean, and said:\n{said}"
    );
    let _ = std::fs::remove_dir_all(&tmp);
}

/// Adoption refuses a repository that is already bound, rather than replacing the answer to
/// "which method governs this" with whoever ran a command last.
#[test]
fn adoption_refuses_a_repository_already_bound() {
    let binding = Binding::new("acme/checkout", METHOD);
    let refused = adopt(&binding, true).expect_err("an already-bound tree is refused");
    assert!(matches!(refused, Refused::AlreadyBound { .. }));
    assert!(refused.message().contains("praxis/config.kdl"), "{}", refused.message());
}

/// And one with no name. Every record this method holds is attributed, and the config is
/// where the attribution starts — a git remote names whose machine this is, not what the
/// record is about.
#[test]
fn adoption_refuses_a_repository_with_no_name() {
    let binding = Binding::new("   ", METHOD);
    assert!(matches!(adopt(&binding, false), Err(Refused::NoRepository)));
}

/// C2 — nothing adoption writes names a retired kind.
///
/// `TS.260823.06`'s rule, applied to the output of this slice. The templates are inside the
/// shipped set and `praxis check` already refuses them if they teach one; this asserts the
/// RENDERED output too, because substitution could reintroduce a word the template avoided.
#[test]
fn nothing_adoption_writes_names_a_retired_kind() {
    let (schema, _) = praxis_core::Schema::method();
    let words = schema.retired_words();
    assert!(!words.is_empty(), "the method declares a retired vocabulary to check against");

    for file in written() {
        let found = praxis_core::surface::teaching(
            &file.path,
            &file.content,
            &words,
            schema.explained_by(),
        );
        assert!(
            found.is_empty(),
            "{} teaches {:?}",
            file.path,
            found.iter().map(|t| (t.line, &t.word)).collect::<Vec<_>>()
        );
    }
}

/// No placeholder survives into an adopter's tree. A file that reads as configured and is
/// not is worse than one that is obviously blank.
#[test]
fn no_placeholder_reaches_the_adopter() {
    for file in written() {
        assert!(unresolved(&file.content).is_empty(), "{} left {:?}", file.path, unresolved(&file.content));
    }
}

/// C3 — the generated verify entry point names only probes this plugin ships.
///
/// The whole slice in one assertion. Its predecessor called `check-sprint-id-collision.sh`
/// and `check-design-approval-gate.sh`, both retired and both removed from the tree, and the
/// second was the one gate the plugin described as failing closed. A gate that cannot run is
/// worse than no gate, because it reads as one.
#[test]
fn the_generated_gate_names_only_probes_that_ship() {
    let verify = file("scripts/verify.sh");
    let root = std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../.."));

    // Trailing `;` and `}` come with the shell, not with the path.
    let named: Vec<&str> = verify
        .split_whitespace()
        .map(|w| w.trim_end_matches([';', '}', ')']))
        .filter(|w| w.starts_with("scripts/check-") && w.ends_with(".sh"))
        .collect();
    assert!(!named.is_empty(), "the generated gate runs some probes");

    for probe in &named {
        assert!(
            root.join(probe).exists(),
            "the generated verify.sh calls {probe}, which this plugin does not ship"
        );
    }

    // And the derivation is total: every probe the record declares is in the script, so the
    // list cannot fall behind the tree either.
    for (_, path) in shipped_probes() {
        assert!(named.contains(&path.as_str()), "{path} ships and the generated gate skips it");
    }
}

/// A retired probe is not generated, which is the edge the previous front door fell off.
#[test]
fn a_retired_probe_is_not_generated() {
    let generated = shipped_probes();
    for dead in ["check-sprint-id-collision", "check-design-approval-gate", "check-contract-freshness"] {
        assert!(
            !generated.iter().any(|(_, p)| p.contains(dead)),
            "{dead} was retired and the generated gate still calls it"
        );
    }
}

/// C4 — the engineering-discipline doctrine survives adoption and the spine scaffolding does
/// not.
///
/// Both halves, because either alone is the wrong outcome. Deleting the overlay entirely
/// would have thrown away doctrine that is about CODE and was never spine-bound; keeping it
/// as it was would have shipped sixteen skill templates restating the plugin's own rules in
/// somebody else's tree.
#[test]
fn the_engineering_doctrine_survives_and_the_spine_scaffolding_does_not() {
    let overlay = file(".github/instructions/capability-structure.instructions.md");

    for kept in [
        "design-capability-layout",
        "define-seam-contract",
        "test-by-ownership",
        "implement-with-defensive-patterns",
        "check-anti-dumping.sh",
        "functional core",
    ] {
        assert!(overlay.contains(kept), "the overlay lost `{kept}`, which is about code");
    }

    let paths: Vec<String> = written().into_iter().map(|f| f.path).collect();
    for absent in ["INIT.", "SPRINT.", "waves/", "sprints/", "praxis.config.yaml"] {
        assert!(
            !paths.iter().any(|p| p.contains(absent)),
            "adoption writes {absent}, which the record retired"
        );
    }
    assert_eq!(paths.len(), 4, "four files: {paths:?}");
}

/// The overlay POINTS rather than restates. A copy of the plugin's doctrine in the host tree
/// is a second copy, and the second copy is what drifts.
#[test]
fn the_overlay_points_rather_than_restates() {
    let overlay = file(".github/instructions/capability-structure.instructions.md");
    assert!(overlay.contains("pointer"), "the overlay says what it is");
    assert!(
        overlay.lines().count() < 130,
        "the overlay is {} lines — past about this length it is restating doctrine that \
         belongs in the plugin skill",
        overlay.lines().count()
    );
}

fn binary() -> std::path::PathBuf {
    std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/debug/praxis"))
        .to_path_buf()
}

fn temp_dir(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("praxis-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("a temp tree");
    dir
}

/// The engine and the canon agree on what a template may ask for.
///
/// `validate-plugin.sh` refuses a template naming a key outside `placeholderKeys`; this
/// refuses an engine that fills a different set. Either alone would let the two drift — and
/// the drift is invisible until a `{{key}}` reaches somebody else's repository looking
/// configured.
#[test]
fn the_engine_and_the_canon_agree_on_the_placeholder_keys() {
    let canon = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../.praxis-canon.json"
    ))
    .expect("the canon is readable");

    let declared: Vec<String> = canon
        .split("\"placeholderKeys\"")
        .nth(1)
        .and_then(|rest| rest.split(']').next())
        .map(|block| {
            // Odd-indexed pieces of a `"`-split are the quoted strings; the even ones are
            // the commas and whitespace between them.
            block
                .split('"')
                .skip(1)
                .step_by(2)
                .map(str::to_owned)
                .collect()
        })
        .unwrap_or_default();

    let mut from_engine = praxis_core::adopt::placeholder_keys();
    let mut from_canon = declared;
    from_engine.sort();
    from_canon.sort();
    assert_eq!(
        from_engine, from_canon,
        "the engine fills one set of keys and the canon declares another"
    );
}
