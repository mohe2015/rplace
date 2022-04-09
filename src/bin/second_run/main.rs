use std::{io, fs::File};

use ciborium::de::from_reader;
use rplace::RPlacePixel;




fn main() -> io::Result<()> {
    let bf = File::open("test.bin")?;

    let result: Vec<RPlacePixel> = from_reader(bf).unwrap();

    println!("{:?}", result);

    Ok(())
}