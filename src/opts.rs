use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Opts {
    /// comport Arduino/ESP32 ist connected to
    #[structopt(short, long)]
    pub port: Option<String>,

    /// IP des PC auf dem TheBus läuft
    #[structopt(short, long, default_value = "127.0.0.1")]
    pub ip: String,

    /// Baud rate
    #[structopt(short, long, default_value = "115200")]
    pub baud: u32,

    /// Zeit in Millisekunden zwischen API aufrufen
    #[structopt(short, long, default_value = "200")]
    pub sleeptime: u64,

    /// enable debugging 
    #[structopt(short, long)]
    pub debug: bool,

    /// enable debugging of serial comport
    #[structopt(long)]
    pub debug_serial: bool,

    /// enable debugging of commands
    #[structopt(long)]
    pub debug_command: bool,

    /// show all available comports
    #[structopt(short, long)]
    pub list: bool,

    /// enable verbose output
    #[structopt(short, long)]
    pub verbose: bool,

    /// reset all vehicle values
    #[structopt(short, long)]
    pub clear: bool,
}
