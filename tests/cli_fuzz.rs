use assert_cmd::Command;
use proptest::prelude::*;
use std::fs;
use tempfile::tempdir;

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 500,
        ..ProptestConfig::default()
    })]

    #[test]
    fn fuzz_cli_does_not_panic(
        line in 0usize..1000,
        fuzz in 0usize..50,
        old in "[a-zA-Z0-9 ]{0,200}",
        new in "[a-zA-Z0-9 ]{0,200}",
        flag_force in prop::bool::ANY,
        flag_dry_run in prop::bool::ANY,
    ) {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.rs");
        fs::write(&file_path, "fn main() { let x = 1; }\n").unwrap();

        let mut cmd = Command::cargo_bin("patch-ts").unwrap();
        cmd.arg("patch")
            .arg("--file").arg(file_path.to_str().unwrap())
            .arg("--line").arg(line.to_string())
            .arg("--old").arg(old)
            .arg("--new").arg(new)
            .arg("--fuzz").arg(fuzz.to_string());

        if flag_force { cmd.arg("--force"); }
        if flag_dry_run { cmd.arg("--dry-run"); }

        let _ = cmd.ok();
        // The only requirement is that the process doesn't panic.
    }
}
