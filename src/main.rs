use crates_io_api::SyncClient;
use std::env;
use std::time::Duration;
use thiserror::Error;

fn main() -> Result<()> {
    let client = SyncClient::new(
        "cargo-latest (cargo-latest@mdorst.net)",
        Duration::from_millis(1000),
    )?;
    for arg in env::args().skip(1) {
        match client.get_crate(&arg) {
            Ok(krate) => println!(r#"{} = "{}" "#, arg, krate.crate_data.max_version),
            Err(_) => eprintln!("Crate '{}' not found.", arg),
        }
    }
    Ok(())
}

#[derive(Error, Debug)]
enum Error {
    #[error(transparent)]
    InvalidHeaderValue(#[from] http::header::InvalidHeaderValue),
}

type Result<T, E = Error> = std::result::Result<T, E>;
