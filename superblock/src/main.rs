use std::{fs::File, io::{Seek, Read}};

// use binrw::BinRead;

// #[derive(BinRead)]
// #[allow(dead_code)]
// struct Superblock {
// }

/// Superblock Position for BTRFS
static BTRFS_SUPERBLOCK_POS: u64 = 0x10000;

fn main() {
    let mut f = match File::open("btrfs.img") {
        Ok(v) => v,
        Err(err) => {
            eprintln!("got error while read file: {err}");
            std::process::exit(1)
        }
    };

    let mut contents: Vec<u8> = Vec::with_capacity(20);

    match f.seek(std::io::SeekFrom::Start(BTRFS_SUPERBLOCK_POS)) {
        Ok(v) => {
            println!("seek success: {v}");
        },
        Err(err) => {
            eprintln!("got error while seek: {err}");
            std::process::exit(1);
        },
    };
    
    if let Err(err) = f.read_to_end(&mut contents) {
        eprintln!("got error while read: {err}");
        std::process::exit(1);
    }

    println!("{:?}", contents)
}
