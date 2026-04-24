use clap::Parser;
pub mod types;
pub mod patch; pub mod balance; pub mod explain; pub mod fix; pub mod git; pub mod semdiff;
pub mod provenance; pub mod gate; pub mod score; pub mod index; pub mod evolve; pub mod review;
pub mod attest; pub mod generate_tests; pub mod verify; pub mod watch; pub mod mcp; pub mod undo;
pub mod lsp; pub mod adapt; pub mod trace_verify; pub mod heredoc; pub mod impact; pub mod entity;
pub mod key; pub mod heal; pub mod recall;

use anyhow::Result;
use types::*;

pub fn run() -> Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Command::Patch(args) => {
            if let Some(ref file) = args.file { crate::validate::validate_path(file, args.allow_all_paths)?; }
            if let Some(ref old) = args.old { crate::validate::validate_string(old, "--old")?; }
            if let Some(ref new) = args.new { crate::validate::validate_string(new, "--new")?; }
            if let Some(ref expect) = args.expect { crate::validate::validate_string(expect, "--expect")?; }
            if let Some(ref content) = args.content { crate::validate::validate_string(content, "--content")?; }
        }
        Command::Balance(args) => {
            if let Some(ref file) = args.file { crate::validate::validate_path(file, args.allow_all_paths)?; }
        }
        _ => {}
    }
    let result = match cli.command {
        Command::Patch(args) => patch::handle_patch(args),
        Command::Balance(args) => balance::handle_balance(args),
        Command::Explain(args) => explain::handle_explain(args),
        Command::Fix(args) => fix::handle_fix(args),
        Command::Git(git_args) => git::handle_git(git_args),
        Command::SemDiff(args) => semdiff::handle_sem_diff(args),
        Command::Provenance(args) => provenance::handle_provenance_query(args),
        Command::Gate(args) => gate::handle_gate(args),
        Command::Score(args) => score::handle_score(args),
        Command::Index(args) => index::handle_index(args),
        Command::Evolve(args) => evolve::handle_evolve(args),
        Command::Review(args) => review::handle_review(args),
        Command::Attest(args) => attest::handle_attest(args),
        Command::GenerateTests(args) => generate_tests::handle_generate_tests(args),
        Command::Verify(args) => verify::handle_verify(args),
        Command::TraceVerify(args) => trace_verify::handle_trace_verify(args),
        Command::Watch(args) => watch::handle_watch(args),
        Command::Impact(args) => impact::handle_impact(args),
        Command::Entity(args) => entity::handle_entity(args),
        Command::Key(args) => key::handle_key(args),
        Command::Heal(args) => heal::handle_heal(args),
        Command::Recall(args) => recall::handle_recall(args),
        Command::Undo => undo::handle_undo(),
        Command::Redo => undo::handle_redo(),
        Command::History => undo::handle_history(),
        Command::Lsp => lsp::handle_lsp(),
        Command::Mcp => mcp::handle_mcp(),
        Command::McpHttp(args) => mcp::handle_mcp_http(args),
        Command::AdaptThreshold => adapt::handle_adapt_threshold(),
        Command::AdaptStrategy => adapt::handle_adapt_strategy(),
    };
    if let Err(ref e) = result { eprintln!("{}", e); std::process::exit(1); }
    Ok(())
}
