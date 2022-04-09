use std::io::{Write, Read};

use rstar::primitives::GeomWithData;
use serde::{Serialize, Deserialize};

// TODO FIXME partialeq probably wrong
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub struct RPlacePixelData {
    pub user: u32,
    pub timestamp_millis: u16,
    pub timestamp_seconds: u8,
    pub timestamp_minutes: u8,
    pub timestamp_hours: u8,
    pub timestamp_days: u8,
    pub color: u8,

    // color 8 bits - 32
    // x 16 bits - 2000
    // y 16 bits - 2000
    // timestamp 32 bits (only these few days) 4 days * 24 hours * 60 minutes * 60 seconds * 1000 milliseconds
    // user 32 bits
}

pub type RPlacePixel = GeomWithData<[i16; 2], RPlacePixelData>;


pub fn write_rplacepixel<W: Write>(rplacepixel: &RPlacePixel, write: &mut W) {
    write.write(&rplacepixel.geom()[0].to_ne_bytes()).unwrap();
    write.write(&rplacepixel.geom()[1].to_ne_bytes()).unwrap();
    write.write(&rplacepixel.data.user.to_ne_bytes()).unwrap();
    write.write(&rplacepixel.data.timestamp_millis.to_ne_bytes()).unwrap();
    write.write(&rplacepixel.data.timestamp_seconds.to_ne_bytes()).unwrap();
    write.write(&rplacepixel.data.timestamp_minutes.to_ne_bytes()).unwrap();
    write.write(&rplacepixel.data.timestamp_hours.to_ne_bytes()).unwrap();
    write.write(&rplacepixel.data.timestamp_days.to_ne_bytes()).unwrap();
    write.write(&rplacepixel.data.color.to_ne_bytes()).unwrap();
}

pub fn read_rplacepixel<R: Read>(read: &mut R) -> Result<RPlacePixel, std::io::Error> {
    let mut vec: Vec<u8> = vec![0; 16];
    read.read_exact(&mut vec)?;

    Ok(GeomWithData::new([i16::from_ne_bytes(vec[0..1].try_into().unwrap()), i16::from_ne_bytes(vec[2..3].try_into().unwrap())], RPlacePixelData {
        user: u32::from_ne_bytes(vec[4..7].try_into().unwrap()),
        timestamp_millis: u16::from_ne_bytes(vec[8..9].try_into().unwrap()),
        timestamp_seconds: vec[10],
        timestamp_minutes: vec[11],
        timestamp_hours: vec[12],
        timestamp_days: vec[13],
        color: vec[14],
    }))
}