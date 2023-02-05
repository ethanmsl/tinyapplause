// disable clippy lint prevent a lint from running
#![allow(clippy::uninlined_format_args)]
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "tinyApplause")]
#[command(about = "does some random things")]
#[command(long_about = "boopsy-bospy-kimklam")]
#[command(version)]
// #[command(next_line_help=true)]
struct BaseArgs {
    /// god this sucks
    name: Vec<String>,

    /// install support cli
    #[arg(long, value_name = "Install")]
    to_install: bool,

    #[arg(long)]
    numbs: Option<u32>,

    /// subcomzzz
    #[command(subcommand)]
    subcommand: Option<SubCmnds>,
}

#[derive(Subcommand)]
enum SubCmnds {
    #[command(author, version, about, long_about = "boopsy-bospy-kimklam")]
    /// just chatty
    TellMeThings {
        /// heroine name
        #[arg(short = 'n', long)]
        heroine: Vec<String>,

        /// hero name
        #[arg(long = "gyro")]
        hero: String,
    },
}

fn main() {
    let cli = BaseArgs::parse();

    if cli.to_install {
        println!("installing support cli");
        println!("woo!!!");
        println!("Woo1111");
        println!(":) :) :) :) :)");
    }

    if !cli.name.is_empty() {
        println!("name: {:?}", cli.name);
    }

    if let Some(numbs) = cli.numbs {
        println!("numbs: {}", numbs);
    }

    if let Some(subcommand) = cli.subcommand {
        match subcommand {
            SubCmnds::TellMeThings { heroine, hero } => {
                println!("heroine: {heroine:?}");
                println!("hero: {:?}", hero);
            }
        }
    }
}
