use std::str::FromStr;
use structopt::StructOpt;

#[derive(Debug, StructOpt, PartialEq)]
pub enum Algorithm {
    #[structopt(name = "md2")]
    Md2,
    #[structopt(name = "md4")]
    Md4,
    #[structopt(name = "md5")]
    Md5,
    #[structopt(name = "sha1")]
    Sha1,
    #[structopt(name = "sha224")]
    Sha224,
    #[structopt(name = "sha256")]
    Sha256,
    #[structopt(name = "sha384")]
    Sha384,
    #[structopt(name = "sha512")]
    Sha512,
    #[structopt(name = "sha512_224")]
    Sha512_224,
    #[structopt(name = "sha512_256")]
    Sha512_256,
    #[structopt(name = "help")]
    Help,
}

impl FromStr for Algorithm {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "md2"           =>      Ok(Algorithm::Md2),
            "md4"           =>      Ok(Algorithm::Md4),
            "md5"           =>      Ok(Algorithm::Md5),
            "sha1"          =>      Ok(Algorithm::Sha1),
            "sha224"        =>      Ok(Algorithm::Sha224),
            "sha256"        =>      Ok(Algorithm::Sha256),
            "sha384"        =>      Ok(Algorithm::Sha384),
            "sha512"        =>      Ok(Algorithm::Sha512),
            "sha512_224"    =>      Ok(Algorithm::Sha512_224),
            "sha512_256"    =>      Ok(Algorithm::Sha512_256),
            "help"          =>      Ok(Algorithm::Help),
            _               =>      Err("Invalid algorithm.".to_string()),
        }
    }
}
