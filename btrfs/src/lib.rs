mod structs;
mod csum;

use std::{fs::File, os::unix::prelude::FileExt};
use binrw::{io::Cursor, BinRead, BinWriterExt};
use structs::{Superblock, BTRFS_SUPERBLOCK_OFFSET, CSUM, CSUMValue};
use anyhow::Result;

#[allow(dead_code)]
impl Superblock {
    /// Create new Superblock from given file
    pub fn new(f: File) -> Result<Superblock> {
        let mut contents = [0u8; structs::BTRFS_SUPERBLOCK_SIZE];

        f.read_exact_at(&mut contents, BTRFS_SUPERBLOCK_OFFSET)?;

        let superblock = Superblock::read(&mut Cursor::new(contents))?;

        Ok(superblock)
    }
    
    /// Serialize Body Except CSUM
    pub fn serialize_body(self) -> Result<Vec<u8>> {
        let mut writer = Cursor::new(Vec::new());
        writer.write_le(&self)?;
        let mut v: Vec<u8> = writer.clone().into_inner();
        v.drain(0..32);
        Ok(v)
    }

    /// Get CSUM of the superblock
    pub fn get_csum(self) -> Option<CSUMValue> {
        let csum_algorithm = self.get_csum_algorithm();
        let v = self.csum.clone();
        match csum_algorithm {
            Some(CSUM::CRC32C) => {
                Some(CSUMValue::CRC32C(v[0..4].try_into().unwrap_or([0u8; 4])))
            },
            Some(CSUM::XXHASH) => {
                Some(CSUMValue::XXHASH(v[0..8].try_into().unwrap_or([0u8; 8])))
            }
            Some(CSUM::SHA256) => {
                Some(CSUMValue::SHA256(v))
            }
            Some(CSUM::BLAKE2B) => {
                Some(CSUMValue::BLAKE2B(v))
            }
            _ => None
        }
    }

    /// Get CSUM Algorithm of the superblock
    pub fn get_csum_algorithm(self) -> Option<CSUM> {
        match self.csum_type {
            0 => Some(CSUM::CRC32C),
            1 => Some(CSUM::XXHASH),
            2 => Some(CSUM::SHA256),
            3 => Some(CSUM::BLAKE2B),
            _ => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::panic;
    use std::fs;

    #[test]
    fn read_superblock() {
        // Read file
        let f = match fs::File::open("./btrfs.img") {
            Ok(v) => v,
            Err(err) => {
                panic!("got error while open file: {err}");
            }
        };

        let superblock = match Superblock::new(f) {
            Ok(v) => v,
            Err(err) => {
                panic!("got error while create superblock: {err}");
            }
        };

        println!("sys_chunk_array_size: {:?}", superblock.sys_chunk_array_size);

        println!("{:#?}", superblock.root_backups);

        let superblock_csum_alg = superblock.get_csum_algorithm();

        println!("algorithm: {:?}", superblock_csum_alg);

        if let Ok(v) = superblock.clone().serialize_body() {
            println!("serialized len: {:?}", v.len());
            assert!(v.len() == 4064); // 4K - 32Bytes
            let h = crc32c::crc32c(v.as_slice());
            println!("serialized crc32: {:02x?}", h.to_le_bytes());
        }

        assert!(superblock_csum_alg.is_some());
    }
}