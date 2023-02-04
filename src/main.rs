use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = "boopsy-bospy-kimklam")]
struct BaseArgs {
    /// install support cli
    #[arg(long, value_name = "Install")]
    to_install: bool,

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
        #[arg(long)]
        heroine: String,

        /// hero name
        #[arg(long)]
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

    if let Some(subcommand) = cli.subcommand {
        match subcommand {
            SubCmnds::TellMeThings { heroine, hero } => {
                println!("heroine: {heroine}");
                println!("hero: {hero}");
            }
        }
    }
}
