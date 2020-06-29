use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use structopt::StructOpt;

use jjay::error::*;

#[derive(StructOpt)]
pub struct Options {
    #[structopt(help = "", short = "c", long = "compact")]
    pub compact: bool,

    #[structopt(help = "")]
    pub file: PathBuf,
}

fn main() {
    let opts = Options::from_args();

    if let Err(err) = run(opts) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}

fn run(opts: Options) -> ScriptResult<()> {
    // read script
    let script = if &opts.file == Path::new("-") {
        let mut script = String::new();
        io::stdin().read_to_string(&mut script)?;
        script
    } else {
        fs::read_to_string(opts.file)?
    };

    // run script
    let value = jjay::run_script(script)?;

    // print value
    let out = std::io::stdout();
    if opts.compact {
        value.write_to(out)?;
    } else {
        value.write_to_pretty(out)?;
    }

    Ok(())
}
