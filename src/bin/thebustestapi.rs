use clap::Parser;
use TheBus2Komsi::opts::Opts;
use TheBus2Komsi::realmain::real_main;

fn main() {
    let opts = Opts::parse();

    // default
    real_main(&opts);
}
