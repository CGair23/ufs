use structopt::StructOpt;
use crate::DEFAULT_CONF_PATH;

#[derive(Debug, StructOpt)]
#[structopt(name = "ufs_cli", about = "ufs command line tool")]
pub struct Opt {
    #[structopt(subcommand)]
    pub command: Command
}

#[derive(Debug, StructOpt)]
pub enum Command {
    /// Upload file
    #[structopt(name = "upload")]
    Upload(UploadOpt),

    /// Start file server
    #[structopt(name = "start")]
    Start(StartOpt),
}

#[derive(Debug, StructOpt)]
pub struct UploadOpt {
    #[structopt(short, long = "ip-address")]
    pub ip: String,

    #[structopt(short, long)]
    pub port: u16,

    #[structopt(short = "f", long = "file-path")]
    pub file: String,

    #[structopt(short = "t", long = "task-id")]
    pub task: String,
}

#[derive(Debug, StructOpt)]
pub struct StartOpt {
    #[structopt(short, long, default_value = "127.0.0.1")]
    pub host: String,
    #[structopt(short, long = "ip-address", default_value = "127.0.0.1")]
    pub ip: String,
    #[structopt(short, long, default_value = "8080")]
    pub port: u16,
    #[structopt(short, long, default_value = DEFAULT_CONF_PATH)]
    pub config: String,
}