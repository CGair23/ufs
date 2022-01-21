use tokio::runtime;
use config::RuntimeConfig;
use anyhow::Result;
use hyper::server::Server as HyperServer;
use std::net::SocketAddr;
use hyper::Error;
use hyper::service::{make_service_fn, service_fn};
use std::sync::Arc;

use crate::utils::{get_valid_dirpath};
use crate::handlers::handle::upload_service;
use crate::PACKAGE_NAME;

pub struct Server {
    conf: RuntimeConfig,
    threads: usize,
}

impl Server {
    pub fn new(config: RuntimeConfig) -> Self {
        // Configure number of worker threads
        let cpus = num_cpus::get();
        log::info!("We are on a multicore system with {} CPUs", cpus);
        Server {
            conf: config,
            threads: cpus
        }
    }

    /// Build and run the multi-thread `Server`.
    pub fn run(&self) -> Result<()>{
        // build runtime
        runtime::Builder::new_multi_thread()
        .thread_name(PACKAGE_NAME)
        .enable_all()
        .build()
        .unwrap_or_else(|err| exit!("Cannot create async runtime: {}", err))    // TODO: error handling
        // Execute the future, blocking the current thread until completion
        .block_on(async {
                let r = self.start_server().await;
                if r.is_err() {
                    exit!("Server error during start up: {:?}", r.unwrap_err())
                }
            });
        Ok(())
    }

    async fn start_server(&self) -> Result<()> {
        // Check for a valid root directory
        let fs_root_dir = get_valid_dirpath(&self.conf.fs_root)?;
        let fs_root_dir = Arc::new(fs_root_dir.into_os_string().into_string().expect("parse Pathbuf error"));

        // Run the corresponding HTTP Server asynchronously with its given options
        // HTTP/1
        let addr = SocketAddr::new(self.conf.server_config.ip.parse()?, self.conf.server_config.port);
        log::info!("Bound to TCP socket {}", addr.to_string());

        // Add a UploadService to handle each connection...
        // The closure inside `make_service_fn` is run for each connection,
        // creating a 'service' to handle requests for that specific connection.
        let upload_service =
            make_service_fn(move |_| {    
            // While the fs_root_dir was moved into the make_service closure,
            // we need to clone it here because this closure is called
            // once for every connection.
            //
            // Each connection could send multiple requests, so
            // the `Service` needs a clone to handle later requests.
            let fs_root_dir = fs_root_dir.clone();
            async move { 
                // This is the `Service` that will handle the connection.
                // `service_fn` is a helper to convert a function that
                // returns a Response into a `Service`. 
                Ok::<_, Error>(service_fn(move |req| {
                    let fs_root_dir = fs_root_dir.clone();     
                    async move {
                        let response = upload_service(req, fs_root_dir).await.unwrap();
                        Ok::<_, Error>(response)
                }
                })) 
            }
            });

        let server = HyperServer::bind(&addr)
            .serve(upload_service);
        if let Err(e) = server.await {
            exit!("Server error: {}", e);
        }

        Ok(())
    }
}