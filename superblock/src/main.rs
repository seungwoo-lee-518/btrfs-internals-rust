use std::{fs::File, io::Cursor, os::unix::prelude::FileExt};

use binrw::{BinRead, BinWrite};
use derivative::Derivative;

use crate::csum::get_hash;

mod csum;

#[derive(BinRead, BinWrite, Debug, Derivative, Clone)]
#[allow(dead_code)]
#[brw(little)]
pub struct Superblock {
    pub csum: [u8; 32],             // 32
    pub fsid: [u8; 16],             // 16
    pub bytenr: u64,                // 8
    pub flags: u64,                 // 8
    pub magic: [u8; 8],             // 8
    pub generation: u64,            // 8
    pub root_tree: u64,             // 8
    pub chunk_tree: u64,            // 8
    pub log_root: u64,              // 8
    pub log_root_transid: u64,      // 8
    pub total_bytes: u64,           // 8
    pub bytes_used: u64,            // 8
    pub root_dir_objectid: u64,     // 8
    pub num_devices: u64,           // 8
    pub sectorsize: u32,            // 4
    pub nodesize: u32,              // 4
    pub leafsize: u32,              // 4
    pub stripesize: u32,            // 4
    pub sys_chunk_array_size: u32,  // 4
    pub chunk_root_generation: u64, // 8
    pub compat_flags: u64,          // 8
    pub compat_ro_flags: u64,       // 8
    pub incompat_flags: u64,        // 8
    pub csum_type: u16,             // 2
    pub root_level: u8,             // 1
    pub chunk_root_level: u8,       // 1
    pub log_root_level: u8,         // 1
    pub dev_item: DevItem,
    pub label: [u8; 256],           // Labels
    pub padding: [u8; 3541]
}

#[derive(BinRead, BinWrite, Copy, Clone, Debug)]
#[brw(little)]
pub struct DevItem {
    pub dev_id: u64,                // 8
    pub dev_total_bytes: u64,       // 8
    pub dev_bytes_used: u64,        // 8
    pub dev_io_align: u32,          // 4
    pub dev_io_width: u32,          // 4
    pub dev_sector_size: u32,       // 4
    pub dev_type: u64,              // 8
    pub dev_generation: u64,        // 8
    pub dev_start_offset: u64,      // 8
    pub dev_group: u32,             // 4
    pub dev_seek_speed: u8,         // 1
    pub dev_bandwidth: u8,          // 1
    pub dev_uuid: [u8; 16],         // UUID
    pub dev_fsid: [u8; 16],         // FSID
}

/// Superblock Position for BTRFS
static BTRFS_SUPERBLOCK_POS: u64 = 0x10000;
const BTRFS_SUPERBLOCK_SIZE: usize = 4096;

fn main() {
    let f = match File::open("btrfs.img") {
        Ok(v) => v,
        Err(err) => {
            eprintln!("got error while read file: {err}");
            std::process::exit(1)
        }
    };

    let mut contents = [0u8; BTRFS_SUPERBLOCK_SIZE];
    
    if let Err(err) = f.read_exact_at(&mut contents, BTRFS_SUPERBLOCK_POS) {
        eprintln!("got error while read_exact_at: {err}");
        std::process::exit(1);
    }

    let superblock = match Superblock::read(&mut Cursor::new(contents)) {
        Ok(v) => v,
        Err(err) => {
            eprintln!("got error while read: {err}");
            std::process::exit(1)
        }
    };

    println!("superblock: {:?}", superblock);
    println!("superblock.fsid: {:02x?}", superblock.fsid);
    println!("superblock.dev.fsid: {:02x?}", superblock.dev_item.dev_fsid);
    println!("superblock.dev.uuid: {:?}", superblock.dev_item.dev_uuid);
    println!("csum: {:?}", get_hash(superblock));
}
