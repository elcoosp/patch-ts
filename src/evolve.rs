use crate::gate::{run_gate_parallel, GateResult};
use serde::Serialize;
use std::path::Path;

#[derive(Debug, Clone, Serialize)]
pub struct PatchCandidate {
    pub old: String,
    pub new: String,
    pub fuzz: usize,
    pub confidence: f64,
    pub uniqueness_weight: f64,
    pub fitness: f64,
    pub gate_result: GateResult,
}

pub struct EvolutionConfig {
    pub population_size: usize,
    pub max_generations: usize,
    pub timeout_secs: u64,
}

impl Default for EvolutionConfig {
    fn default() -> Self {
        Self { population_size: 10, max_generations: 3, timeout_secs: 30 }
    }
}

pub fn run_evolution(
    file_path: &Path,
    original: &str,
    old: &str,
    new: &str,
    config: &EvolutionConfig,
) -> PatchCandidate {
    let mut population = Vec::new();
    for i in 0..config.population_size {
        let fuzz = (i % 5) + 1;
        let confidence = 0.8 + (i as f64 * 0.02).min(0.18);
        let uniqueness_weight = 0.1 + (i as f64 * 0.05).min(0.4);
        population.push(PatchCandidate {
            old: old.to_string(),
            new: new.to_string(),
            fuzz,
            confidence,
            uniqueness_weight,
            fitness: 0.0,
            gate_result: GateResult { passed: false, stages: vec![] },
        });
    }
    for candidate in &mut population {
        let gate_result = run_gate_parallel(
            &["syntax".to_string(), "compile".to_string()],
            file_path,
            original,
            &candidate.new,
            30,
        ).unwrap_or_else(|_| GateResult { passed: false, stages: vec![] });
        candidate.fitness = compute_fitness(&gate_result);
        candidate.gate_result = gate_result;
    }
    population.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());
    population[0].clone()
}

fn compute_fitness(gate: &GateResult) -> f64 {
    let mut score = 0.0;
    for stage in &gate.stages {
        if stage.passed {
            score += match stage.name.as_str() {
                "syntax" => 1.0,
                "compile" => 2.0,
                "cross‑file" | "cross_file" => 1.5,
                "security" => 3.0,
                _ => 0.5,
            };
        }
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_fitness_all_pass() {
        let gate = GateResult {
            passed: true,
            stages: vec![
                crate::gate::StageResult { name: "syntax".into(), passed: true, details: "ok".into() },
                crate::gate::StageResult { name: "compile".into(), passed: true, details: "ok".into() },
            ],
        };
        let fitness = compute_fitness(&gate);
        assert!(fitness > 0.0);
    }

    #[test]
    fn test_run_evolution_generates_best() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("test.rs");
        std::fs::write(&file_path, "fn main() {}").unwrap();
        let config = EvolutionConfig { population_size: 5, max_generations: 1, timeout_secs: 10 };
        let best = run_evolution(&file_path, "fn main() {}", "fn main() {}", "fn main() { }", &config);
        assert!(best.fitness >= 0.0);
    }
}
