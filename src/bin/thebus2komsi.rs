use clap::Parser;
use the_bus_2_komsi::opts::Opts;
use the_bus_2_komsi::serial::{show_precise_com_ports};
use the_bus_2_komsi::realmain::real_main;

fn main() {
    let opts = Opts::parse();

    if opts.list {
        show_precise_com_ports();
        return;
    }

    // default
    real_main(&opts);
}
