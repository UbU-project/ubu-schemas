use std::{collections::HashMap, error::Error, fs, path::Path};

use anyhow::{anyhow, bail, Context, Result};
use jsonschema::{Retrieve, Uri};
use serde_json::Value;
use walkdir::WalkDir;

#[derive(Clone, Debug)]
struct InMemoryRetriever {
    schemas: HashMap<String, Value>,
}

impl Retrieve for InMemoryRetriever {
    fn retrieve(&self, uri: &Uri<String>) -> Result<Value, Box<dyn Error + Send + Sync>> {
        let raw = uri.as_str();
        let base = raw.split('#').next().unwrap_or(raw);
        self.schemas
            .get(raw)
            .or_else(|| self.schemas.get(base))
            .cloned()
            .ok_or_else(|| format!("unregistered schema URI blocked: {raw}").into())
    }
}

fn main() -> Result<()> {
    let repo_root = std::env::current_dir().context("failed to read current directory")?;
    let schemas = load_schemas(&repo_root.join("schemas"))?;
    validate_fixture_tree(&repo_root, &schemas, "valid", true)?;
    validate_fixture_tree(&repo_root, &schemas, "invalid", false)?;
    Ok(())
}

fn load_schemas(schemas_dir: &Path) -> Result<HashMap<String, Value>> {
    let mut schemas = HashMap::new();

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
            .ok_or_else(|| anyhow!("schema missing $id: {}", path.display()))?
            .to_owned();
        if schemas.insert(id.clone(), schema).is_some() {
            bail!("duplicate schema $id: {id}");
        }
    }

    Ok(schemas)
}

fn validate_fixture_tree(
    repo_root: &Path,
    schemas: &HashMap<String, Value>,
    fixture_kind: &str,
    expect_valid: bool,
) -> Result<()> {
    let root = repo_root.join("fixtures").join(fixture_kind);
    if !root.exists() {
        bail!("fixture tree missing: {}", root.display());
    }

    let mut checked = 0usize;
    let retriever = InMemoryRetriever {
        schemas: schemas.clone(),
    };

    for entry in WalkDir::new(&root).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if !entry.file_type().is_file()
            || path.extension().and_then(|ext| ext.to_str()) != Some("json")
        {
            continue;
        }

        checked += 1;
        let schema = schema_for_fixture(repo_root, path, schemas)?;
        let validator = jsonschema::draft202012::options()
            .with_retriever(retriever.clone())
            .should_validate_formats(true)
            .build(schema)
            .with_context(|| format!("failed to build validator for {}", path.display()))?;

        let instance = read_json(path)?;
        let is_valid = validator.is_valid(&instance);

        match (expect_valid, is_valid) {
            (true, false) => {
                let errors = validator
                    .iter_errors(&instance)
                    .map(|error| error.to_string())
                    .collect::<Vec<_>>()
                    .join("; ");
                bail!("valid fixture failed: {}: {errors}", path.display());
            }
            (false, true) => bail!("invalid fixture unexpectedly passed: {}", path.display()),
            _ => {}
        }
    }

    if checked == 0 {
        bail!("no {fixture_kind} fixtures found under {}", root.display());
    }

    println!("validated {checked} {fixture_kind} fixtures");
    Ok(())
}

fn schema_for_fixture<'a>(
    repo_root: &Path,
    fixture_path: &Path,
    schemas: &'a HashMap<String, Value>,
) -> Result<&'a Value> {
    let relative = fixture_path
        .strip_prefix(repo_root.join("fixtures").join("valid"))
        .or_else(|_| fixture_path.strip_prefix(repo_root.join("fixtures").join("invalid")))
        .with_context(|| {
            format!(
                "fixture path outside expected trees: {}",
                fixture_path.display()
            )
        })?;

    let mut components = relative.components();
    let category = components
        .next()
        .ok_or_else(|| anyhow!("fixture path missing category: {}", fixture_path.display()))?
        .as_os_str()
        .to_string_lossy();
    let schema_name = components
        .next()
        .ok_or_else(|| {
            anyhow!(
                "fixture path missing schema folder: {}",
                fixture_path.display()
            )
        })?
        .as_os_str()
        .to_string_lossy();

    let id = format!("https://schemas.ubunow.net/phase1/{category}/{schema_name}.schema.json");
    schemas.get(&id).ok_or_else(|| {
        anyhow!(
            "no schema registered for fixture {}: {id}",
            fixture_path.display()
        )
    })
}

fn read_json(path: &Path) -> Result<Value> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("failed to read fixture {}", path.display()))?;
    serde_json::from_str(&content)
        .with_context(|| format!("failed to parse fixture {}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn retriever_blocks_unregistered_uris() {
        let retriever = InMemoryRetriever {
            schemas: HashMap::from([(
                "https://schemas.ubunow.net/phase1/common/id.schema.json".to_owned(),
                json!({"type": "string"}),
            )]),
        };
        let schema = json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$ref": "https://schemas.ubunow.net/phase1/common/missing.schema.json"
        });

        assert!(jsonschema::draft202012::options()
            .with_retriever(retriever)
            .build(&schema)
            .is_err());
    }
}
