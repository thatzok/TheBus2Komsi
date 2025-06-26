use clap::Parser;
use the_bus_2_komsi::opts::Opts;
use the_bus_2_komsi::realmain::real_main;

fn main() {
    let opts = Opts::parse();

    // default
    real_main(&opts);
}
