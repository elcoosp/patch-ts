use anyhow::Result;
use super::types::*;
use crate::scitt::generate_keypair;

pub fn handle_key(args: KeyArgs) -> Result<()> {
    match args.action {
        KeyAction::Generate { output } => {
            let (signing_key, verifying_key) = generate_keypair();
            let keypair = serde_json::json!({"signing_key": hex::encode(signing_key.to_bytes()), "verifying_key": hex::encode(verifying_key.to_bytes())});
            std::fs::write(&output, serde_json::to_string_pretty(&keypair)?)?;
            println!("Keypair written to {}", output);
        }
        KeyAction::Rotate { key: _ } => { anyhow::bail!("Key rotation not yet implemented"); }
    }
    Ok(())
}
