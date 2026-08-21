//! `praxis-core` — the functional core.
//!
//! It parses KDL, loads the schema the record declares, and decides whether an entity
//! conforms. It reads no files, prints nothing, and holds no opinion the record does
//! not state: ADR.260819.01/A4 forbids the engine encoding what the record declares,
//! so every rule applied here arrives as data from `NA.260820.01/schema`.

pub mod check;
pub mod schema;

pub use check::{
    Known, Refusal, Severity, Violation, check_corpus, check_document, check_node, index_all,
    refused,
};
pub use schema::{Cardinality, EntitySpec, FieldSpec, Schema};

/// Parse KDL source, returning the document or the parse error unchanged — the codec
/// refuses syntax and nothing else.
pub fn parse(source: &str) -> Result<kdl::KdlDocument, kdl::KdlError> {
    source.parse()
}
