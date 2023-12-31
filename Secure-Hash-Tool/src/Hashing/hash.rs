use md2::Md2;
use md4::Md4;
use md5;
use sha1::Sha1;
use sha2::{Sha256, Sha512, Sha512_224, Sha512_256, Sha224, Sha384, Digest};
use bcrypt::{DEFAULT_COST, hash};
use crate::Hashing::algorithms::Algorithm;

pub fn calculate_hash(algorithm: &Algorithm, data: &[u8]) -> String
{
    match algorithm {
        Algorithm::Md2       =>     {
            let mut hasher = Md2::new();
            hasher.update(data);
            let result = hasher.finalize();
            format!("{:x}", result)
        },
        Algorithm::Md4       =>     {
            let mut hasher = Md4::new();
            hasher.update(data);
            let result = hasher.finalize();
            format!("{:x}", result)
        },
        Algorithm::Md5       =>     {
            let hasher = md5::compute(data);
            format!("{:x}", hasher)
        },
        Algorithm::Sha1    =>     {
            let mut hasher = Sha1::new();
            hasher.update(data);
            let result = hasher.finalize();
            format!("{:x}", result)
        },
        Algorithm::Sha224    =>     {
            let mut hasher = Sha224::new();
            hasher.update(data);
            let result = hasher.finalize();
            format!("{:x}", result)
        },
        Algorithm::Sha256    =>     {
            let mut hasher = Sha256::new();
            hasher.update(data);
            let result = hasher.finalize();
            format!("{:x}", result)
        },
        Algorithm::Sha384    =>     {
            let mut hasher = Sha384::new();
            hasher.update(data);
            let result = hasher.finalize();
            format!("{:x}", result)
        }
        Algorithm::Sha512    =>     {
            let mut hasher = Sha512::new();
            hasher.update(data);
            let result = hasher.finalize();
            format!("{:x}", result)
        },
        Algorithm::Sha512_224   =>     {
            let mut hasher = Sha512_224::new();
            hasher.update(data);
            let result = hasher.finalize();
            format!("{:x}", result)
        } ,
        Algorithm::Sha512_256   =>     {
            let mut hasher = Sha512_256::new();
            hasher.update(data);
            let result = hasher.finalize();
            format!("{:x}", result)
        },
        Algorithm::Bcrypt       =>     {
            let hasher = match hash(data, DEFAULT_COST) {
                Ok(h)       =>      h,
                Err(_)              =>      {
                    eprintln!("error.");
                    return String::new();
                }
            };
            hasher
        },
        Algorithm::Help     =>      {
            eprintln!("The --help option is used to display the program's help");
            eprintln!("It does not calculate hash");
            std::process::exit(0);
        }
    }
}
