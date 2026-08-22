//! Proving the rules. `TS.260821.02`: report any declared rule that nothing in the record
//! demonstrates refusing.
//!
//! **A rule that has never been shown to refuse is indistinguishable from one that
//! cannot.** `ITER.260821.17/AG1` proved it the expensive way: a correct rule reported
//! nothing because its refusals were computed and silently dropped, and it was caught only
//! because the answer had been predicted. Three attribution bugs, each found by a rule
//! firing and never by a test.
//!
//! A witness lives in the **record**, beside the rule it witnesses. That is the whole
//! design: a witness written in Rust would be a second implementation of the same check,
//! and two implementations agreeing proves only that one person wrote both.

use kdl::KdlDocument;

use crate::check::{Facts, Known, check_corpus_given, check_document, index_all};
use crate::schema::Schema;

/// What proving one rule concluded.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Proof {
    /// The witness produced a refusal enforcing this rule. The rule can fire.
    Witnessed { rule: String, refusals: usize },
    /// Declared, and nothing demonstrates it refusing.
    Unwitnessed { rule: String, why: &'static str },
}

impl Proof {
    pub fn rule(&self) -> &str {
        match self {
            Self::Witnessed { rule, .. } | Self::Unwitnessed { rule, .. } => rule,
        }
    }

    pub fn proven(&self) -> bool {
        matches!(self, Self::Witnessed { .. })
    }
}

/// Why a rule has no witness. The three are different problems and read differently.
pub const NO_WITNESS: &str = "the record declares no witness for it";
pub const WITNESS_UNPARSEABLE: &str = "its witness is not a record the codec can read";
pub const WITNESS_REFUSED_NOTHING: &str =
    "its witness produced no refusal enforcing it — either the rule is unimplemented, or its \
     refusals are computed and dropped";

/// Run every declared rule against its witness.
///
/// The witness is checked by the SAME functions that check the tree. A witness that passes
/// through a different path would prove that path works and nothing about the real one —
/// which is exactly how AG1 survived: the refusal was computed correctly and lost on the
/// way out.
pub fn prove(schema: &Schema) -> Vec<Proof> {
    schema
        .rules()
        .map(|rule| {
            let Some(source) = &rule.witness else {
                return Proof::Unwitnessed { rule: rule.name.clone(), why: NO_WITNESS };
            };
            let Ok(witness) = source.parse::<KdlDocument>() else {
                return Proof::Unwitnessed {
                    rule: rule.name.clone(),
                    why: WITNESS_UNPARSEABLE,
                };
            };

            let facts = Facts { shipped: rule.given_shipped.clone() };
            let refusals = refusals_enforcing(&rule.name, &witness, schema, &facts);
            if refusals == 0 {
                Proof::Unwitnessed { rule: rule.name.clone(), why: WITNESS_REFUSED_NOTHING }
            } else {
                Proof::Witnessed { rule: rule.name.clone(), refusals }
            }
        })
        .collect()
}

/// How many refusals enforcing this rule the witness produces, through the real checker.
fn refusals_enforcing(
    rule: &str,
    witness: &KdlDocument,
    schema: &Schema,
    facts: &Facts,
) -> usize {
    let mut known = Known::default();
    index_all(witness, &mut known);

    let mut found = check_document(witness, schema, &known);
    found.extend(check_corpus_given(std::slice::from_ref(witness), schema, facts));
    found.into_iter().filter(|v| v.refusal.rule() == rule).count()
}

/// The rules nothing demonstrates. This is the answer to "which of our gates has never
/// fired", which today is a question nobody can answer at all.
pub fn unwitnessed(schema: &Schema) -> Vec<Proof> {
    prove(schema).into_iter().filter(|p| !p.proven()).collect()
}
