pub static MEMDISK: DummyMemDisk<4096> = DummyMemDisk::new();

pub struct DummyMemDisk<const N: usize> {
    data: [u8; N],
}

impl<const N: usize> DummyMemDisk<N> {
    pub const fn new() -> Self {
        Self {
            data: [0; N]
        }
    }
    /// fun fact! bytes_per_sector is usually block_size in most case
    pub fn block_size(&self) -> usize { 512 } // bytes

    fn read(&self, buf: &mut [u8], lba: usize)
        -> Result<(), &str> {
        let start = lba * self.block_size();
        let end = start + buf.len();
        if end > self.data.len() {
            return Err("LBA out of range");
        }
        buf.copy_from_slice(&self.data[start..end]);

        Ok(())
    }

    fn write(&mut self, buf: &[u8], lba: usize)
        -> Result<(), &str> {
        let start = self.lba_start(lba);
        let end = start + buf.len();
        if end > self.data.len() {
            return Err("LBA out of range");
        }
        self.data.copy_from_slice(&buf[start..end]);

        Ok(())
    }

    fn lba_start(&self, lba: usize) -> usize {
        lba * self.block_size()
    }
}

#[cfg(test)]
mod tests {
    use crate::{block_device::DummyMemDisk, PartitionEntry};

    #[test]
    fn read_sector() {
        // dummy entry
        let entry = PartitionEntry {
            boot_flag: 0,
            chs_begin: [0; 3],
            type_code: 0,
            chs_end: [0; 3],
            lba_begin: 2, // read starting from sector 2
            n_sectors: 2, // read 2 sectors
        };

        let disk = DummyMemDisk::<1024>::new();
        let mut buf = [0u8; 1024]; // 2 * 512 kb
        let expected = [0u8; 1024];

        disk.read(&mut buf, entry.lba_begin as usize).unwrap();
        assert_eq!(buf, expected);
    }

    #[test]
    fn write_sector() {
        // dummy entry
        let entry = PartitionEntry {
            boot_flag: 0,
            chs_begin: [0; 3],
            type_code: 0,
            chs_end: [0; 3],
            lba_begin: 2, // read starting from sector 2
            n_sectors: 2, // read 2 sectors
        };

        let mut disk = DummyMemDisk::new();
        let buf = [255u8; 1024]; // 2 * 512 kb
        let expected = [255u8; 1024];

        disk.write(&buf, entry.lba_begin as usize).unwrap();
        assert_eq!(disk.data, expected)
    }
}
