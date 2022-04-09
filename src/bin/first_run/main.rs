use std::{io::{self, BufReader}, hash::BuildHasherDefault};
use flate2::read::GzDecoder;
use rplace::RPlacePixel;
use rstar::primitives::GeomWithData;
use time::{PrimitiveDateTime, macros::format_description, format_description::FormatItem};
use std::io::prelude::*;
use std::fs::File;
use rustc_hash::{FxHashMap, FxHasher};
use base64::{decode_config_slice, STANDARD};

// https://crates.io/crates/bincode

// cat /home/pi/2022_place_canvas_history.csv.gzip | gunzip | wc -c
// 21714634193
// ~22 GB

// user count: 10.381.162

// log2(32*2000*2000*10381162*4*24*60*60)

const FORMAT1: &[FormatItem] = format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond] UTC");
const FORMAT2: &[FormatItem] = format_description!("[year]-[month]-[day] [hour]:[minute]:[second] UTC");

fn main() -> io::Result<()> {
    let mut output = Vec::with_capacity(160353105);

    let stdout = io::stdout();
    let handle = stdout.lock();

    let f = File::open("/home/pi/2022_place_canvas_history.csv.gzip")?;
    let mut gz = BufReader::new(GzDecoder::new(f));

    let mut next_user_id = 0;
    //let mut next_pixel_color = -1;
    let mut user_ids = FxHashMap::with_capacity_and_hasher(10381162, BuildHasherDefault::<FxHasher>::default());
    //let mut pixel_colors = FxHashMap::default();
    let mut line = Vec::<u8>::new();
    let mut number_of_bytes_read = 0;
    gz.read_until(b'\n', &mut line)?; // drop header
    line.clear();
    loop {
        let line_length = gz.read_until(b'\n', &mut line)?;
        if line_length == 0 {
            break;
        }
        number_of_bytes_read += line_length;
        if number_of_bytes_read % (256 * 1024) == 0 {
            eprintln!("{}", f64::from(u32::try_from(number_of_bytes_read/1024).unwrap()) / f64::from(21205697));
        }

        // unfortunately already timestamps can have two digit or three digit milliseconds so we need to split the data. userid and colors should be the same though
        let mut it = line.split(|c| *c == b',');
        let timestamp = it.next().unwrap();
        let timestamp = std::str::from_utf8(timestamp).unwrap();

        //writeln!(handle, "{}", timestamp)?;

        let timestamp = PrimitiveDateTime::parse(timestamp, &FORMAT1).or_else(|_| PrimitiveDateTime::parse(timestamp, &FORMAT2)).unwrap();

        let timestamp = timestamp.assume_utc();

        let mut user_id = vec![0; 64];
        assert_eq!(64, decode_config_slice(it.next().unwrap(), STANDARD, &mut user_id).unwrap());
        let user_id = match user_ids.get(&user_id) {
            Some(v) => *v,
            None => {
                user_ids.insert(user_id.clone(), next_user_id);
                let ret = next_user_id;
                next_user_id += 1;
                ret
            }
        };
        let pixel_color = u32::from_str_radix(std::str::from_utf8(&it.next().unwrap()[1..]).unwrap(), 16).unwrap();
        /*let pixel_color = match pixel_colors.get(&pixel_color) {
            Some(v) => *v,
            None => {
                next_pixel_color += 1;
                pixel_colors.insert(pixel_color, next_pixel_color);
                next_pixel_color
            }
        };*/
        let pixel_color = match pixel_color {
            52416 => 0,
            0 => 5,
            11815616 => 20,
            16729344 => 25,
            5368308 => 13,
            16754688 => 27,
            41832 => 7,
            8461983 => 17,
            13948889 => 22,
            8318294 => 16,
            9014672 => 18,
            4799169 => 11,
            16726145 => 24,
            52344 => 8,
            16766517 => 29,
            16775352 => 30,
            30063 => 6,
            7161903 => 15,
            10250534 => 19,
            7143450 => 14,
            6970623 => 2,
            9745407 => 1,
            14986239 => 4,
            12451897 => 21,
            14553215 => 23,
            16757872 => 28,
            16777215 => 31,
            40618 => 3,
            3576042 => 10,
            2379940 => 9,
            16751018 => 26,
            5329490 => 12,
            _ => panic!(),
        };
        let coordinate_x = std::str::from_utf8(&it.next().unwrap()[1..]).unwrap().parse::<i16>().unwrap();
        let coordinate_y = it.next().unwrap();
        let coordinate_y = std::str::from_utf8(&coordinate_y[..coordinate_y.len()-2]).unwrap().parse::<i16>().unwrap();

        let value = GeomWithData::new([coordinate_x, coordinate_y], RPlacePixel {
            user: user_id,
            timestamp_millis: timestamp.millisecond(),
            timestamp_seconds: timestamp.second(),
            timestamp_minutes: timestamp.minute(),
            timestamp_hours: timestamp.hour(),
            timestamp_days: timestamp.day(),
            color: pixel_color,
        });

        output.push(value);

        //writeln!(handle, "{},{},{},{},{}", timestamp, user_id, pixel_color, coordinate_x, coordinate_y)?;
        line.clear();
    }

    bincode::serialize_into(File::create("test.bin")?, &output).unwrap();

    //eprintln!("{:#?}", pixel_colors);
    eprintln!("user count: {}", next_user_id);

    Ok(())
}
