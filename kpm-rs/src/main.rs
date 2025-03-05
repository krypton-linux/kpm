/// kpm-rs - Krypton Package Manager made by rust
///
/// Copyright (c) 2025 Krypton-linux team
///
/// This program may be freely redistributed under the terms of the GNU General Public License.
/// 

use clap::Parser;
use tr::tr;


#[derive(Parser)]
struct Cli{
    #[arg(short, long)]
    version: bool

}


fn main() {
    let args = Cli::parse();

    if args.version {
        let alpm_v = alpm::version();
        let papm_version = "0.1 Alpha";
        println!("papm {}",  papm_version);
        println!("libalpm v{}", alpm_v);
        return;
    }

    eprintln!("{}", tr!("No subcommand specified"));

}
