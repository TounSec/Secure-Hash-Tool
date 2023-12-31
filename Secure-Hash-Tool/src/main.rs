#![allow(non_snake_case)]
mod Hashing;

use Hashing::Algorithm;
use Hashing::calculate_hash;
use Hashing::Opt;
use structopt::StructOpt;

fn main()
{
    let opt = Opt::from_args();

    if opt.algorithm == Algorithm::Help {
        Opt::clap().print_help().unwrap();
        return;
    }

    let hash = calculate_hash(&opt.algorithm, opt.text.as_bytes());

    println!("The {:?} Hash is : {}", opt.algorithm, hash);
}