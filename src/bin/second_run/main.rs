use std::{io, fs::File};

use rplace::RPlacePixel;




fn main() -> io::Result<()> {
    let bf = File::open("test.bin")?;

    let result: Vec<RPlacePixel> = rmp_serde::from_read(bf).unwrap();

    println!("{:?}", result);

    Ok(())
}