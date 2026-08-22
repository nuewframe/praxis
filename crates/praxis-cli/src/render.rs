//! The renderer. It composes a document from a `read-model@v1` result and knows nothing
//! about what any view MEANS.
//!
//! This module is finding E10's falsifier, held under load: if projection ever needs a
//! code path specific to one view, the read-side split is fake. Its whole input is
//! `&ReadModel` — it cannot reach a capability's state even if it wanted to, and there is
//! no branch here on a view's name. The day one appears, the seam is fiction.

use praxis_core::ReadModel;

/// The same result, as Markdown for an archival document. A SECOND renderer over the same
/// type, which is the sharper form of E10's falsifier: two renderers, one result type, and
/// still no branch on what any view means.
pub fn render_markdown(model: &ReadModel, stamp: &str) -> String {
    let mut out = String::new();
    out.push_str(&format!("# {}\n\n", title_case(&model.view)));
    out.push_str(&format!("{}\n\n", model.answers));
    // A preview is never mistakable for a record. The header says which it is, and it says
    // it before anything a reader might quote (TS.260820.08/C1).
    if !model.publishable {
        out.push_str("> **PREVIEW — not a record.** Composed on demand from the record as it\n");
        out.push_str("> stands, for a version that does not exist yet. Nothing was written to\n");
        out.push_str("> produce it, and nothing should be written from it.\n\n");
    }
    // An archival result names the VERSION it depicts, and deliberately not the moment it
    // was generated. A wall-clock stamp would make every re-render differ from the last,
    // and verification of a published tree is a comparison (TS.260820.11). The working
    // renderer stamps the moment for the opposite reason: `right now` is its whole point.
    if model.publishable {
        out.push_str(&format!("> Depicts **{stamp}**, and nothing else. Regenerated whole from\n"));
        out.push_str("> the record; never edited in place.\n\n");
    } else {
        out.push_str(&format!("> As of {}. Ask again for a different answer.\n\n", model.as_of));
    }

    for section in &model.sections {
        out.push_str(&format!("## {}\n\n", section.name));
        if section.is_empty() {
            let why = section.empty_because.as_deref().unwrap_or("(no reason given)");
            out.push_str(&format!("_Nothing here — {why}._\n\n"));
            continue;
        }
        out.push_str(&format!("| {} |\n", section.columns.join(" | ")));
        out.push_str(&format!(
            "| {} |\n",
            section.columns.iter().map(|_| "---").collect::<Vec<_>>().join(" | ")
        ));
        for row in &section.rows {
            out.push_str(&format!(
                "| {} |\n",
                row.iter().map(|c| squash(c)).collect::<Vec<_>>().join(" | ")
            ));
        }
        out.push('\n');
    }

    // Prose a human wrote, printed where it was referenced from. Paragraph breaks in the
    // source are kept: a capability's usage is several invocations, and squashing them into
    // one line makes a reference page unreadable — which is `TS.260821.12` in miniature.
    if !model.defines.is_empty() {
        out.push_str("## In detail\n\n");
        for (name, prose) in &model.defines {
            out.push_str(&format!("### {name}\n\n"));
            for paragraph in prose.split("\n\n") {
                out.push_str(&format!("{}\n\n", squash(paragraph)));
            }
        }
    }
    out
}

fn title_case(slug: &str) -> String {
    let words: Vec<String> = slug.split('-').map(str::to_owned).collect();
    let mut out = words.join(" ");
    if let Some(first) = out.get_mut(0..1) {
        first.make_ascii_uppercase();
    }
    out
}

/// A composed document. Several results, in declared order, with one heading each — and
/// still no branch on what any of them MEANS. Composition is ordering, not interpretation.
pub fn render_composed(document: &praxis_core::Composed) -> String {
    let mut out = String::new();
    out.push_str(&format!("{}\n", document.title));
    out.push_str(&format!("as of {} · composed on demand, never committed\n", document.as_of));
    for part in &document.parts {
        out.push_str(&format!("\n{}\n{}\n", "═".repeat(60), part.owner));
        out.push_str(&render(&part.model));
    }
    out
}

pub fn render(model: &ReadModel) -> String {
    let mut out = String::new();
    out.push_str(&format!("{}\n", model.view));
    out.push_str(&format!("{}\n", model.answers));
    out.push_str(&format!(
        "as of {} · {}\n",
        model.as_of,
        if model.publishable { "publishable" } else { "computed on demand, never committed" }
    ));

    for section in &model.sections {
        out.push_str(&format!("\n── {} ──\n", section.name));
        if section.is_empty() {
            let why = section.empty_because.as_deref().unwrap_or("(no reason given)");
            out.push_str(&format!("   nothing here — {why}\n"));
            continue;
        }
        let widths = widths(&section.columns, &section.rows);
        out.push_str(&format!("   {}\n", line(&section.columns, &widths)));
        out.push_str(&format!("   {}\n", rule(&widths)));
        for row in &section.rows {
            out.push_str(&format!("   {}\n", line(row, &widths)));
        }
    }

    // Prose a human wrote, printed where it was referenced from rather than repeated in
    // every row that points at it.
    if !model.defines.is_empty() {
        out.push_str("\n── what these conditions mean ──\n");
        for (name, prose) in &model.defines {
            out.push_str(&format!("   {name}\n"));
            for chunk in wrap(&squash(prose), 84) {
                out.push_str(&format!("      {chunk}\n"));
            }
        }
    }

    out
}

fn widths(columns: &[String], rows: &[Vec<String>]) -> Vec<usize> {
    let mut w: Vec<usize> = columns.iter().map(|c| c.chars().count()).collect();
    for row in rows {
        for (i, cell) in row.iter().enumerate() {
            if i < w.len() {
                w[i] = w[i].max(squash(cell).chars().count().min(64));
            }
        }
    }
    w
}

fn line(cells: &[String], widths: &[usize]) -> String {
    cells
        .iter()
        .enumerate()
        .map(|(i, c)| {
            let text = truncate(&squash(c), 64);
            let pad = widths.get(i).copied().unwrap_or(0);
            if i + 1 == cells.len() { text } else { format!("{text:<pad$}") }
        })
        .collect::<Vec<_>>()
        .join("  ")
}

fn rule(widths: &[usize]) -> String {
    widths.iter().map(|w| "─".repeat(*w)).collect::<Vec<_>>().join("  ")
}

/// Multi-line KDL strings arrive with their authored line breaks. A cell is one value.
fn squash(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn truncate(text: &str, max: usize) -> String {
    if text.chars().count() <= max {
        return text.to_owned();
    }
    let kept: String = text.chars().take(max.saturating_sub(1)).collect();
    format!("{kept}…")
}

fn wrap(text: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();
    for word in text.split_whitespace() {
        if !current.is_empty() && current.chars().count() + 1 + word.chars().count() > width {
            lines.push(std::mem::take(&mut current));
        }
        if !current.is_empty() {
            current.push(' ');
        }
        current.push_str(word);
    }
    if !current.is_empty() {
        lines.push(current);
    }
    lines
}
