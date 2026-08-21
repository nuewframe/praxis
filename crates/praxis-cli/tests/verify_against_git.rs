//! `TS.260820.11`, end to end, against a real repository.
//!
//! The comparison logic is settled in `praxis-core`'s tests. What this settles is the part
//! that only a real git can answer: that the commit an index node names is read, that a
//! byte differing in the working tree is caught and NAMED, and that a commit which cannot
//! be read fails the check instead of being skipped.

use std::fs;
use std::path::Path;
use std::process::Command;

fn git(dir: &Path, args: &[&str]) -> String {
    let out = Command::new("git")
        .args(args)
        .current_dir(dir)
        .output()
        .expect("git runs");
    assert!(out.status.success(), "git {args:?} failed: {}", String::from_utf8_lossy(&out.stderr));
    String::from_utf8_lossy(&out.stdout).trim().to_owned()
}

fn praxis(dir: &Path) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_praxis"))
        .args(["verify-published", "praxis"])
        .current_dir(dir)
        .output()
        .expect("praxis runs")
}

/// A repository with one published, cut release. Returns its path.
fn published_repo(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("praxis-verify-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(dir.join("docs/releases/0.1.0")).expect("dirs");
    fs::create_dir_all(dir.join("praxis/releases")).expect("dirs");

    fs::write(
        dir.join("docs/releases/0.1.0/what-shipped.md"),
        "# What shipped\n\n> Depicts **0.1.0**, and nothing else.\n",
    )
    .expect("write");

    git(&dir, &["init", "--quiet"]);
    git(&dir, &["config", "user.email", "test@example.com"]);
    git(&dir, &["config", "user.name", "Test"]);
    git(&dir, &["add", "-A"]);
    git(&dir, &["commit", "--quiet", "-m", "publish 0.1.0"]);
    let commit = git(&dir, &["rev-parse", "HEAD"]);

    // The record is written AFTER the publish commit, which is the ordering the frame
    // requires: the index names a commit that already contains the documents.
    write_record(&dir, &commit);
    dir
}

fn write_record(dir: &Path, commit: &str) {
    fs::write(
        dir.join("praxis/schema.kdl"),
        r##"
notional-architecture "NA.test" {
    schema {
        entity "release" {
            field "version" each="1"
            field "state"   each="1"
            field "index"   each="0..1"
        }
    }
}
"##,
    )
    .expect("write");
    fs::write(
        dir.join("praxis/releases/REL.0.1.0.kdl"),
        format!(
            r##"
release "REL.0.1.0" {{
    version "0.1.0"
    state "released"

    index {{
        tag "v0.1.0"
        commit {commit:?}
    }}
}}
"##
        ),
    )
    .expect("write");
}

#[test]
fn an_untouched_published_tree_verifies() {
    let dir = published_repo("clean");
    let out = praxis(&dir);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(out.status.success(), "stderr: {}", String::from_utf8_lossy(&out.stderr));
    assert!(stdout.contains("0.1.0 verified"), "{stdout}");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn c1_a_hand_edited_published_document_fails_the_check_by_name() {
    let dir = published_repo("edited");
    fs::write(
        dir.join("docs/releases/0.1.0/what-shipped.md"),
        "# What shipped\n\n> Depicts **0.1.0**, and nothing else.\n\nand a line nobody published\n",
    )
    .expect("write");

    let out = praxis(&dir);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(!out.status.success(), "a hand edit must fail the check");
    assert!(stderr.contains("what-shipped.md"), "and the document is named: {stderr}");
    assert!(stderr.contains("drift in 0.1.0"), "{stderr}");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn c2_committing_more_work_afterwards_does_not_disturb_the_check() {
    // The record moves on; the published document does not, and is not supposed to.
    let dir = published_repo("moved-on");
    fs::write(dir.join("praxis/later.kdl"), "// work that happened after the release\n").expect("write");
    git(&dir, &["add", "-A"]);
    git(&dir, &["commit", "--quiet", "-m", "more work"]);

    let out = praxis(&dir);
    assert!(
        out.status.success(),
        "a check that fails once work resumes is a check everyone disables: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn c4_a_commit_that_cannot_be_read_fails_rather_than_skipping() {
    let dir = published_repo("missing-commit");
    write_record(&dir, "0000000000000000000000000000000000000000");

    let out = praxis(&dir);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(!out.status.success(), "an unreadable commit must fail, not skip");
    assert!(stderr.contains("cannot be verified"), "{stderr}");
    assert!(stderr.contains("shallow clone"), "and it says what would cause that: {stderr}");
    let _ = fs::remove_dir_all(&dir);
}

/// `TS.260820.08`/C2 — composing a preview writes nothing to the published tree.
#[test]
fn reviewing_leaves_the_published_tree_byte_unchanged() {
    let dir = published_repo("preview");
    let published = dir.join("docs/releases/0.1.0/what-shipped.md");
    let before = fs::read(&published).expect("read");

    fs::write(
        dir.join("praxis/iteration.kdl"),
        r##"
thin-slice "TS.a" {
    slug "a-thing"
}
iteration "ITER.1" {
    on-slice "TS.a"
    state "working"
}
"##,
    )
    .expect("write");

    let out = Command::new(env!("CARGO_BIN_EXE_praxis"))
        .args(["review", "ITER.1", "praxis", "--markdown"])
        .current_dir(&dir)
        .output()
        .expect("praxis runs");
    assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));

    let after = fs::read(&published).expect("read");
    assert_eq!(before, after, "reviewing leaves no residue in the published tree");

    // And the preview landed in the working projection path, which is gitignored.
    let preview = dir.join("praxis/.render/ITER.1.preview.md");
    let text = fs::read_to_string(&preview).expect("the preview exists");
    assert!(
        text.contains("**PREVIEW — not a record.**"),
        "C1: a preview is never mistakable for a record: {text}"
    );
    let _ = fs::remove_dir_all(&dir);
}
