use argparse::{ArgumentParser, StoreTrue, Store};

pub struct Options {
    pub version: bool,
    pub verbose: bool,
    pub cmd: String,
}

impl Options {
    pub fn new() -> Options {
        Options {
            version: false,
            verbose: false,
            cmd: String::new(),
        }
    }

    pub fn parse_args_static() -> Options {
        let mut opt = Options::new();
        opt.parse_args();
        opt
    }

    pub fn parse_args(&mut self) {
        let mut ap = ArgumentParser::new();
        ap.set_description("show - command line tool.");
        ap.refer(&mut self.version).add_option(&["-v", "--version"], StoreTrue, "Print version");
        ap.refer(&mut self.verbose).add_option(&["-V", "--verbose"], StoreTrue, "Verbose print");
        ap.refer(&mut self.cmd).add_argument("CMD", Store, "Command, use ':::' show all");
        ap.parse_args_or_exit();
    }
}
