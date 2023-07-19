use std::str::FromStr;
use structopt::StructOpt;

#[derive(Debug, StructOpt, PartialEq)]
pub enum Algorithm {
    #[structopt(name = "sha256")]
    Sha256,
    #[structopt(name = "sha512")]
    Sha512,
    #[structopt(name = "help")]
    Help,
}

impl FromStr for Algorithm {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "sha256"        =>      Ok(Algorithm::Sha256),
            "sha512"        =>      Ok(Algorithm::Sha512),
            "help"          =>      Ok(Algorithm::Help),
            _               =>      Err("Invalid algorithm.".to_string()),
        }
    }
}
