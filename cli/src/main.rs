use std::env::args;

use sapp::cli::*;
use sapp::error::Error;

fn main() -> Result<(), Error> {
    exec(&args().collect::<Vec<String>>())
}
