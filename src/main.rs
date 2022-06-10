#![warn(warnings)]

mod app;
mod config;
mod module;

use app::Application;
use config::Config;
use clap::Parser;

#[derive(clap::Parser, serde::Deserialize)]
struct Args {
    #[clap(long, short)]
    config: String,
    #[clap(long, short)]
    foreground: bool,
}

#[tokio::main]
async fn main() -> Result<(), hyper::Error> {
    env_logger::init();

    let args = Args::parse();

    let config = match load_config(args.config) {
        Ok(config) => config,
        Err(err) => panic!("{err}"),
    };

    Application::new()
        .execute(config)
        .await
}

fn load_config(path: String) -> Result<Config, String> {
    use std::io::Read;

    let mut file = match std::fs::File::open(path.clone()) {
        Ok(file) => file,
        Err(err) => return Err(format!("Unable to open {path:?}: {err}")),
    };

    let mut content = String::new();

    match file.read_to_string(&mut content) {
        Ok(content) => content,
        Err(err) => return Err(format!("Unable to read {path:?}: {err}")),
    };

    let config = match toml::from_str(content.as_str()) {
        Ok(config) => config,
        Err(err) => return Err(format!("Unable to parse configuration: {err}")),
    };

    Ok(config)
}
