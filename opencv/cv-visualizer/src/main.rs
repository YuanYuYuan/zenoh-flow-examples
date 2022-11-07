use zenoh::prelude::r#async::*;
use std::str::FromStr;
use clap::Parser;
use serde::{Serialize, Deserialize};
use std::{path::PathBuf, time::Duration};
use anyhow::Result;
use futures::select;
use futures::prelude::*;
use async_std::task::sleep;
use opencv::{core, highgui, prelude::*, videoio};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    config: PathBuf,
}

#[derive(Serialize, Deserialize)]
struct Config {
    zenoh_config: zenoh::config::Config,
    key_expr: String,
}

impl Config {
    fn load(path: PathBuf) -> Result<Self> {
        let config: Config = serde_yaml::from_str(&std::fs::read_to_string(path)?)?;
        Ok(config)
    }
}

#[async_std::main]
async fn main() -> Result<()> {
    let Args { config } = Args::parse();
    let config = Config::load(config)?;
    let session = zenoh::open(config.zenoh_config).res().await.unwrap();
    let subscriber = session.declare_subscriber(&config.key_expr).res().await.unwrap();

    async_std::task::spawn_blocking(move || {
        loop {
            while let Ok(data) = subscriber.recv_async().await {
                let decoded = opencv::imgcodecs::imdecode(
                    &opencv::types::VectorOfu8::from_iter(data),
                    opencv::imgcodecs::IMREAD_COLOR,
                )
                .unwrap();
                if decoded.size().unwrap().width > 0 {
                    highgui::imshow("Test", &decoded).unwrap();
                }
                highgui::wait_key(10).unwrap();
            }
        }
    });

    Ok(())
}
