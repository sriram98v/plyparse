extern crate ply_rs;
extern crate clap;
use ply_rs as ply;
use ply_rs::writer::Writer;
use ply_rs::ply::{Encoding};
use clap::{arg, Command};

/// Demonstrates simplest use case for reading from a file.
fn main() {
    // parse arguments
    let matches = Command::new("PLY file encoding converter")
        .version("1.0")
        .author("Sriram Vijendran <vijendran.sriram@gmail.com>")
        .about("Converts .ply file encoding")
        .arg(arg!([file] ".ply file").required(true))
        .arg(arg!([encoding] "file encoding. be for big endian, le for little endian, ascii for Ascii ").required(true))
        .get_matches();

    // set up a reader, in this case a file.
    let mut f = std::fs::File::open(&matches.get_one::<std::path::PathBuf>("file").expect("required")).unwrap();

    // // create a parser
    let p = ply::parser::Parser::<ply::ply::DefaultElement>::new();

    // use the parser: read the entire file
    let ply = p.read_ply(&mut f);

    // make sure it did work
    assert!(ply.is_ok());
    let mut ply = ply.unwrap();

    ply.header.encoding = match matches.get_one::<String>("encoding").expect("required").as_str(){
        "be" => Encoding::BinaryBigEndian,
        "le" => Encoding::BinaryLittleEndian,
        _ => Encoding::Ascii,
    };

    let mut buf = std::fs::OpenOptions::new().write(true).truncate(true).open(&matches.get_one::<std::path::PathBuf>("file").expect("required")).unwrap();

    // set up a writer
    let w = Writer::new();
    let written = w.write_ply(&mut buf, &mut ply).unwrap();
    println!("{} bytes written", written);

}
