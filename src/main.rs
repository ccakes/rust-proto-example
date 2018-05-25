extern crate protobuf;
use protobuf::parse_from_bytes;

mod proto;
use proto::ethernet_controller_driver_bag_v2_type;

use std::fs;
use std::io::Read;

fn main() {
    let mut buf = vec![];
    let mut fh = fs::File::open("content.bin").expect("Error opening file");
    let _ = fh.read_to_end(&mut buf).expect("Error reading file");

    let msg = match parse_from_bytes::<ethernet_controller_driver_bag_v2_type>(&buf) {
        Ok(m) => m,
        Err(e) => panic!("Error: {}", e)
    };

    println!("{:#?}", msg);
}
