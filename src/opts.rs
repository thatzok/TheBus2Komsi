use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Opts {

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

}
