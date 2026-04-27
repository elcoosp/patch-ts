use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "patch-ts", about = "Tree-sitter-aware patching tool for LLM agents")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Patch(PatchArgs),
    Balance(BalanceArgs),
    Explain(ExplainArgs),
    Fix(FixArgs),
    Git(GitArgs),
    SemDiff(SemDiffArgs),
    Provenance(ProvenanceQueryArgs),
    Gate(GateArgs),
    Score(ScoreArgs),
    Index(IndexArgs),
    Evolve(EvolveArgs),
    Review(ReviewArgs),
    Attest(AttestArgs),
    GenerateTests(GenerateTestsArgs),
    Verify(VerifyArgs),
    TraceVerify(TraceArgs),
    Watch(WatchArgs),
    Impact(ImpactArgs),
    Entity(EntityArgs),
    Key(KeyArgs),
    Heal(HealArgs),
    Recall(RecallArgs),
    Mcp,
    McpHttp(McpHttpArgs),
    McpGateway(McpGatewayArgs),
    Undo,
    Redo,
    History,
    Lsp,
    AdaptThreshold,
    AdaptStrategy,
}

#[derive(Parser, Debug)]
pub struct AttestArgs {
    #[arg(long)] pub since: String,
    #[arg(long)] pub output: Option<String>,
    #[arg(long)] pub json: bool,
    #[arg(long)] pub cra_report: bool,
}

#[derive(Parser, Debug)]
pub struct McpGatewayArgs {
    #[arg(long, default_value = "9090")]
    pub port: u16,
    #[arg(long, default_value = "policy.toml")]
    pub policy: String,
    #[arg(long)]
    pub auth_token: Option<String>,
}

#[derive(Parser, Debug)]
pub struct GenerateTestsArgs {
    #[arg(short, long)] pub file: String,
    #[arg(long)] pub old: String,
    #[arg(long)] pub new: String,
    #[arg(long)] pub output: Option<String>,
    #[arg(long)] pub json: bool,
}

#[derive(Parser, Debug)]
pub struct VerifyArgs {
    #[arg(short, long)] pub file: String,
    #[arg(long)] pub json: bool,
}

#[derive(Parser, Debug)]
pub struct ReviewArgs {
    #[arg(short, long)] pub file: String,
    #[arg(long)] pub old: String,
    #[arg(long)] pub new: String,
    #[arg(long)] pub tui: bool,
    #[arg(long)] pub dashboard: bool,
    #[arg(long)] pub json: bool,
}

#[derive(Parser, Debug)]
pub struct EvolveArgs {
    #[arg(short, long)] pub file: String,
    #[arg(long)] pub old: String,
    #[arg(long)] pub new: String,
    #[arg(long, default_value = "10")] pub population_size: usize,
    #[arg(long, default_value = "30")] pub evolve_timeout: u64,
    #[arg(long)] pub apply: bool,
    #[arg(long)] pub json: bool,
}

#[derive(Parser, Debug)]
pub struct IndexArgs {
    #[arg(long)] pub callers: Option<String>,
    #[arg(long)] pub rebuild: bool,
    #[arg(long)] pub json: bool,
}

#[derive(Parser, Debug)]
pub struct ScoreArgs {
    #[arg(short, long)] pub file: String,
    #[arg(long)] pub old: String,
    #[arg(long)] pub new: String,
    #[arg(long)] pub confidence: Option<f64>,
    #[arg(long)] pub uniqueness_score: Option<f64>,
    #[arg(long)] pub cross_file_impact: Option<usize>,
    #[arg(long)] pub json: bool,
}

#[derive(Parser, Debug)]
pub struct GateArgs {
    #[arg(long, default_value = "syntax,compile")] pub stages: String,
    #[arg(short, long)] pub file: String,
    #[arg(long)] pub json: bool,
    #[arg(long, default_value = "30")] pub compile_timeout: u64,
    #[arg(long)] pub parallel: bool,
    #[arg(long)] pub markdown: bool,
    #[arg(long)] pub lsp_command: Option<String>,
    #[arg(long)] pub lsp_args: Option<String>,
}

