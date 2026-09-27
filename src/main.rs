use clap::{Parser, Subcommand, ValueEnum};
use hidapi::{HidApi, HidDevice};

#[derive(Parser)]
#[command(name = "unacer")]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Mode {
        #[arg(value_enum)]
        setting: PerfMode,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum PerfMode {
    Silent,
    Normal,
    Performance,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Mode { setting } => match setting {
            PerfMode::Silent => {
                
            }
            PerfMode::Normal => {
            }
            PerfMode::Performance => {
            }
        },
    }
    
}

