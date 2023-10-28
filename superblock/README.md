# Superblock

* Parsing BTRFS Superblock

## Usage

### (1): Create Block Device
```bash
$ fallocate -l 10G btrfs.img
$ mkfs.btrfs btrfs.img
```

### (2): Inspect it

```bash
$ cargo build --release
$ ./target/release/superblock
# seek success: 65536
# superblock: Superblock { csum: [108, 205, 23, 21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], fsid: [15, 98, 50, 67, 78, 212, 73, 198, 160, 253, 184, 69, 222, 124, 65, 147], bytenr: 65536, flags: 1, magic: [95, 66, 72, 82, 102, 83, 95, 77], generation: 6, root_tree: 30588928, chunk_tree: 22036480, log_tree: 0, log_root_transid: 0, total_bytes: 10737418240, bytes_used: 147456, root_dir_objectid: 6, num_devices: 1, sectorsize: 4096, nodesize: 16384, leafsize: 16384, stripesize: 4096, sys_chunk_array_size: 129, chunk_root_generation: 6, compat_flags: 0, compat_ro_flags: 3, incompat_flags: 833, csum_type: 0, root_level: 0, chunk_root_level: 0, log_root_level: 0, dev_id: 1, dev_total_bytes: 10737418240, dev_bytes_used: 562036736, dev_io_align: 4096, dev_io_width: 4096, dev_sector_size: 4096, dev_type: 0, dev_generation: 0, dev_start_offset: 0, dev_group: 0, dev_seek_speed: 0, dev_bandwidth: 0, dev_uuid: [26, 30, 153, 199, 210, 250, 73, 206, 148, 204, 109, 54, 130, 56, 70, 215], dev_fsid: [15, 98, 50, 67, 78, 212, 73, 198, 160, 253, 184, 69, 222, 124, 65, 147], label: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], cache_generation: 0, uuid_tree_generation: 0, ... }
# superblock.fsid: [f, 62, 32, 43, 4e, d4, 49, c6, a0, fd, b8, 45, de, 7c, 41, 93]
# superblock.dev.fsid: [f, 62, 32, 43, 4e, d4, 49, c6, a0, fd, b8, 45, de, 7c, 41, 93]
# superblock.dev.uuid: [1a, 1e, 99, c7, d2, fa, 49, ce, 94, cc, 6d, 36, 82, 38, 46, d7]
```

#### (2-1): Compare with `btrfs inspect-internal dump-super`

```bash
$ btrfs inspect-internal dump-super btrfs.img
# ...
```

## TODOs

- [ ] Implement `csum` functionality.
  - Seems like there are multiple hash is supoorted.
- [ ] Implement `sys_chunk_array` functionality.
- [ ] Implement `super_roots` functionality.

## Reference

* https://btrfs.readthedocs.io/en/latest/dev/On-disk-format.html
* https://github.com/util-linux/util-linux/blob/master/libblkid/src/superblocks/btrfs.c
