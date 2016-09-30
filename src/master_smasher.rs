extern crate moho;

use std::error::Error;

pub fn run() -> Result<(), Box<Error>> {
    try!(moho::init("Master Smasher"));
    Ok(())
}
