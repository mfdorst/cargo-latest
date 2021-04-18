use crates_io_api::SyncClient;
use std::env;
use std::time::Duration;
use thiserror::Error;

fn main() -> Result<()> {
    let client = SyncClient::new(
        "cargo-latest (cargo-latest@mdorst.net)",
        Duration::from_millis(1000),
    )?;
    let args = env::args().collect::<Vec<_>>();
    // When run with `cargo latest`, cargo does not strip out the `latest` parameter, so we have to strip that out.
    // In order to allow this to be run as `cargo-latest`, we need to check if the second argument is `latest`.
    // Unfortunately this causes a bug where if you want to check the version of the `latest` crate with `cargo-latest`,
    // you have to do `cargo-latest latest latest` as a workaround. However, this is the lesser of two evils.
    let n_args_to_skip = match args.len() > 2 && args[1] == "latest" {
        true => 2,
        false => 1,
    };
    for arg in env::args().skip(n_args_to_skip) {
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
