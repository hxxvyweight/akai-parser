use std::io;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};


fn main() -> std::io::Result<()> { 

    let mut buffer: [u8; 512] = [0; 512];

    let mut iso_file = File::open("../Akai CD-ROM Sound Library Volume 8.iso").expect("Failed to open file");

    let bytes_read = iso_file.read(&mut buffer).expect("Failed to read bytes");

    println!("Successfully read {} bytes!\n", bytes_read);
    println!("--- HEX DUMP ---");

    for byte in buffer {

        print!("{:02X} ", byte);
    }
    
    println!("\n-------");

    
    Ok(())




} 

