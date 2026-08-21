//! Publishing. `TS.260820.10`: regenerate every published document whole, each stating the
//! release it depicts.
//!
//! A document that is spliced can be corrupted; a document that is regenerated cannot.
//! There is no in-place edit here and no substitution path — every document is composed
//! from the record, whole, or it is not written.
//!
//! What is published is a `read-model@v1` result like any other, so the renderer needs no
//! code path specific to any one view. That is finding `E10`'s falsifier stated as an
//! acceptance criterion: if projection ever needs to know what a view MEANS, this
//! capability and the content-owning ones merge.

use crate::admission::Corpus;
use crate::view::{ReadModel, Section};

/// One document to be written, and the result it depicts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Document {
    /// Relative to the archival root — `docs/releases/<version>/<name>.md`.
    pub file: String,
    pub model: ReadModel,
}

/// Everything a publish would write, composed before anything is written.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Publication {
    Ready { version: String, documents: Vec<Document> },
    /// Refused, and why. Nothing is written.
    Refused(Vec<String>),
}

/// Compose the published set for a version.
///
/// It publishes for a version that is **planned, not yet cut**. That ordering was forced
/// by building: `TS.260820.09` requires the recorded commit to CONTAIN this release's
/// published directory, and a commit taken at cut time cannot contain documents written
/// afterwards. So the set is published, committed, and only then cut — which makes the
/// containment true by construction rather than by hoping the steps ran in order.
/// There is deliberately no `at` parameter. An archival result names the version it
/// depicts and never the moment it was produced: a wall-clock reading anywhere in the
/// document makes every re-render differ from the last, and verification of a published
/// tree is a comparison (`TS.260820.11`). A function that took the moment and then had to
/// remember not to use it would be one refactor away from using it.
pub fn publish(version: &str, corpus: &Corpus) -> Publication {
    let Some(release) = corpus.release(version) else {
        return Publication::Refused(vec![format!(
            "{version} is no release the record holds — bind work to it before publishing for it"
        )]);
    };
    if release.cut() {
        return Publication::Refused(vec![format!(
            "{version} is already cut. The published set must exist in the commit the index \
             names, so it is written BEFORE the cut, never after"
        )]);
    }
    if release.binds.is_empty() {
        return Publication::Refused(vec![format!(
            "{version} binds nothing, so there is no shipped work to describe"
        )]);
    }

    // Membership is DERIVED from the lifetime declarations and from nothing else. An
    // engine that composes its own list has taken the decision back from the record
    // (TS.260820.12).
    let mut documents = Vec::new();
    let mut uncomposable = Vec::new();
    for view in corpus.views.iter().filter(|v| v.publishable == Some(true)) {
        let Some(lands_at) = &view.publishes_to else {
            uncomposable.push(format!(
                "{} is publishable and declares no `publishes-to`, so there is nowhere to put it",
                view.name
            ));
            continue;
        };
        let model = match view.name.as_str() {
            "the-published-set-for-a-release" => published_set(version, corpus),
            "capabilities-and-what-they-own" => capabilities(version, corpus),
            "the-decisions-that-shaped-this" => decisions(version, corpus),
            other => {
                uncomposable.push(format!(
                    "{other} is declared publishable and the engine has no composer for it. The \
                     published set must equal the publishable declarations exactly, so this is a \
                     refusal rather than a shorter set"
                ));
                continue;
            }
        };
        documents.push(Document { file: landing(lands_at, version, &view.name), model });
    }

    if !uncomposable.is_empty() {
        return Publication::Refused(uncomposable);
    }
    if documents.is_empty() {
        return Publication::Refused(vec![format!(
            "the record declares no publishable view, so {version} would publish nothing. That is \
             a gap in the declarations rather than an empty release"
        )]);
    }

    Publication::Ready { version: version.to_owned(), documents }
}

