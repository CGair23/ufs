mod utils;
mod cli;
mod handlers;
mod server;

use server::Server;
use config::RuntimeConfig;

use cli::{Opt, Command, UploadOpt};
use structopt::StructOpt;
use anyhow::Result;
use tokio::io::AsyncReadExt;
use tokio::{runtime, fs};
use hyper::{Request, Body, Client};
use hyper::header::CONTENT_TYPE;

use std::io::Write;
use std::path::Path;

const DEFAULT_CONF_PATH: &str = "./config/default.toml";
const PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");
const BOUNDARY: &'static str = "------------------------ea3bbcf87c101592";

fn main() -> Result<()>{
    env_logger::init_from_env(
        env_logger::Env::new()
        .filter_or("UFS_LOG", "RUST_LOG")
        .write_style_or("UFS_LOG_STYLE", "RUST_LOG_STYLE"),
    );

    let args = Opt::from_args();
    match args.command {
        Command::Upload(opt) => {
            runtime::Builder::new_multi_thread()
                .thread_name(PACKAGE_NAME)
                .enable_all()
                .build()
                .unwrap_or_else(|err| exit!("Cannot create async runtime: {}", err))    // TODO: error handling
                .block_on(async {
                        if let Err(e) = send_file(opt).await {
                            exit!("Send file error: {:?}", e);
                        }
                    });    
        },

        Command::Start(opt) => {
            let mut config = RuntimeConfig::from_toml(DEFAULT_CONF_PATH)?;
            config.server_host(opt.host);
            config.server_ip(opt.ip);
            config.server_port(opt.port);
            Server::new(config).run()?;
        }
    }

    Ok(())
}

async fn send_file(opt: UploadOpt) -> Result<()>{
    let upload_path = opt.file;
    let data = file_data(upload_path).await?;

    let uri = format!("http://{}:{}", opt.ip, opt.port);
    let req = Request::post(uri)
    .header(CONTENT_TYPE, &*format!("multipart/form-data; boundary={}", BOUNDARY))
    .body(Body::from(data))?;

    log::trace!("REQUEST={:?}", req);

    // let client = Client::builder().build::<_, hyper::Body>(https);   // https client
    let client = Client::new();
    let response = client.request(req).await?;

    log::trace!("RESPONSE={:?}", response);

    if !response.status().is_success() {
        exit!("Failed to get a successful response status!");
    }
    log::info!("Status: {}", response.status());
    
    Ok(())
}

async fn file_data<P: AsRef<Path>>(path: P) -> Result<Vec<u8>> {
    let mut result = Vec::new();
    write!(result, "--{}\r\n", BOUNDARY)?;
    write!(result, "Content-Disposition: form-data; name=\"file\"; filename=\"upload.txt\"\r\n")?;
    write!(result, "Content-Type: text/plain\r\n")?;
    write!(result, "\r\n")?;
    let mut file = fs::File::open(path).await?;
    file.read_to_end(&mut result).await?;
    write!(result, "\r\n")?;
    write!(result, "--{}--\r\n", BOUNDARY)?;

    Ok(result)
}
