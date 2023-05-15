use clap::Parser;
use color_eyre::eyre::Result;

use system::{System, SystemConfig};

const ABOUT: &str = r#"
██╗  ██╗███████╗███╗   ███╗██╗   ██╗██████╗ ███████╗
╚██╗██╔╝██╔════╝████╗ ████║██║   ██║██╔══██╗██╔════╝
 ╚███╔╝ █████╗  ██╔████╔██║██║   ██║██████╔╝███████╗
 ██╔██╗ ██╔══╝  ██║╚██╔╝██║██║   ██║██╔══██╗╚════██║
██╔╝ ██╗███████╗██║ ╚═╝ ██║╚██████╔╝██║  ██║███████║
╚═╝  ╚═╝╚══════╝╚═╝     ╚═╝ ╚═════╝ ╚═╝  ╚═╝╚══════╝"#;

#[derive(Parser, Debug)]
#[command(bin_name = "xemurs")]
#[command(next_line_help = true)]
#[command(name = "xemurs", author, version, about = ABOUT, long_about = Some(ABOUT))]
struct Cli;

fn main() -> Result<()> {
    color_eyre::install()?;
    Cli::parse();

    System::new(SystemConfig::default())?.start()?;

    Ok(())
}