/// `the-decisions-that-shaped-this` — what was decided, what the alternatives were, and
/// what tested it.
///
/// `E19`'s second use: claim-settlement carries the decision, its amendments and the
/// findings that tested it. What discovery rejected is delivery-record's half, and reaches
/// a reader through this same document once that half exists.
fn decisions(version: &str, corpus: &Corpus) -> ReadModel {
    let mut made = Section::new(
        "what was decided",
        &["decision", "forced by", "chose", "would be shown wrong by"],
    )
    .empty_because("nothing recorded — no iteration has had to make a choice explicitly");
    let mut rejected = Section::new("what it rejected", &["decision", "alternative"])
        .empty_because("nothing recorded — no decision names an alternative");
    let mut tested = Section::new("what tested it", &["decision", "finding", "from"])
        .empty_because("nothing recorded — no finding names a decision it tested");
    let mut amended = Section::new("corrections", &["decision", "amendment"])
        .empty_because("nothing recorded — no decision has been amended");

    for decision in &corpus.decisions {
        made.push(vec![
            decision.title.clone(),
            decision.iteration.clone(),
            decision.chose.clone(),
            decision
                .falsified_by
                .clone()
                .unwrap_or_else(|| "nothing named — which makes it a preference".to_owned()),
        ]);
        for alternative in &decision.over {
            rejected.push(vec![decision.title.clone(), alternative.clone()]);
        }
        for amendment in &decision.amendments {
            amended.push(vec![decision.title.clone(), amendment.clone()]);
        }
        // C4: the findings that tested it, reachable FROM the decision.
        for attempt in &corpus.attempts {
            for finding in attempt.findings.iter().filter(|f| f.tests.as_deref() == Some(decision.title.as_str())) {
                tested.push(vec![
                    decision.title.clone(),
                    finding.id.clone(),
                    attempt.id.clone(),
                ]);
            }
        }
    }

    let mut model = ReadModel::new(
        "the-decisions-that-shaped-this",
        "what was decided, what were the alternatives, and what tested it?",
        version,
    );
    model.publishable = true;
    model.section(made).section(rejected).section(tested).section(amended)
}

/// Where a view lands, from its own `publishes-to`. A path ending in `/` is a directory,
/// and the document inside it is named for the view — so the record decides the location
/// and the engine only fills in the version.
fn landing(declared: &str, version: &str, view: &str) -> String {
    let path = declared.replace("<version>", version);
    if path.ends_with('/') { format!("{path}{view}.md") } else { path }
}

/// `the-published-set-for-a-release` — what shipped at version N, which symptoms it
/// resolved, and what it left owed.
fn published_set(version: &str, corpus: &Corpus) -> ReadModel {
    let release = corpus.release(version);
    let mut shipped = Section::new("what shipped", &["iteration", "slice", "contributes"])
        .empty_because("this version binds nothing");
    let mut owed = Section::new("what it left owed", &["iteration", "finding", "carries"])
        .empty_because("no bound iteration carried a finding");

    for id in release.map(|r| r.binds.as_slice()).unwrap_or_default() {
        let Some(attempt) = corpus.attempts.iter().find(|a| &a.id == id) else {
            continue;
        };
        let slug = corpus
            .slice(&attempt.on_slice)
            .map_or_else(|| attempt.on_slice.clone(), |s| format!("{} — {}", attempt.on_slice, s.slug));
        shipped.push(vec![
            attempt.id.clone(),
            slug,
            if attempt.contributes.is_empty() {
                "not declared".to_owned()
            } else {
                attempt.contributes.join(" · ")
            },
        ]);
        for finding in &attempt.findings {
            owed.push(vec![
                attempt.id.clone(),
                finding.id.clone(),
                finding.carries.clone().unwrap_or_else(|| "—".to_owned()),
            ]);
        }
    }

    let mut resolved = Section::new("symptoms it resolved", &["symptom"])
        .empty_because("no symptom names this version as what resolved it");
    for symptom in corpus.resolved_by(version) {
        resolved.push(vec![symptom.to_owned()]);
    }

    let mut model = ReadModel::new(
        "the-published-set-for-a-release",
        format!("what shipped at {version}, which symptoms it resolved, and what it left owed"),
        // The "moment" of an archival result is the version it depicts.
        version,
    );
    model.publishable = true;
    model.section(shipped).section(resolved).section(owed)
}

/// `capabilities-and-what-they-own` — what the system must be able to do, and which events
/// each keeps consistent.
fn capabilities(version: &str, corpus: &Corpus) -> ReadModel {
    let mut section = Section::new("capabilities", &["capability", "derived from", "events owned"])
        .empty_because("the record names no capability");
    for capability in &corpus.capabilities {
        section.push(vec![
            capability.id.clone(),
            if capability.from_cluster.is_empty() {
                "not declared".to_owned()
            } else {
                capability.from_cluster.clone()
            },
            capability.owns.len().to_string(),
        ]);
    }

    let mut owns = Section::new("what each owns", &["capability", "event"])
        .empty_because("no capability declares an owned event");
    for capability in &corpus.capabilities {
        for event in &capability.owns {
            owns.push(vec![capability.id.clone(), event.clone()]);
        }
    }

    let mut model = ReadModel::new(
        "capabilities-and-what-they-own",
        "what must the system be able to do, and which events does each keep consistent?",
        version,
    );
    model.publishable = true;
    // An archival result names the version it depicts — read-model@v1's fourth constraint,
    // and the whole of "true for exactly one version and false for every other".
    model.section(section).section(owns).define("depicts", version)
}
