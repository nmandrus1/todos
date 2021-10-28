use std::path::PathBuf;

use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "todos", about = "Figure out what needs to be done")]
pub struct Args {
    // Input file, find project directory if None
    #[structopt(parse(from_os_str))]
    pub file: Option<PathBuf>,
}
