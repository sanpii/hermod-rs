extern crate docopt;
#[macro_use]
extern crate serde_derive;
extern crate toml;

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

    let config = match load_config(args.flag_config) {
        Ok(config) => config,
        Err(err) => panic!("{}", err),
    };

    println!("{:#?}", config);
}

fn load_config(path: String) -> Result<Config, String> {
    use std::io::Read;

    let mut file = match ::std::fs::File::open(path.clone()) {
        Ok(file) => file,
        Err(err) => return Err(format!("Unable to open {:?}: {}", path, err)),
    };

    let mut content = String::new();

    match file.read_to_string(&mut content) {
        Ok(content) => content,
        Err(err) => return Err(format!("Unable to read {:?}: {}", path, err)),
    };

    let config = match ::toml::from_str(content.as_str()) {
        Ok(config) => config,
        Err(err) => return Err(format!("Unable to parse configuration: {}", err)),
    };

    Ok(config)
}