#[derive(Parser, Debug)]
pub struct GitArgs { #[command(subcommand)] pub action: GitAction }

#[derive(Subcommand, Debug)]
pub enum GitAction { Apply(GitApplyArgs), Diff(GitDiffArgs) }

#[derive(Parser, Debug)]
pub struct GitApplyArgs {
    pub commit: String,
    #[arg(long)] pub file: Option<String>,
    #[arg(long)] pub dry_run: bool,
    #[arg(long, default_value = "5")] pub fuzz: usize,
    #[arg(long)] pub force: bool,
    #[arg(long)] pub no_backup: bool,
    #[arg(long)] pub json: bool,
}

#[derive(Parser, Debug)]
pub struct GitDiffArgs {
    pub target: String,
    #[arg(long)] pub file: Option<String>,
    #[arg(long)] pub apply: bool,
    #[arg(long)] pub dry_run: bool,
    #[arg(long, default_value = "5")] pub fuzz: usize,
    #[arg(long)] pub force: bool,
    #[arg(long)] pub no_backup: bool,
    #[arg(long)] pub json: bool,
}

#[derive(Parser, Debug)]
pub struct SemDiffArgs {
    #[arg(long)] pub file: Option<String>,
    #[arg(long)] pub old: String,
    #[arg(long)] pub new: String,
    #[arg(long)] pub json: bool,
    #[arg(long)] pub tui: bool,
}

#[derive(Parser, Debug)]
pub struct ProvenanceQueryArgs {
    #[arg(long)] pub since: Option<String>,
    #[arg(long)] pub file: Option<String>,
    #[arg(long)] pub output: Option<String>,
    #[arg(long)] pub json: bool,
}

#[derive(Parser, Debug)]
pub struct TraceArgs {
    #[arg(long)] pub since: Option<String>,
    #[arg(long)] pub key: Option<String>,
    #[arg(long)] pub json: bool,
}

#[derive(Parser, Debug)]
pub struct McpHttpArgs {
    #[arg(long, default_value = "9090")] pub port: u16,
    #[arg(long, default_value = "127.0.0.1")] pub bind: String,
    #[arg(long)] pub auth_token: Option<String>,
}

#[derive(Parser, Debug)]
pub struct ImpactArgs {
    #[arg(long)] pub symbol: String,
    #[arg(long)] pub recursive: bool,
    #[arg(long)] pub json: bool,
}

#[derive(Parser, Debug)]
pub struct EntityArgs { #[command(subcommand)] pub action: EntityAction }

#[derive(Subcommand, Debug)]
pub enum EntityAction {
    List { #[arg(short, long)] file: String, #[arg(long)] json: bool },
    Show { #[arg(long)] symbol: String, #[arg(short, long)] file: String },
    Replace { #[arg(long)] symbol: String, #[arg(short, long)] file: String, #[arg(long)] new: String },
    Body { #[arg(long)] symbol: String, #[arg(short, long)] file: String, #[arg(long)] new: String },
}

#[derive(Parser, Debug)]
pub struct KeyArgs { #[command(subcommand)] pub action: KeyAction }

#[derive(Subcommand, Debug)]
pub enum KeyAction {
    Generate { #[arg(long, default_value = "keypair.json")] output: String },
    Rotate { #[arg(long)] key: Option<String> },
}

#[derive(Parser, Debug)]
pub struct HealArgs {
    #[arg(short, long)] pub file: String,
    #[arg(long)] pub apply: bool,
    #[arg(long, default_value = "10")] pub max_cost: usize,
    #[arg(long, default_value = "language-aware")] pub heuristic: String,
    #[arg(long)] pub json: bool,
}

#[derive(Parser, Debug)]
pub struct RecallArgs {
    #[arg(short, long)] pub file: String,
    #[arg(short, long)] pub line: usize,
    #[arg(long)] pub old: String,
    #[arg(long)] pub new: String,
    #[arg(long)] pub error_code: String,
    #[arg(long)] pub error_message: Option<String>,
    #[arg(long, default_value = "5")] pub context_lines: usize,
    #[arg(long)] pub json: bool,
    #[arg(long)] pub prompt: bool,
    #[arg(long)] pub entropy: bool,
    #[arg(long, default_value = "2.5")] pub entropy_threshold: f64,
    #[arg(long)] pub pre_fetch: bool,
    #[arg(long)] pub minimal: bool,
    #[arg(long)] pub session: Option<String>,
    #[arg(long)] pub max_tokens: Option<usize>,
}

#[derive(Parser, Debug)]
pub struct PatchArgs {
    #[arg(short, long, required_unless_present = "files")] pub file: Option<String>,
    #[arg(long, conflicts_with = "file")] pub files: Option<String>,
    #[arg(short, long, required_unless_present_any = ["diff","delete","after","marker","url","git_commit", "symbol"])] pub line: Option<usize>,
    #[arg(short = 'z', long, default_value = "5")] pub fuzz: usize,
    #[arg(long)] pub old: Option<String>,
    #[arg(long)] pub new: Option<String>,
    #[arg(long, default_value = "0.9")] pub confidence: f64,
    #[arg(long)] pub fix_indent: bool,
    #[arg(long, conflicts_with = "line")] pub diff: bool,
    #[arg(long)]
    pub fix_headers: bool,
    #[arg(long, conflicts_with_all = ["line","diff"])] pub delete: Option<usize>,
    #[arg(long, requires = "delete")] pub expect: Option<String>,
    #[arg(long, conflicts_with_all = ["line","diff","delete"])] pub after: Option<usize>,
    #[arg(long, requires = "after")] pub content: Option<String>,
    #[arg(long)] pub dry_run: bool,
    #[arg(long)] pub force: bool,
    #[arg(long)] pub no_backup: bool,
    #[arg(long)] pub json: bool,
    #[arg(long)] pub no_auto_repair: bool,
    #[arg(long, conflicts_with = "line")] pub marker: Option<String>,
    #[arg(long)] pub serial: bool,
    #[arg(long)] pub plugin: Option<String>,
    #[arg(long)] pub allow_all_paths: bool,
    #[arg(long, conflicts_with_all = ["line","diff","delete","after","marker"])] pub symbol: Option<String>,
    #[arg(long, requires = "symbol")] pub entity_body: bool,
    #[arg(long, conflicts_with_all = ["line","diff","delete","after","marker"])] pub url: Option<String>,
    #[arg(long, conflicts_with_all = ["line","diff","delete","after","marker"])] pub git_commit: Option<String>,
    #[arg(long)] pub no_strip_fence: bool,
    #[arg(long)] pub no_compile_check: bool,
    #[arg(long, default_value = "30")] pub compile_timeout: u64,
    #[arg(long)] pub no_sanitize: bool,
    #[arg(long)] pub no_ellipsis: bool,
    #[arg(long, default_value = "0.2")] pub uniqueness_weight: f64,
    #[arg(long)] pub strict_whitespace: bool,
    #[arg(long)] pub cross_file: bool,
    #[arg(long)] pub agent: Option<String>,
    #[arg(long)] pub model: Option<String>,
    #[arg(long)] pub no_provenance: bool,
    #[arg(long)] pub validate_first: bool,
    #[arg(long)]
    pub verify: bool,
    #[arg(long)]
    pub verify_test: Option<String>,
}

#[derive(Parser, Debug)]
pub struct BalanceArgs {
    #[arg(short, long, required_unless_present = "files")] pub file: Option<String>,
    #[arg(long, conflicts_with = "file")] pub files: Option<String>,
    #[arg(long)] pub function: Option<String>,
    #[arg(long)] pub apply: bool,
    #[arg(long)] pub no_backup: bool,
    #[arg(long, default_value = "10")] pub max_cost: usize,
    #[arg(long)] pub json: bool,
    #[arg(long)] pub serial: bool,
    #[arg(long)] pub plugin: Option<String>,
    #[arg(long)] pub allow_all_paths: bool,
}

#[derive(Parser, Debug)]
pub struct FixArgs {
    #[arg(long)] pub error_file: Option<String>,
    #[arg(long)] pub apply: bool,
    #[arg(long)] pub force: bool,
    #[arg(long)] pub file: Option<String>,
    #[arg(long)] pub json: bool,
}

#[derive(Parser, Debug)]
pub struct WatchArgs {
    #[arg(short, long)] pub path: String,
    #[arg(long, default_value = "500")] pub delay: u64,
    #[arg(long)] pub ignore: Option<Vec<String>>,
    #[arg(long)] pub hooks: Option<String>,
}

#[derive(Parser, Debug)]
pub struct ExplainArgs {
    #[arg(short, long)] pub file: String,
    #[arg(short, long)] pub line: usize,
    #[arg(long)] pub json: bool,
    #[arg(long, default_value = "dark")] pub theme: Option<String>,
}
