fn main() -> anyhow::Result<()> {
    env_logger::init();
    patch_ts::cli::run()
}
