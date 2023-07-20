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
            md2,
            md4,
            md5,
            md6,
            sha1,
            sha224,
            sha256,
            sha384,
            sha512,
            sha512_224,
            sha512_256,
            bcrypt [cost: 12]
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
        "v1.0.0"
    }

    pub fn about() -> &'static str
    {
        "
          _        _        _        _        _        _        _        _        _        _    
        _( )__   _( )__   _( )__   _( )__   _( )__   _( )__   _( )__   _( )__   _( )__   _( )__ 
      _|     _|_|     _|_|     _|_|     _|_|     _|_|     _|_|     _|_|     _|_|     _|_|     _|
     (_ S _ (_(_ E _ (_(_ C _ (_(_ U _ (_(_ R _ (_(_ E _ (_(_ H _ (_(_ A _ (_(_ S _ (_(_ H _ (_ 
       |_( )__| |_( )__| |_( )__| |_( )__| |_( )__| |_( )__| |_( )__| |_( )__| |_( )__| |_( )__|

        🦀🔢 Rust tool for hashing text using multiple algorithms 🔢🦀
        "
    }
}
