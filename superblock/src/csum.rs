use binrw::{io::{Cursor, Seek, SeekFrom}, BinWriterExt, BinReaderExt};
use anyhow::{Result, Ok};
use crate::Superblock;

#[allow(dead_code)]
/// Get Hash of the superblock
pub fn get_hash(superblock: Superblock) -> Result<u32> {
    let mut s = superblock.clone();
    s.csum = [0u8; 32];
    let mut writer = Cursor::new(Vec::new());
    writer.write_le(&s)?;
    let mut reader = writer;
    reader.seek(SeekFrom::Start(0))?;
    let b: [u8; 4096] = reader.read_le()?;
    // println!("{:?}, length={:?}", b, b.len());
    Ok(crc32fast::hash(&b))
}
