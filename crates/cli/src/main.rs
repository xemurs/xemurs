use clap::{Parser, Subcommand};
use color_eyre::eyre::Result;

use system::emulator::Emulator;
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
enum Cli {
    /// Starts the emulator
    #[clap(subcommand)]
    Start(EmulatorOpt),
}

#[derive(Debug, Subcommand)]
enum EmulatorOpt {
    CosmacVip,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    match Cli::parse() {
        Cli::Start(emulator) => match emulator {
            EmulatorOpt::CosmacVip => {
                use emulator::cosmac_vip::CosmacVip;

                let cosmac_vip = CosmacVip::new();
                let mut system = System::new(SystemConfig::default(), Box::new(cosmac_vip))?;

                system.start()?;
            }
        },
    }

    Ok(())
}
