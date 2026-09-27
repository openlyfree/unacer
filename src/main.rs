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
    let mode: u8;

    match cli.command {
        Commands::Mode { setting } => match setting {
            PerfMode::Silent => {
                mode = 0x00;
            }
            PerfMode::Normal => {
                mode = 0x01;
            }
            PerfMode::Performance => {
                mode = 0x02;
            }
        },
    }

    let mut payload = vec![0xA0, 0x00, 0xA0, 0x01, 0x00, 0x01, mode, 0x00, 0x00];
    payload.resize(65, 0x00);

    let device = get_hid();

    device
        .send_feature_report(&payload)
        .expect("feature report send fail");

    let mut get_buf = vec![0u8; 65];
    get_buf[0] = 0xA0;

    device
        .get_feature_report(&mut get_buf)
        .expect("feature report get fail");

    let status = (get_buf[1] as u16) | ((get_buf[2] as u16) << 8);
    let command_id = (get_buf[3] as u16) | ((get_buf[4] as u16) << 8);

    //awknowledge doesnt always mean change happened (idk tho)
    if status != 0xE000 || command_id != 0x0001 {
        eprintln!("EC did not acknowledge: {:02X?}", get_buf);
        std::process::exit(1);
    }
}

fn get_hid() -> HidDevice {
    let api = HidApi::new().expect("HidApi init error");
    api.open(0x1025, 0x174B).expect("hid device open fail")
}
