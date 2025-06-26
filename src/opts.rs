use clap::Parser;

#[derive(Parser, Debug)]
pub struct Opts {
    /// enable debugging
    #[arg(short, long)]
    pub debug: bool,

    /// enable debugging of serial comport
    #[arg(long)]
    pub debug_serial: bool,

    /// enable debugging of commands
    #[arg(long)]
    pub debug_command: bool,

    /// show all available comports
    #[arg(short, long)]
    pub list: bool,

    /// enable verbose output
    #[arg(short, long)]
    pub verbose: bool,
}
