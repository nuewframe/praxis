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
            "how-to-use-a-capability" => crate::guide::guides(version, corpus),
            "what-this-plugin-ships" => shipped_doctrine(version, corpus),
            "what-this-product-means" => concepts(version, corpus),
            "how-it-fits-together" => architecture(version, corpus),
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
pub(crate) fn published_set(version: &str, corpus: &Corpus) -> ReadModel {
    let release = corpus.release(version);
    let bound: Vec<&crate::admission::Attempt> = release
        .map(|r| r.binds.as_slice())
        .unwrap_or_default()
        .iter()
        .filter_map(|id| corpus.attempts.iter().find(|a| &a.id == id))
        .collect();

    // Every slice this version shipped, once, in the words the slice itself uses. A slice
    // worked by two iterations is one thing a reader gained, not two.
    let mut slices: Vec<&crate::admission::Slice> = Vec::new();
    for attempt in &bound {
        for id in &attempt.on_slices {
            if let Some(slice) = corpus.slice(id)
                && !slices.iter().any(|s| s.id == slice.id)
            {
                slices.push(slice);
            }
        }
    }

    // The first question. `useful-alone` is written as "what you get if this ships and
    // nothing after it does", which is a release note line already.
    // One column, and the sentence looked up beside it. `read-model@v1` constraint 2: prose
    // is REFERENCED, not inlined — a row names the thing and the paragraph a human wrote
    // about it is defined by name. Inlining it would also mean sanitising the record's own
    // words to fit a cell, and a publisher that edits its source is a second author.
    let mut gained = Section::new("what you can do now", &["what changed"])
        .empty_because("this version binds no slice, so nothing about what you can do changed");
    let mut why: Vec<(String, String)> = Vec::new();
    for slice in &slices {
        let heading =
            if slice.title.is_empty() { slice.slug.clone() } else { slice.title.clone() };
        gained.push(vec![heading.clone()]);
        let prose = slice.useful_alone.clone().unwrap_or_else(|| slice.outcome.clone());
        if !prose.is_empty() {
            why.push((heading, prose));
        }
    }

    // The second question. A command is how somebody starts, and the record knows which
    // slices added one.
    let mut commands = Section::new("how to start", &["command", "when you reach for it"])
        .empty_because(
            "this version added no command — every slice it bound is a view, which changes what \
             you can SEE rather than what you can do",
        );
    for slice in slices.iter().filter(|s| s.command.is_some()) {
        commands.push(vec![
            format!("praxis {}", slice.command.clone().unwrap_or_default()),
            slice.trigger.clone(),
        ]);
    }

    // The third question, and the one a release is usually silent about. A finding that
    // carries a claim is a shortfall somebody wrote down rather than a bug nobody found.
    let mut owed = Section::new("what is known to be missing", &["finding", "against claim"])
        .empty_because("no bound iteration recorded a shortfall — which is worth doubting");
    for attempt in &bound {
        for finding in attempt.findings.iter().filter(|f| f.carries.is_some()) {
            owed.push(vec![
                finding.id.clone(),
                finding.carries.clone().unwrap_or_default(),
            ]);
            if !finding.text.is_empty() {
                why.push((finding.id.clone(), finding.text.clone()));
            }
        }
    }

    // The accounting. Kept, and last: it is what an auditor wants after the reader has gone.
    let mut ledger = Section::new("appendix — what bound to this version", &["iteration", "slice"])
        .empty_because("this version binds nothing");
    for attempt in &bound {
        ledger.push(vec![attempt.id.clone(), attempt.on_slices.join(" · ")]);
    }

    let mut resolved = Section::new("problems this version attacked", &["symptom"])
        .empty_because("no symptom names this version as what resolved it");
    for symptom in corpus.resolved_by(version) {
        resolved.push(vec![symptom.to_owned()]);
    }

    let mut model = ReadModel::new(
        "the-published-set-for-a-release",
        format!("what can you do at {version} that you could not before, and what is still missing?"),
        // The "moment" of an archival result is the version it depicts.
        version,
    );
    model.publishable = true;
    // Order is the argument: what you gained, how to begin, what is missing, what it
    // attacked — then the ledger. A reader who stops after the first section has the answer
    // they came for.
    let mut model = model
        .section(gained)
        .section(commands)
        .section(owed)
        .section(resolved)
        .section(ledger);
    for (name, prose) in why {
        model = model.define(name, prose);
    }
    model
}

