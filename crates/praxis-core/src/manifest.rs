//! Harness manifests as projections. `TS.260823.07`: version and description come from the
//! record; everything else about a manifest — skills paths, hooks, per-harness structure —
//! is a fact about how doctrine reaches that harness, and stays hand-authored.
//!
//! A manifest is JSON, so it is patched rather than rendered whole the way a released
//! document is (`TS.260820.10`): re-composing the file from nothing would mean this engine
//! also deciding the structural shape each harness wants, which is not what was stale. What
//! was stale is three facts inside a shape that was already right.

use serde_json::Value;

/// Every manifest this repository ships, relative to the repository root. `.claude-plugin/
/// marketplace.json` is the one exception: the derived fields live inside its `plugins`
/// array rather than at the top, because the top level describes the MARKETPLACE, not
/// praxis.
pub const PATHS: &[&str] = &[
    ".claude-plugin/plugin.json",
    ".claude-plugin/marketplace.json",
    ".codex-plugin/plugin.json",
    ".cursor-plugin/plugin.json",
    "gemini-extension.json",
];

/// What a manifest derives from the record.
#[derive(Debug, Clone, Copy)]
pub struct Projection<'a> {
    pub version: &'a str,
    /// `None` when the record declares no mission — nothing to derive a description FROM,
    /// so the field is left exactly as it stood rather than overwritten with nothing.
    pub description: Option<&'a str>,
}

/// Apply a projection to one manifest's existing JSON, keeping every field the projection
/// does not name.
///
/// # Errors
/// The path names no manifest this engine knows how to derive, or the existing text is not
/// JSON.
pub fn derive(path: &str, existing: &str, projection: Projection<'_>) -> Result<String, String> {
    if !PATHS.contains(&path) {
        return Err(format!("{path} is not a manifest this engine derives"));
    }
    let mut value: Value =
        serde_json::from_str(existing).map_err(|e| format!("{path} is not valid JSON: {e}"))?;

    let target = if path == ".claude-plugin/marketplace.json" {
        value
            .get_mut("plugins")
            .and_then(Value::as_array_mut)
            .and_then(|plugins| {
                plugins.iter_mut().find(|p| p.get("name").and_then(Value::as_str) == Some("praxis"))
            })
            .ok_or_else(|| format!("{path} names no `praxis` entry in `plugins`"))?
    } else {
        &mut value
    };

    let Some(object) = target.as_object_mut() else {
        return Err(format!("{path}'s praxis entry is not a JSON object"));
    };
    object.insert("version".to_owned(), Value::String(projection.version.to_owned()));
    if let Some(description) = projection.description
        && object.contains_key("description")
    {
        object.insert("description".to_owned(), Value::String(description.to_owned()));
    }

    let mut rendered = serde_json::to_string_pretty(&value)
        .map_err(|e| format!("{path} could not be re-rendered: {e}"))?;
    rendered.push('\n');
    Ok(rendered)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_and_description_are_overwritten_and_everything_else_survives() {
        let existing = r#"{
  "name": "praxis",
  "version": "0.7.1",
  "description": "stale prose",
  "skills": "./skills/"
}"#;
        let out = derive(
            ".codex-plugin/plugin.json",
            existing,
            Projection { version: "0.8.0", description: Some("fidelity is computed") },
        )
        .expect("derives");
        let value: Value = serde_json::from_str(&out).expect("valid json");
        assert_eq!(value["version"], "0.8.0");
        assert_eq!(value["description"], "fidelity is computed");
        assert_eq!(value["skills"], "./skills/");
    }

    #[test]
    fn a_missing_description_is_left_untouched() {
        let existing = r#"{"name": "praxis", "version": "0.7.1"}"#;
        let out = derive(
            "gemini-extension.json",
            existing,
            Projection { version: "0.8.0", description: None },
        )
        .expect("derives");
        let value: Value = serde_json::from_str(&out).expect("valid json");
        assert_eq!(value["version"], "0.8.0");
        assert!(value.get("description").is_none());
    }

    #[test]
    fn the_marketplace_manifest_is_patched_inside_its_plugins_entry() {
        let existing = r#"{
  "name": "nuewframe-marketplace",
  "description": "hosts praxis",
  "plugins": [
    {"name": "praxis", "version": "0.7.1", "description": "stale"}
  ]
}"#;
        let out = derive(
            ".claude-plugin/marketplace.json",
            existing,
            Projection { version: "0.8.0", description: Some("fidelity is computed") },
        )
        .expect("derives");
        let value: Value = serde_json::from_str(&out).expect("valid json");
        assert_eq!(value["description"], "hosts praxis", "the MARKETPLACE's own description survives");
        assert_eq!(value["plugins"][0]["version"], "0.8.0");
        assert_eq!(value["plugins"][0]["description"], "fidelity is computed");
    }
}
