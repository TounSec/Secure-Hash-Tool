use crate::Hashing::algorithms::Algorithm;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = Opt::name(),
    version = Opt::version(),
    about = Opt::about(),
)]
pub struct Opt {
    #[structopt(
        short = "a",
        long = "algorithm",
        default_value = "sha256",
        help = "Choose the hash algorithm (
            sha256,
            sha512
        )",
        )]
    pub algorithm: Algorithm,

    #[structopt(name = "TEXT", help = "The text to hash")]
    pub text: String,
}

impl Opt {
    pub fn name() -> &'static str
    {
        "Secure-Hash-Tool"
    }

    pub fn version() -> &'static str
    {
        "1.0.0v"
    }

    pub fn about() -> &'static str
    {
        "🦀🔢 Rust tool for hashing text using multiple algorithms 🔢🦀"
    }
}