/// `what-this-product-means` — the problem this exists for, and what every word means.
///
/// First in the directory, because a reader who does not know what a slice IS cannot use a
/// changelog about slices. Every sentence here was written by hand, for people, and sat in
/// the record unpublished: the frame states its own problem and principle, and the schema
/// says what each kind is in a clause beginning `is=`.
///
/// The glossary is generated from whatever the schema declares, so a kind an adopting
/// project adds appears in ITS release notes without this function being touched.
fn concepts(version: &str, corpus: &Corpus) -> ReadModel {
    // Why, before what. A reader told what a tool does and never why it exists has been
    // handed a manual for a decision they have not made yet.
    let mut why = Section::new("why this exists", &["", ""])
        .empty_because(
            "the record holds no vision or mission, so this product states no reason for \
             existing — only a problem it attacks",
        );
    if !corpus.anchor.vision.is_empty() {
        why.push(vec!["the world if this succeeds".to_owned(), corpus.anchor.vision.clone()]);
    }
    if !corpus.anchor.mission.is_empty() {
        why.push(vec!["what this does about it".to_owned(), corpus.anchor.mission.clone()]);
    }
    if !corpus.anchor.delivered_when.is_empty() {
        why.push(vec![
            "what would count as delivered".to_owned(),
            corpus.anchor.delivered_when.clone(),
        ]);
    }

    let mut problem = Section::new("the problem this exists for", &["", ""])
        .empty_because("the record holds no frame, so this product states no problem");
    if let Some(frame) = corpus.frames.first() {
        problem.push(vec!["what is wrong".to_owned(), frame.title.clone()]);
        if !frame.root_cause.is_empty() {
            problem.push(vec!["why it is wrong".to_owned(), frame.root_cause.clone()]);
        }
        if !frame.principle.is_empty() {
            problem.push(vec!["what follows from that".to_owned(), frame.principle.clone()]);
        }
    }

    let mut symptoms = Section::new("how it shows up", &["", "still present?"])
        .empty_because("the record names no symptom");
    for symptom in &corpus.symptoms {
        symptoms.push(vec![
            symptom.text.clone(),
            match symptom.resolved_by.as_deref() {
                Some(v) => format!("resolved at {v}"),
                None => "yes".to_owned(),
            },
        ]);
    }

    let mut glossary = Section::new("what the words mean", &["term", "meaning"])
        .empty_because("the schema describes no kind, so nothing here can be defined");
    for (kind, means) in &corpus.vocabulary {
        glossary.push(vec![kind.clone(), means.clone()]);
    }

    let mut model = ReadModel::new(
        "what-this-product-means",
        "what problem does this exist for, and what does every word mean?",
        version,
    );
    model.publishable = true;
    model.section(why).section(problem).section(symptoms).section(glossary).define("depicts", version)
}

/// `how-it-fits-together` — what each capability does, refuses, and holds true.
///
/// Published beside what it CAN do: a description omitting what a thing cannot do is the
/// artifact this frame distrusts, and `not` is a field every capability already declares.
fn architecture(version: &str, corpus: &Corpus) -> ReadModel {
    let mut product = Section::new("what it does", &["capability", "does", "refuses to do"])
        .empty_because("no capability declares itself part of the product");
    let mut engine = Section::new("how it is built", &["capability", "does", "refuses to do"])
        .empty_because("no capability describes the engine");
    for capability in &corpus.capabilities {
        let row = vec![capability.id.clone(), capability.doing.clone(), capability.not.clone()];
        if capability.facet == "product" { product.push(row) } else { engine.push(row) }
    }

    // What would be WRONG if two of its facts disagreed. The sharpest thing a capability
    // record says, and the published set has never carried it.
    let mut holds = Section::new("what each keeps true", &["capability", "invariant"])
        .empty_because("no capability declares what it keeps consistent");
    for capability in &corpus.capabilities {
        for invariant in &capability.keeps_consistent {
            holds.push(vec![capability.id.clone(), invariant.clone()]);
        }
    }

    let mut decided = Section::new("decisions behind it", &["decision", "chosen over"])
        .empty_because("no decision was recorded");
    for decision in &corpus.decisions {
        decided.push(vec![decision.title.clone(), decision.over.join(" · ")]);
    }

    let mut model = ReadModel::new(
        "how-it-fits-together",
        "what does each part do, what does it refuse to do, and what does it keep true?",
        version,
    );
    model.publishable = true;
    model.section(product).section(engine).section(holds).section(decided).define("depicts", version)
}

