use std::collections::HashMap;
use std::path::Path;
use crate::symbols::SymbolIndex;

pub struct CallSite {
    pub file: String,
    pub line: usize,
    pub column: usize,
}

/// Build a project‑wide symbol index for all source files under `root`,
/// using the existing `symbols::build_index` function.
pub fn build_project_index(root: &Path) -> HashMap<String, SymbolIndex> {
    let mut index = HashMap::new();
    let root = root.to_path_buf();
    for entry in glob::glob(root.join("**").join("*.rs").to_str().unwrap_or("")).unwrap().flatten() {
        if let Ok(content) = std::fs::read_to_string(&entry) {
            if let Some(sym) = crate::symbols::build_index(&content, "rs") {
                index.insert(entry.to_string_lossy().to_string(), sym);
            }
        }
    }
    index
}

/// Find call sites for a given function name across the project index.
pub fn find_callers(func_name: &str, index: &HashMap<String, SymbolIndex>) -> Vec<CallSite> {
    let mut callers = Vec::new();
    for (file, _sym_index) in index {
        // Simple: check if the function name appears in any other file's symbol index
        // For deeper analysis, we'd parse the file and look for call_expression nodes
        if let Some(content) = std::fs::read_to_string(file).ok() {
            // Use a simple regex as a fallback for now
            let re = regex::Regex::new(&format!(r"\b{}\s*\(", regex::escape(func_name))).unwrap();
            for (i, line) in content.lines().enumerate() {
                if re.is_match(line) {
                    callers.push(CallSite {
                        file: file.clone(),
                        line: i + 1,
                        column: 0,
                    });
                }
            }
        }
    }
    callers
}
