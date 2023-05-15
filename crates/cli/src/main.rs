use clap::Parser;

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

fn main() {
    Cli::parse();
}
