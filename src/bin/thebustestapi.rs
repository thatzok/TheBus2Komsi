use structopt::StructOpt;
use TheBus2Komsi::opts::Opts;
use TheBus2Komsi::realmain::real_main;

fn main() {
    let opts = Opts::from_args();

    // default
    real_main(&opts);
}
