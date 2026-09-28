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
    /// CPU fan tachometer, in RPM.
    #[command(name = "get-rpm")]
    GetRpm,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum PerfMode {
    Silent,
    Normal,
    Performance,
}

fn main() {
    let cli = Cli::parse();
    let device = get_hid();

    match cli.command {
        Commands::Mode { setting } => {
            let mode = match setting {
                PerfMode::Silent => 0x02,
                PerfMode::Normal => 0x01,
                PerfMode::Performance => 0x00,
            };
            
            let mut payload = vec![0xA0, 0x00, 0xA0, 0x01, 0x00, 0x01, mode, 0x00, 0x00];
            payload.resize(65, 0x00);
            
            let reply = exchange(&device, &payload);
            let status = (reply[1] as u16) | ((reply[2] as u16) << 8);
            let command_id = (reply[3] as u16) | ((reply[4] as u16) << 8);

            //awknowledge doesnt always mean change happened (idk tho)
            if status != 0xE000 || command_id != 0x0001 {
                eprintln!("EC did not acknowledge: {:02X?}", reply);
                std::process::exit(1);
            }
        }

        Commands::GetRpm => {
            let mut payload = vec![0xA0, 0x00, 0xA0, 0x08, 0x00, 0x02, 0x02, 0x00, 0x00];
            payload.resize(65, 0x00);
            
            let reply = exchange(&device, &payload);
            let status = (reply[1] as u16) | ((reply[2] as u16) << 8);
            let command_id = (reply[3] as u16) | ((reply[4] as u16) << 8);
            
            if status != 0xE000 || command_id != 0x0008 {
                eprintln!("EC did not acknowledge: {:02X?}", reply);
                std::process::exit(1);
            }
            
            let rpm = (reply[8] as u16) | ((reply[9] as u16) << 8);
            println!("{rpm}");
        }
    }
}

fn exchange(device: &HidDevice, payload: &[u8]) -> Vec<u8> {
    device
        .send_feature_report(payload)
        .expect("feature report send fail");

    let mut get_buf = vec![0u8; 65];
    get_buf[0] = 0xA0;

    device
        .get_feature_report(&mut get_buf)
        .expect("feature report get fail");

    get_buf
}

fn get_hid() -> HidDevice {
    let api = HidApi::new().expect("HidApi init error");
    api.open(0x1025, 0x174B).expect("hid device open fail")
}
