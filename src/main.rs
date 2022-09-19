mod subdomain;
mod error;
mod ports;
mod model;

use std::env;
use std::time::Duration;
use reqwest::{Client, redirect};
pub use error::Error;
use crate::model::Subdomain;

fn main() -> Result<(), anyhow::Error> {
    let args:Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err(Error::CliUsage.into());
    }

    let target = args[1].as_str();

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(256)
        .build().unwrap();

    let http_timeout = Duration::from_secs(5);
    let http_client = Client::builder()
        .redirect(redirect::Policy::limited(4))
        .timeout(http_timeout)
        .build()?;



    pool.install(|| {
        let scan_result: Vec<Subdomain> = subdomains::enumerate(&http_client, target)
            .unwrap()
            .into_par_iter()
            .map(ports::scan_ports)
            .collect();

        for subdomain in scan_result {
            println!("{}:", &subdomain.domain);
            for port in &subdomain.open_ports {
                println!("    {}", port.port);
            }

            println!();
        }
    })

    Ok(())
}
