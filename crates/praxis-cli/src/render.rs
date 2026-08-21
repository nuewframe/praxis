//! The renderer. It composes a document from a `read-model@v1` result and knows nothing
//! about what any view MEANS.
//!
//! This module is finding E10's falsifier, held under load: if projection ever needs a
//! code path specific to one view, the read-side split is fake. Its whole input is
//! `&ReadModel` — it cannot reach a capability's state even if it wanted to, and there is
//! no branch here on a view's name. The day one appears, the seam is fiction.

use praxis_core::ReadModel;

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
