use std::path::PathBuf;

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
    CosmacVip { rom: PathBuf },
}

fn main() -> Result<()> {
    color_eyre::install()?;

    match Cli::parse() {
        Cli::Start(emulator) => match emulator {
            EmulatorOpt::CosmacVip { rom: path } => {
                use emulator::cosmac_vip::{CosmacVip, Rom};

                let rom = Rom::from_path(&path);
                let mut cosmac_vip = CosmacVip::new(rom);
                let mut system = System::new(SystemConfig::default())?;

                cosmac_vip.start(&mut system)?;
            }
        },
    }

    Ok(())
}
