//! `TS.260820.11` — prove a published document was never hand-edited after the fact.
//!
//! C2 is the load-bearing claim, and it is load-bearing for a reason that has nothing to
//! do with drift: a check that fails continuously is a check everyone disables, and that
//! failure mode is likelier than the thing the check exists to catch.

use praxis_core::{Drift, Published, Verified, verify};

fn file(path: &str, body: &str) -> Published {
    Published { path: path.to_owned(), bytes: body.as_bytes().to_vec() }
}

const AS_PUBLISHED: &str = "# What shipped\n\n> Depicts **0.1.0**, and nothing else.\n";

#[test]
fn c1_a_hand_edited_document_fails_and_is_named() {
    let published = [file("docs/releases/0.1.0/what-shipped.md", AS_PUBLISHED)];
    let edited = [file(
        "docs/releases/0.1.0/what-shipped.md",
        "# What shipped\n\n> Depicts **0.1.0**, and nothing else.\n\nand one more thing\n",
    )];

    let result = verify("0.1.0", Some(&published), &edited);
    assert!(result.failed());
    let Verified::Drifted { drift, .. } = result else { panic!("expected drift") };
    assert_eq!(drift.len(), 1);
    assert_eq!(drift[0].path(), "docs/releases/0.1.0/what-shipped.md", "the document is NAMED");
    assert!(matches!(drift[0], Drift::Edited { .. }));
    assert!(drift[0].message().contains("only superseded"));
}

#[test]
fn c1_a_one_byte_change_is_still_a_change() {
    let published = [file("docs/releases/0.1.0/a.md", AS_PUBLISHED)];
    let edited = [file("docs/releases/0.1.0/a.md", &AS_PUBLISHED.replace("0.1.0", "0.1.1"))];
    assert!(verify("0.1.0", Some(&published), &edited).failed(), "compared as bytes");
}

#[test]
fn c2_resuming_work_does_not_make_the_check_fail() {
    // The comparison is against the COMMIT the release names, not against the record. A
    // record that has moved on is not evidence about a document published before it did.
    let published = [file("docs/releases/0.1.0/what-shipped.md", AS_PUBLISHED)];
    let unchanged = published.clone();
    assert!(!verify("0.1.0", Some(&published), &unchanged).failed());

    // There is nowhere in this function to pass the current record, which is what makes
    // the property structural rather than a discipline someone has to keep.
    assert_eq!(verify("0.1.0", Some(&published), &unchanged), Verified::Clean {
        release: "0.1.0".to_owned(),
        files: 1
    });
}

#[test]
fn c2_a_later_release_publishing_its_own_documents_does_not_disturb_this_one() {
    let published = [file("docs/releases/0.1.0/a.md", AS_PUBLISHED)];
    // 0.2.0's directory is not in 0.1.0's comparison at all — different directory,
    // different commit, different question.
    let in_tree = [file("docs/releases/0.1.0/a.md", AS_PUBLISHED)];
    assert!(!verify("0.1.0", Some(&published), &in_tree).failed());
}

#[test]
fn c4_a_commit_that_cannot_be_read_fails_rather_than_skipping() {
    let in_tree = [file("docs/releases/0.1.0/a.md", AS_PUBLISHED)];
    let result = verify("0.1.0", None, &in_tree);
    assert!(result.failed(), "a check that silently skips is how verification gets disabled");
    let Verified::Unverifiable { why, .. } = result else { panic!("expected unverifiable") };
    assert!(why.contains("shallow clone"));
    assert!(why.contains("Failing rather than skipping"));
}

#[test]
fn c4_a_commit_holding_no_published_directory_fails_too() {
    let in_tree = [file("docs/releases/0.1.0/a.md", AS_PUBLISHED)];
    let result = verify("0.1.0", Some(&[]), &in_tree);
    assert!(result.failed());
    assert!(matches!(result, Verified::Unverifiable { .. }));
}

#[test]
fn a_published_document_that_was_deleted_is_drift() {
    let published = [file("docs/releases/0.1.0/a.md", AS_PUBLISHED)];
    let result = verify("0.1.0", Some(&published), &[]);
    let Verified::Drifted { drift, .. } = result else { panic!("expected drift") };
    assert!(matches!(drift[0], Drift::Removed { .. }));
    assert!(drift[0].message().contains("do not stop being that"));
}

#[test]
fn a_file_added_to_a_published_directory_is_drift() {
    // Nothing says what release an added file depicts, which is the whole property a
    // published directory has.
    let published = [file("docs/releases/0.1.0/a.md", AS_PUBLISHED)];
    let in_tree = [file("docs/releases/0.1.0/a.md", AS_PUBLISHED), file("docs/releases/0.1.0/notes.md", "mine")];
    let Verified::Drifted { drift, .. } = verify("0.1.0", Some(&published), &in_tree) else {
        panic!("expected drift")
    };
    assert_eq!(drift.len(), 1);
    assert!(matches!(drift[0], Drift::Added { .. }));
}

#[test]
fn every_drifted_document_is_named_not_just_counted() {
    let published = [
        file("docs/releases/0.1.0/a.md", "one"),
        file("docs/releases/0.1.0/b.md", "two"),
        file("docs/releases/0.1.0/c.md", "three"),
    ];
    let in_tree = [
        file("docs/releases/0.1.0/a.md", "one"),
        file("docs/releases/0.1.0/b.md", "EDITED"),
        file("docs/releases/0.1.0/c.md", "ALSO EDITED"),
    ];
    let Verified::Drifted { drift, .. } = verify("0.1.0", Some(&published), &in_tree) else {
        panic!("expected drift")
    };
    assert_eq!(drift.len(), 2);
    let named: Vec<&str> = drift.iter().map(Drift::path).collect();
    assert!(named.contains(&"docs/releases/0.1.0/b.md") && named.contains(&"docs/releases/0.1.0/c.md"));
    assert!(!named.contains(&"docs/releases/0.1.0/a.md"), "and the clean one is not accused");
}
