use anyhow::Result;
use super::types::*;
pub fn handle_mcp() -> Result<()> { crate::mcp::run_mcp()?; Ok(()) }
pub fn handle_mcp_http(args: McpHttpArgs) -> Result<()> {
    let addr: std::net::SocketAddr = format!("{}:{}", args.bind, args.port).parse()?;
    let rt = tokio::runtime::Builder::new_multi_thread().enable_all().build()?;
    rt.block_on(crate::mcp_http::run_http_mcp(addr, args.auth_token))?;
    Ok(())
}

use super::types::McpGatewayArgs;

pub fn handle_mcp_gateway(args: McpGatewayArgs) -> Result<()> {
    let policy = crate::mcp_gateway::load_policy(std::path::Path::new(&args.policy))?;
    eprintln!("MCP Gateway started with policy from {}", args.policy);
    eprintln!("Allowed tools: {:?}", policy.policy.allowed_tools);
    // For now, we'll just start the MCP HTTP server and let the gateway wrap it.
    // In a full implementation, we'd spawn the HTTP server with a policy‑enforcing handler.
    // As a stub, we forward to the existing mcp_http handler with a warning.
    eprintln!("Warning: MCP Gateway stub – running without full policy enforcement.");
    let addr: std::net::SocketAddr = format!("127.0.0.1:{}", args.port).parse()?;
    let rt = tokio::runtime::Builder::new_multi_thread().enable_all().build()?;
    rt.block_on(crate::mcp_http::run_http_mcp(addr, args.auth_token))?;
    Ok(())
}