/// `what-this-plugin-ships` — the doctrine a version shipped, what asked for each piece, and
/// which of the plugin's guarantees actually fail closed (`TS.260821.07`).
///
/// Every fact here is DERIVED. Which version shipped a surface is not stored on the surface:
/// it comes from what the surface serves, through the iteration that delivered that slice,
/// to the release the iteration bound to. Storing it would be a second copy of a fact the
/// record already holds, and the first thing that would go stale.
///
/// This replaces `gen-coverage-matrix.sh` and `gen-doctrine-index.sh`, both of which learned
/// what the plugin contains by matching a pattern against the working tree. The fragile
/// thing about that was never the pattern. It was that the prose WAS the state, so the
/// pattern was the only reader, and the only way to find out it had stopped matching was for
/// somebody to notice the output looked wrong.
fn shipped_doctrine(version: &str, corpus: &Corpus) -> ReadModel {
    let mut shipped = Section::new(
        "doctrine shipped",
        &["surface", "kind", "serves", "shipped in"],
    )
    .empty_because("the record declares no doctrine-surface, so what this plugin ships is not \
                    something it can be asked about");
    for surface in corpus.surfaces.iter().filter(|s| !s.retired()) {
        shipped.push(vec![
            surface.path.clone(),
            surface.kind.clone(),
            surface.serves.join(" · "),
            shipped_in(surface, corpus),
        ]);
    }

    let mut retired = Section::new("retired, and where it still stands", &["surface", "stands at"])
        .empty_because("no surface has been retired");
    for surface in corpus.surfaces.iter().filter(|s| s.retired()) {
        retired.push(vec![
            surface.path.clone(),
            surface.stands_at.clone().unwrap_or_else(|| "not recorded".to_owned()),
        ]);
    }

    // What the plugin guarantees, and — the part an adopter actually needs — whether each
    // one is a gate or a notice.
    let mut guarantees =
        Section::new("what it guarantees", &["invariant", "severity", "kept by", "languages"])
            .empty_because("the record declares no invariant");
    for invariant in &corpus.invariants {
        let keepers: Vec<&str> = corpus
            .surfaces
            .iter()
            .filter(|s| !s.retired() && s.serves.iter().any(|t| t == &invariant.id))
            .filter(|s| s.kind == "probe")
            .map(|s| s.path.as_str())
            .collect();
        let coverage = if invariant.structural.is_some() {
            "every language — it reads structure, not text".to_owned()
        } else if invariant.languages.is_empty() {
            "not declared".to_owned()
        } else {
            invariant.languages.join(" · ")
        };
        guarantees.push(vec![
            invariant.id.clone(),
            if invariant.fails_closed() { "fails closed".to_owned() } else { "reports".to_owned() },
            if keepers.is_empty() {
                "nothing — this guarantee has no keeper".to_owned()
            } else {
                keepers.join(" · ")
            },
            coverage,
        ]);
    }

    let mut excluded = Section::new("not covered by this answer", &["question", "ask instead"])
        .empty_because("this answer covers everything the record holds");
    excluded.push(vec![
        "whether a probe WORKS".to_owned(),
        "`praxis prove` — this says a guarantee has a keeper, never that the keeper keeps it"
            .to_owned(),
    ]);
    excluded.push(vec![
        "surfaces whose `shipped in` reads `not derivable`".to_owned(),
        "they serve an invariant, a capability or the frame rather than a slice, and no chain \
         runs from those to a release. Storing a version on the surface would fix the column \
         and break the fact"
            .to_owned(),
    ]);

    let mut model = ReadModel::new(
        "what-this-plugin-ships",
        "what doctrine did this version ship, what asked for each piece, and which guarantees \
         fail closed?",
        version,
    );
    model.publishable = true;
    model
        .section(shipped)
        .section(guarantees)
        .section(retired)
        .section(excluded)
        .define("depicts", version)
}

/// Which release shipped this surface, derived rather than stored.
///
/// `serves` names a slice → some iteration covers that slice → that iteration bound to a
/// release. A surface serving an invariant or the frame has no such chain, and says so
/// instead of guessing: a column that reads "0.8.0" because the tool had nothing better is
/// worse than one that admits it.
fn shipped_in(surface: &crate::surface::Surface, corpus: &Corpus) -> String {
    let mut versions: Vec<String> = Vec::new();
    for target in &surface.serves {
        for attempt in corpus.attempts.iter().filter(|a| a.covers(target)) {
            if let Some(release) = corpus.bound_to(&attempt.id)
                && !versions.contains(&release.version)
            {
                versions.push(release.version.clone());
            }
        }
    }
    if versions.is_empty() {
        "not derivable from what it serves".to_owned()
    } else {
        versions.join(" · ")
    }
}

/// `capabilities-and-what-they-own` — what the system must be able to do, and which events
/// each keeps consistent.
fn capabilities(version: &str, corpus: &Corpus) -> ReadModel {
    // Two sections, not one. A reader who installed the product and a maintainer who builds
    // it are asking different questions of the same list, and answering both with one table
    // is how docs/releases/0.8.0 came to hand a reader the engine's internals as a feature
    // list (TS.260821.06/C3).
    let mut section =
        Section::new("what the product can do", &["capability", "derived from", "events owned"])
            .empty_because(
                "the record names no capability with facet=product — every capability it holds \
                 describes how the tool is built, not what it does for whoever installed it",
            );
    let mut internals =
        Section::new("how it is built", &["capability", "derived from", "events owned"])
            .empty_because("the record names no engine capability");
    for capability in &corpus.capabilities {
        let row = vec![
            capability.id.clone(),
            if capability.from_cluster.is_empty() {
                "not declared".to_owned()
            } else {
                capability.from_cluster.clone()
            },
            capability.owns.len().to_string(),
        ];
        if capability.facet == "product" {
            section.push(row);
        } else {
            internals.push(row);
        }
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
    model.section(section).section(internals).section(owns).define("depicts", version)
}
