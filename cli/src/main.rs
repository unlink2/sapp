use std::env::args;

use jated::error::Error;
use jated::cli::*;

fn main() -> Result<(), Error> {
    exec(&args().collect::<Vec<String>>()) 
}
