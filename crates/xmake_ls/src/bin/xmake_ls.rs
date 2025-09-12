use clap::Parser;
use mimalloc::MiMalloc;
use std::error::Error;
use xmake_ls::cmd_args::CmdArgs;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    let cmd_args = CmdArgs::parse();
    xmake_ls::run_ls(cmd_args).await
}
