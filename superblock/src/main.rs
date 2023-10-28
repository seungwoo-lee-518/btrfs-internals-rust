use std::{fs::File, io::{Seek, Read, Cursor}};

use binrw::BinRead;

#[derive(BinRead, Debug)]
#[allow(dead_code)]
#[br(little)]
struct Superblock {
    csum: [u8; 32],             // 32
    fsid: [u8; 16],             // 16
    bytenr: u64,                // 8
    flags: u64,                 // 8
    magic: [u8; 0x8],           // 8
    generation: u64,            // 8
    root_tree: u64,             // 8
    chunk_tree: u64,            // 8
    log_tree: u64,              // 8
    log_root_transid: u64,      // 8
    total_bytes: u64,           // 8
    bytes_used: u64,            // 8
    root_dir_objectid: u64,     // 8
    num_devices: u64,           // 8
    sectorsize: u32,            // 4
    nodesize: u32,              // 4
    leafsize: u32,              // 4
    stripesize: u32,            // 4
    sys_chunk_array_size: u32,  // 4
    chunk_root_generation: u64, // 8
    compat_flags: u64,          // 8
    compat_ro_flags: u64,       // 8
    incompat_flags: u64,        // 8
    csum_type: u16,             // 2
    root_level: u8,             // 1
    chunk_root_level: u8,       // 1
    log_root_level: u8,         // 1
}

/// Superblock Position for BTRFS
static BTRFS_SUPERBLOCK_POS: u64 = 0x10000;
static BTRFS_SUPERBLOCK_SIZE: usize = 4096;

fn main() {
    let mut f = match File::open("btrfs.img") {
        Ok(v) => v,
        Err(err) => {
            eprintln!("got error while read file: {err}");
            std::process::exit(1)
        }
    };

    let mut contents = Vec::with_capacity(BTRFS_SUPERBLOCK_SIZE);

    match f.seek(std::io::SeekFrom::Start(BTRFS_SUPERBLOCK_POS)) {
        Ok(v) => {
            println!("seek success: {v}");
        },
        Err(err) => {
            eprintln!("got error while seek: {err}");
            std::process::exit(1);
        },
    };

    if let Err(err) = f.take(BTRFS_SUPERBLOCK_SIZE as u64).read_to_end(&mut contents) {
        eprintln!("got error while read: {err}");
        std::process::exit(1);
    }

    println!("raw contents: {:?}, len={:?}", contents, contents.len());

    let superblock = match Superblock::read(&mut Cursor::new(contents)) {
        Ok(v) => v,
        Err(err) => {
            eprintln!("got error while read: {err}");
            std::process::exit(1)
        }
    };

    println!("superblock: {:?}", superblock);
}
