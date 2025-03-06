/// kpm-rs - Krypton Package Manager written in Rust
///
/// Copyright (c) 2025 Krypton Linux Team
///
/// This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
///
/// This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
///
/// You should have received a copy of the GNU General Public License along with this program. If not, see <http://www.gnu.org/licenses/>.

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
