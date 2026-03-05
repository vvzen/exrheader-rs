use std::path::PathBuf;

use clap::Parser;

use exrheader_rs_lib::{format_metadata, parse_metadata, print_metadata};

#[derive(Debug, Parser)]
struct Cli {
    /// List of EXR files to process.
    exr_paths: Vec<PathBuf>,
}

fn main() -> color_eyre::Result<()> {
    let log_env_setup = env_logger::Env::default().default_filter_or("INFO");
    env_logger::Builder::from_env(log_env_setup).init();

    let cli = Cli::parse();
    log::info!("Files received: {:?}", cli.exr_paths);

    let mut errors = Vec::new();
    let metadata: Vec<_> = cli
        .exr_paths
        .iter()
        .map(|f| (f, parse_metadata(&f)))
        .filter_map(|(f, r)| match r {
            Ok(m) => Some((f, m)),
            Err(e) => {
                errors.push(e);
                None
            }
        })
        .collect();

    for error in errors {
        log::error!("{error}");
    }

    for (file, metadata) in metadata {
        println!("File '{}'\n", file.display());
        let lines = format_metadata(metadata)?;
        print_metadata(&lines)?;
    }

    Ok(())
}
