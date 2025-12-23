use crate::block_device::MEMDISK;

fn get_next_cluster(cluster: usize, fat_start: usize) {
    let bytes_per_sector = MEMDISK.block_size();
    // cluster is a unit btw :3
    let fat_offset = cluster * 4; // each fat entry is 4 bytes
    let fat_sector = fat_start + fat_offset / bytes_per_sector;
    let fat_index = fat_offset % bytes_per_sector;
}
