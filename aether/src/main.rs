use aether::server::{self, serve::run_server};
use aether_core::config_manager::models::gen_config_file;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    verbose: bool,

    #[arg(short, long, default_missing_value = "aether.conf.toml")]
    config_file: Option<String>,

    #[arg(short, long, default_missing_value = "0.0.0.0:7890", num_args = 0..=1)]
    serve: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match args.config_file {
        Some(file) => gen_config_file(&file),
        None => {
            // Use default config
        }
    }

    if let Some(port) = args.serve {
        let config_file = args.config_file;
        let config = run_server(&port, config_file).await;
    }
}
