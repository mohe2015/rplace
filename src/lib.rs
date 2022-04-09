use serde::{Serialize, Deserialize};




#[derive(Serialize, Deserialize, Debug)]
pub struct RPlacePixel {
    user: u32,
    x: u16,
    y: u16,
    timestamp_millis: u16,
    timestamp_seconds: u8,
    timestamp_minutes: u8,
    timestamp_hours: u8,
    timestamp_days: u8,
    color: u8,

    // color 8 bits - 32
    // x 16 bits - 2000
    // y 16 bits - 2000
    // timestamp 32 bits (only these few days) 4 days * 24 hours * 60 minutes * 60 seconds * 1000 milliseconds
    // user 32 bits
}
