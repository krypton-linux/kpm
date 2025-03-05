/// kpm-rs - Krypton Package Manager made by rust
///
/// Copyright (c) 2025 Krypton-linux team
///
/// This program may be freely redistributed under the terms of the GNU General Public License.
/// 

use clap::{Parser, Subcommand};
use tr::tr;
use std::env;
mod sync_db;


#[derive(Parser)]
struct Cli{
    #[command(subcommand)]
    commands: Commands,
    #[arg(short, long)]
    version: bool
}

#[derive(Subcommand)]
enum Commands{
    Syncdb,
}



fn main() {
    tr::tr_init!(env::var("LOCALE_DIR")
    .as_deref()
    .unwrap_or("/usr/share/locale/"));

    let args = Cli::parse();

    println!("{}", tr!("This is a test"));

    if args.version {
        let alpm_v = alpm::version();
        let papm_version = "0.1 Alpha";
        println!("papm {}",  papm_version);
        println!("libalpm v{}", alpm_v);
        return;
    }

    match args.commands{
        Commands::Syncdb => {
            sync_db::sync_db();
            return;
        }
    }


}
