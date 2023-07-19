use sha2::{Sha256, Sha512, Digest};
use crate::Hashing::algorithms::Algorithm;

pub fn calculate_hash(algorithm: &Algorithm, data: &[u8]) -> String 
{
    match algorithm {
        Algorithm::Sha256    =>     {
            let mut hasher = Sha256::new();
            hasher.update(data);
            let result = hasher.finalize();
            format!("{:x}", result)
        },
        Algorithm::Sha512    =>     {
            let mut hasher = Sha512::new();
            hasher.update(data);
            let result = hasher.finalize();
            format!("{:x}", result)
        },
        Algorithm::Help     =>      {
            eprintln!("The --help option is used to display the program's help");
            eprintln!("It does not calculate hash");
            std::process::exit(0);
        }
    }
}
