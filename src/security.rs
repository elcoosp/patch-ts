use regex::Regex;
use once_cell::sync::Lazy;

#[derive(Debug, Clone)]
pub struct SecurityPattern {
    pub category: OwaspThreat,
    pub pattern: Regex,
    pub description: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OwaspThreat {
    ToolPoisoning,
    PromptInjection,
    SupplyChain,
}

impl std::fmt::Display for OwaspThreat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OwaspThreat::ToolPoisoning => write!(f, "tool-poisoning"),
            OwaspThreat::PromptInjection => write!(f, "prompt-injection"),
            OwaspThreat::SupplyChain => write!(f, "supply-chain"),
        }
    }
}

pub static OWASP_PATTERNS: Lazy<Vec<SecurityPattern>> = Lazy::new(|| {
    vec![
        SecurityPattern {
            category: OwaspThreat::ToolPoisoning,
            pattern: Regex::new(r"(?i)ignore_above").unwrap(),
            description: "Hidden instruction marker 'ignore_above'",
        },
        SecurityPattern {
            category: OwaspThreat::ToolPoisoning,
            pattern: Regex::new(r"(?i)noqa").unwrap(),
            description: "Hidden instruction marker 'noqa'",
        },
        SecurityPattern {
            category: OwaspThreat::ToolPoisoning,
            pattern: Regex::new(r"# pragma:").unwrap(),
            description: "Hidden instruction marker '# pragma:'",
        },
        SecurityPattern {
            category: OwaspThreat::ToolPoisoning,
            pattern: Regex::new(r"\beval\s*\(").unwrap(),
            description: "Dynamic code execution via eval()",
        },
        SecurityPattern {
            category: OwaspThreat::ToolPoisoning,
            pattern: Regex::new(r"\bexec\s*\(").unwrap(),
            description: "Dynamic code execution via exec()",
        },
        SecurityPattern {
            category: OwaspThreat::PromptInjection,
            pattern: Regex::new(r"(?i)system:").unwrap(),
            description: "Prompt injection keyword 'system:'",
        },
        SecurityPattern {
            category: OwaspThreat::PromptInjection,
            pattern: Regex::new(r"(?i)instructions:").unwrap(),
            description: "Prompt injection keyword 'instructions:'",
        },
        SecurityPattern {
            category: OwaspThreat::PromptInjection,
            pattern: Regex::new(r"(?i)ignore previous").unwrap(),
            description: "Prompt injection phrase 'ignore previous'",
        },
        SecurityPattern {
            category: OwaspThreat::PromptInjection,
            pattern: Regex::new(r"(?i)new instruction").unwrap(),
            description: "Prompt injection phrase 'new instruction'",
        },
        SecurityPattern {
            category: OwaspThreat::PromptInjection,
            pattern: Regex::new(r"\u202E").unwrap(),
            description: "Unicode bidirectional character (Trojan Source attack)",
        },
        SecurityPattern {
            category: OwaspThreat::SupplyChain,
            pattern: Regex::new(r"^\s*use\s+\w+::").unwrap(),
            description: "New Rust import statement added",
        },
        SecurityPattern {
            category: OwaspThreat::SupplyChain,
            pattern: Regex::new(r"^\s*import\s+").unwrap(),
            description: "New Python/JS import statement added",
        },
        SecurityPattern {
            category: OwaspThreat::SupplyChain,
            pattern: Regex::new(r"^\s*require\s*\(").unwrap(),
            description: "New Node.js require() statement added",
        },
    ]
});

/// Scan a piece of text for OWASP security threats.
/// Returns cloned SecurityPatterns to avoid lifetime issues.
pub fn scan_for_threats(content: &str, categories: &[OwaspThreat]) -> Vec<SecurityPattern> {
    let patterns = OWASP_PATTERNS.iter().filter(|p| categories.contains(&p.category));
    let mut findings = Vec::new();
    for pattern in patterns {
        if pattern.pattern.is_match(content) {
            findings.push(pattern.clone());
        }
    }
    findings
}
