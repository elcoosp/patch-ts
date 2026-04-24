use anyhow::Result;
use super::types::*;
pub fn handle_mcp() -> Result<()> { crate::mcp::run_mcp()?; Ok(()) }
pub fn handle_mcp_http(args: McpHttpArgs) -> Result<()> {
    let addr: std::net::SocketAddr = format!("{}:{}", args.bind, args.port).parse()?;
    let rt = tokio::runtime::Builder::new_multi_thread().enable_all().build()?;
    rt.block_on(crate::mcp_http::run_http_mcp(addr, args.auth_token))?;
    Ok(())
}
