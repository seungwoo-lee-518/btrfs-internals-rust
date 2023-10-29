use binrw::{io::Cursor, BinWriterExt};
use anyhow::{Result, Ok};
use crate::Superblock;

#[allow(dead_code)]
/// Get Hash of the superblock
pub fn get_hash(superblock: Superblock) -> Result<u32> {
    let mut s = superblock.clone();
    s.csum = [0; 32];
    let mut writer = Cursor::new(Vec::new());
    writer.write_le(&s)?;
    println!("{:?}", writer.clone().into_inner().as_slice());
    let mut v: Vec<u8> = writer.clone().into_inner();
    v.drain(0..32);
    let h = crc32c::crc32c(v.as_slice());
    println!("{:02x?}", h.to_le_bytes());
    Ok(0)
}
