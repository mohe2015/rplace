use serde::{Serialize, Deserialize};




#[derive(Serialize, Deserialize, Debug)]
pub struct RPlacePixel {
    pub user: u32,
    pub x: u16,
    pub y: u16,
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
