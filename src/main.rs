extern crate docopt;
#[macro_use]
extern crate serde_derive;

mod config;

use config::Config;

const USAGE: &'static str = "
Usage:
  hermod [-h|--help] [--config=<PATH>] [--foreground]

Options:
  -h --help             Display this message
  -c --config=<PATH>    Specify a config file [default: /etc/hermod/main.cfg]
  -f --foreground       Start foreground (do not daemonize)
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_config: String,
    flag_foreground: bool,
}

fn main() {
    let docopt = match ::docopt::Docopt::new(USAGE) {
        Ok(docopt) => docopt,
        Err(err) => err.exit(),
    };

    let args: Args = match docopt.deserialize() {
        Ok(args) => args,
        Err(err) => err.exit(),
    };

    println!("{:?}", args);
}
