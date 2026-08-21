//! `praxis-core` — the functional core.
//!
//! It parses KDL, loads the schema the record declares, and decides whether an entity
//! conforms. It reads no files, prints nothing, and holds no opinion the record does
//! not state: ADR.260819.01/A4 forbids the engine encoding what the record declares,
//! so every rule applied here arrives as data from `NA.260820.01/schema`.

pub mod admission;
pub mod binding;
pub mod check;
pub mod cut;
pub mod close;
pub mod pickup;
pub mod publish;
pub mod schema;
pub mod view;

pub use check::{
    Known, Refusal, Severity, Violation, check_corpus, check_document, check_node, index_all,
    refused,
};
pub use admission::{Assessment, Capability, Carried, Config, Release, Condition, Conditions, Corpus, Settled, Verdict, assess, project};
pub use binding::{Binding, Proposal, Rejected, bind, propose, unbind};
pub use cut::{Blocked, Cut, cut, seal};
pub use close::{Closing, Unaccounted, close_iteration};
pub use pickup::{Ask, Pickup, Record, pick_up};
pub use publish::{Document, Publication, publish};
pub use schema::{Cardinality, EntitySpec, FieldSpec, Schema};
pub use view::{Cell, ReadModel, Section};

/// Parse KDL source, returning the document or the parse error unchanged — the codec
/// refuses syntax and nothing else.
pub fn parse(source: &str) -> Result<kdl::KdlDocument, kdl::KdlError> {
    source.parse()
}
