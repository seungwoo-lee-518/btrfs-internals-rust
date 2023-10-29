use binrw::{io::Cursor, BinWriterExt};
use anyhow::Result;
use crate::Superblock;

#[allow(dead_code)]
/// Get Hash of the superblock
pub fn get_hash(superblock: Superblock) -> Result<u32> {
    let b = Vec::new();
    let mut s = superblock.clone();
    s.csum = [0u8; 32];
    let mut writer = Cursor::new(b.clone());
    writer.write_le(&s)?;
    Ok(crc32fast::hash(b.as_slice()))
}
