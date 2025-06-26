use structopt::StructOpt;
use TheBus2Komsi::opts::Opts;
use TheBus2Komsi::serial::show_serial_comports;
use TheBus2Komsi::realmain::real_main;

fn main() {
    let opts = Opts::from_args();

    if opts.list {
        show_serial_comports();
        return;
    }

    // default
    real_main(&opts);
}

