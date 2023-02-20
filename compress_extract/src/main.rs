extern crate flate2;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::io::copy;
use std::fs::File;
use std::io::BufReader;
use std::time::Instant;


fn main() {

    if args().len() != 4 {
        eprintln!("Usage: `compress/decompress` `source` `target`");
        return;
    }
    let input = BufReader::new(File::open(args().nth(2).unwrap()).unwrap());
    let output = File::create(args().nth(3).unwrap()).unwrap();

    if args().nth(1).unwrap() == "compress"{
        compression(input, output);
    }else if args().nth(1).unwrap() == "decompress"{
        decompression(input, output);
    }

    }

fn compression(mut input:BufReader<File>, output:File) -> File{
    let mut encoder = GzEncoder::new(output, Compression::default());
    let starttime = Instant::now();
    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();
    println!("Source len: {}", input.get_ref().metadata().unwrap().len());
    println!("Target len: {}", output.metadata().unwrap().len());
    println!("Elapsed: {:?}", starttime.elapsed());
    output
}

fn decompression(mut input: BufReader<File>, mut output: File) -> File {
    let mut decoder = GzDecoder::new(input.get_mut());
    let starttime = Instant::now();
    copy(&mut decoder, &mut output).unwrap();
    println!("Source len: {}", input.get_ref().metadata().unwrap().len());
    println!("Target len: {}", output.metadata().unwrap().len());
    println!("Elapsed: {:?}", starttime.elapsed());
    output
}

