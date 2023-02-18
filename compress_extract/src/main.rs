extern crate flate2;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::io::copy;
use std::fs::File;
use std::io::BufReader;
use std::time::Instant;


fn main() {

    let mut input = BufReader::new((File::open(args());
    let output = File::create(args().net(2).unwrap()).unwrap();
}
