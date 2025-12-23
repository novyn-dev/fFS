#![allow(non_snake_case)]
#![no_std]

use core::panic::PanicInfo;

pub mod cluster;
pub mod block_device;

// each partition entry is 16 bytes
#[repr(C, packed)]
pub struct PartitionEntry {
    boot_flag: u8,
    chs_begin: [u8; 3],
    type_code: u8,
    chs_end: [u8; 3],
    lba_begin: u32, // u8 * 4
    n_sectors: u32, // u8 * 4
}

#[panic_handler]
fn panic_handler(panic: &PanicInfo) -> ! {
    panic!("")
}
