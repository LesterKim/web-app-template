use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct EvalsConfig {
    name: String,
    threshold: f64,
    metric: String,
}

#[derive(Debug, Deserialize)]
struct EvalCase {
    input: String,
    expected: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let config_path = manifest_dir.join("evals.toml");
    let cases_path = manifest_dir.join("fixtures").join("sample_cases.jsonl");

    let config_text = fs::read_to_string(config_path)?;
    let config: EvalsConfig = toml::from_str(&config_text)?;

    let cases_text = fs::read_to_string(cases_path)?;
    let mut cases = Vec::new();
    for line in cases_text.lines().filter(|line| !line.trim().is_empty()) {
        let case: EvalCase = serde_json::from_str(line)?;
        cases.push(case);
    }

    println!(
        "Loaded {} cases for {} (metric: {}, threshold: {})",
        cases.len(),
        config.name,
        config.metric,
        config.threshold
    );

    Ok(())
}
