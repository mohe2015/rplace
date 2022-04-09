use std::{io, fs::File};

use rplace::RPlacePixel;
use rstar::{RTree, primitives::GeomWithData};

fn main() -> io::Result<()> {
    let bf = File::open("test.bin")?;

    let mut result: Vec<GeomWithData<[i16; 2], RPlacePixel>> = bincode::deserialize_from(bf).unwrap();

    result.sort_unstable_by_key(|v| (v.data.timestamp_days, v.data.timestamp_hours, v.data.timestamp_minutes, v.data.timestamp_seconds, v.data.timestamp_millis));    

    println!("{:?}", result);

    let mut tree = RTree::bulk_load(result);

    Ok(())
}