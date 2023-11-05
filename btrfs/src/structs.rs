use binrw::{BinRead, BinWrite};
use derivative::Derivative;

#[derive(BinRead, BinWrite, Debug, Derivative, Clone, Copy)]
#[allow(dead_code)]
#[brw(little)]
/// Superblock information for BTRFS
///
/// Reference: https://github.com/kdave/btrfs-progs/blob/v6.5.3/libbtrfs/ctree.h#L461
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
    pub label: [u8; 256],            // Labels
    pub cache_generation: u64,       // 8
    pub uuid_tree_generation: u64,   // 8
    pub metadata_uuid: [u8; 16],
    pub nr_global_roots: u64,
    pub block_group_root: u64,
    pub block_group_root_generation: u64,
    pub block_group_root_level: u8,
    pub reserved: [u8; 7],
    pub reserved_u64: [u64; 24],
    pub sys_chunk_array: [u8; 2048],
    pub root_backups: [BtrfsRootBackup; 4],
    pub padding: [u8; 565]
}

#[allow(unused)]
#[derive(BinRead, BinWrite, Debug, Derivative, Clone, Copy)]
#[brw(little)]
pub struct BtrfsRootBackup {
    pub tree_root: u64,
    pub tree_root_gen: u64,
    pub chunk_root: u64,
    pub chunk_root_gen: u64,
    pub extent_root: u64,
    pub extent_root_gen: u64,
    pub fs_root: u64,
    pub fs_root_gen: u64,
    pub dev_root: u64,
    pub dev_root_gen: u64,
    pub csum_root: u64,
    pub csum_root_gen: u64,
    pub total_bytes: u64,
    pub bytes_used: u64,
    pub num_devices: u64,
    pub _unused: [u64; 4],
    pub tree_root_level: u8,
    pub chunk_root_level: u8,
    pub extent_root_level: u8,
    pub fs_root_level: u8,
    pub dev_root_level: u8,
    pub csum_root_level: u8,
    pub _unused_8: [u8; 10]
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

/// Offset of BTRFS Superblock
#[allow(dead_code)]
pub static BTRFS_SUPERBLOCK_OFFSET: u64 = 0x10000;

/// Size of the BTRFS Superblock
#[allow(dead_code)]
pub const BTRFS_SUPERBLOCK_SIZE: usize = 4096;

#[allow(dead_code)]
#[derive(PartialEq, Debug)]
#[repr(u16)]
pub enum CSUM {
    /// Default Hashing Algorithm of BTRFS
    /// Currently implementation only supports CRC32C
    CRC32C = 0,
    XXHASH = 1,
    SHA256 = 2,
    BLAKE2B = 3
}

#[allow(dead_code)]
impl CSUM {
    /// Check Algorihm is Supported on implementation
    pub fn supported(self) -> bool {
        return self == CSUM::CRC32C
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
/// CSUM Value type definitions
pub enum CSUMValue {
    CRC32C([u8; 4]),
    XXHASH([u8; 8]),
    SHA256([u8; 32]),
    BLAKE2B([u8; 32])
}

#[cfg(test)]
mod tests {
    use super::CSUM;

    #[test]
    fn test_supported() {
        let crc32c = CSUM::CRC32C;
        assert_eq!(crc32c.supported(), true);

        let xxhash = CSUM::XXHASH;
        assert_eq!(xxhash.supported(), false);
    }
}