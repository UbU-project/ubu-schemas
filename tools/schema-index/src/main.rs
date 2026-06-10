use std::{fs, path::Path};

use anyhow::{anyhow, Context, Result};
use serde_json::{json, Value};
use walkdir::WalkDir;

fn main() -> Result<()> {
    let index = build_index(Path::new("schemas"))?;
    println!("{}", serde_json::to_string_pretty(&index)?);
    Ok(())
}

fn build_index(schemas_dir: &Path) -> Result<Value> {
    let mut entries = Vec::new();

    for entry in WalkDir::new(schemas_dir).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if !entry.file_type().is_file() || !path.to_string_lossy().ends_with(".schema.json") {
            continue;
        }
        let content = fs::read_to_string(path)
            .with_context(|| format!("failed to read schema {}", path.display()))?;
        let schema: Value = serde_json::from_str(&content)
            .with_context(|| format!("failed to parse schema {}", path.display()))?;
        let id = schema
            .get("$id")
            .and_then(Value::as_str)
            .ok_or_else(|| anyhow!("schema missing $id: {}", path.display()))?;
        entries.push(json!({
            "id": id,
            "path": path.to_string_lossy()
        }));
    }

    entries.sort_by(|left, right| left["id"].as_str().cmp(&right["id"].as_str()));
    Ok(json!({ "schemas": entries }))
}
