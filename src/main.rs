mod config;
mod utils;

use config::default;
use tokio::runtime;
// use std::error;

fn main() {
    env_logger::init_from_env(
        env_logger::Env::new()
        .filter_or("UFS_LOG", "RUST_LOG")
        .write_style_or("UFS_LOG_STYLE", "RUST_LOG_STYLE"),
    );

    // build runtime
    runtime::Builder::new_multi_thread()
    .thread_name(default::SERVER_NAME)
    .enable_all()
    .build()
    .unwrap_or_else(|err| exit!("Cannot create async runtime: {}", err))    // TODO: error handling
    // Execute the future, blocking the current thread until completion
    .block_on(async_main());

}


async fn async_main() {}