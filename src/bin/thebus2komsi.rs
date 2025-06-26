use clap::Parser;
use the_bus_2_komsi::opts::Opts;
use the_bus_2_komsi::serial::show_serial_comports;
use the_bus_2_komsi::realmain::real_main;

fn main() {
    let opts = Opts::parse();

    if opts.list {
        show_serial_comports();
        return;
    }

    // default
    real_main(&opts);
}
