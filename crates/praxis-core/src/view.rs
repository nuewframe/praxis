//! `read-model@v1`, as a type.
//!
//! The seam's four constraints are the acceptance criteria for this module, not advice:
//!
//! 1. cells carry values, never markup — [`Cell`] is a plain value, and nothing here can
//!    emit presentation, because nothing here knows what will render it;
//! 2. prose is referenced, not inlined — a row names the *condition*, and the sentence a
//!    human wrote about that condition is looked up by name from [`ReadModel::defines`];
//! 3. empty sections declare their own emptiness — [`Section::empty_because`] is required
//!    when a section has no rows, and [`ReadModel::flaws`] refuses one that does not;
//! 4. an archival result names the version it depicts — a working result names the moment
//!    instead, and [`ReadModel::publishable`] says which kind it is.
//!
//! The point of the type is finding E10: a renderer composes a document from this without
//! ever knowing what the view MEANS. If projection ever needs a code path specific to one
//! view, the read-side split is fake — which is the falsifier WALK.260820.01/W2 tested on
//! paper and this module puts under load in code.

/// One value in a row. A value, never markup: the moment a capability may emit
/// presentation, the split is fiction while every test still passes.
pub type Cell = String;

/// One table in a result: a name, its columns, and its rows — or its emptiness, stated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Section {
    pub name: String,
    pub columns: Vec<String>,
    pub rows: Vec<Vec<Cell>>,
    /// Why this section is empty. A reader who cannot tell "nothing is blocked" from
    /// "blocking was not computed" is reading the trust-transfer problem at table
    /// granularity, so an empty section says which it is.
    pub empty_because: Option<String>,
}

impl Section {
    pub fn new(name: impl Into<String>, columns: &[&str]) -> Self {
        Self {
            name: name.into(),
            columns: columns.iter().map(|c| (*c).to_owned()).collect(),
            rows: Vec::new(),
            empty_because: None,
        }
    }

    pub fn row(mut self, cells: Vec<Cell>) -> Self {
        self.rows.push(cells);
        self
    }

    pub fn push(&mut self, cells: Vec<Cell>) {
        self.rows.push(cells);
    }

    /// State why this section would be empty. Set unconditionally at build time: it is
    /// read only when there are no rows, so the author does not have to know in advance.
    pub fn empty_because(mut self, why: impl Into<String>) -> Self {
        self.empty_because = Some(why.into());
        self
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}

/// One view's content, as a typed result. It carries no markup, no ordering preference,
/// and no opinion about how it will be shown.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadModel {
    /// The view's declared name, as the record names it.
    pub view: String,
    /// What was asked.
    pub answers: String,
    /// The moment this was computed. `right now` is in the question, so a result that
    /// does not say when it was computed is not an answer to it.
    pub as_of: String,
    /// Whether this result survives being frozen. A view whose value is being CURRENT is
    /// projected on demand and never committed (ES.260819.01/E17).
    pub publishable: bool,
    pub sections: Vec<Section>,
    /// Prose a human wrote, by the name a row refers to it by. Referenced rather than
    /// inlined: prose a human wrote is data; prose a renderer composed is presentation.
    pub defines: Vec<(String, String)>,
}

impl ReadModel {
    pub fn new(
        view: impl Into<String>,
        answers: impl Into<String>,
        as_of: impl Into<String>,
    ) -> Self {
        Self {
            view: view.into(),
            answers: answers.into(),
            as_of: as_of.into(),
            publishable: false,
            sections: Vec::new(),
            defines: Vec::new(),
        }
    }

    pub fn section(mut self, section: Section) -> Self {
        self.sections.push(section);
        self
    }

    pub fn define(mut self, name: impl Into<String>, prose: impl Into<String>) -> Self {
        self.defines.push((name.into(), prose.into()));
        self
    }

    pub fn definition(&self, name: &str) -> Option<&str> {
        self.defines
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, prose)| prose.as_str())
    }

    /// Where this result breaks its own seam. Returned rather than panicked: a malformed
    /// result is a refusal like any other, and the caller decides what to do with it.
    pub fn flaws(&self) -> Vec<String> {
        let mut out = Vec::new();
        for section in &self.sections {
            if section.is_empty() && section.empty_because.is_none() {
                out.push(format!(
                    "section {:?} is empty and does not say why — a reader cannot tell \
                     `nothing here` from `this was not computed`",
                    section.name
                ));
            }
            for (i, row) in section.rows.iter().enumerate() {
                if row.len() != section.columns.len() {
                    out.push(format!(
                        "section {:?} row {i} has {} cells for {} columns",
                        section.name,
                        row.len(),
                        section.columns.len()
                    ));
                }
                for cell in row {
                    if cell.contains('|') || cell.contains('<') || cell.contains("**") {
                        out.push(format!(
                            "section {:?} row {i} carries markup in a cell: {cell:?}",
                            section.name
                        ));
                    }
                }
            }
        }
        if self.publishable && !self.as_of.is_empty() && self.view.is_empty() {
            out.push("a publishable result must name what it depicts".to_owned());
        }
        out
    }
}
