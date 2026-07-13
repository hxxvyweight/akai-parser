use std::io;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};


fn main() -> std::io::Result<()> { 

    let mut buffer: [u8; 512] = [0; 512];

    let mut iso_file: std::fs::File = File::open("../test-file/Akai CD-ROM Sound Library Volume 8.iso")?;

    let target_position = SeekFrom::Start(2048);
    
    iso_file.seek(target_position)?;


    let bytes_read: usize = iso_file.read(&mut buffer)?;

    println!("Successfully read {} bytes!\n", bytes_read);
    println!("--- HEX DUMP ---");

    for (i, byte) in buffer.iter().enumerate() {

        print!("{:02X} ", byte);


        if (i + 1) % 16 == 0 {
            println!();
        }
    }
    
    println!("\n-------");

    
    Ok(())

} 

