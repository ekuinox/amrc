use crate::conf::Conf;
use anyhow::Result;
use clap::{Parser, ValueEnum};
use reqwest::Url;
use std::{fs::File, io::BufReader, path::PathBuf};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Method {
    Get,
}

#[derive(Parser, Debug)]
pub struct RequestSubcommand {
    #[clap(name = "CONF_FILE_PATH")]
    conf: PathBuf,

    #[arg(value_enum)]
    method: Method,

    path: String,

    #[clap(long = "json")]
    json: bool,
}

impl RequestSubcommand {
    pub async fn run(self) -> Result<()> {
        let reader = File::open(&self.conf).map(BufReader::new)?;
        let conf: Conf = serde_yaml::from_reader(reader)?;

        let base_url = Url::parse(&conf.preset.base_url)?;
        let url = base_url.join(&self.path)?;

        let request = reqwest::Client::new()
            .get(url)
            .bearer_auth(conf.access_token);

        let text = match self.method {
            Method::Get => request.send().await?.text().await?,
        };

        if self.json {
            let json = serde_json::from_str::<serde_json::Value>(&text)?;
            let text = serde_json::to_string_pretty(&json)?;
            println!("{text}");
        } else {
            println!("{text}")
        }

        Ok(())
    }
}
