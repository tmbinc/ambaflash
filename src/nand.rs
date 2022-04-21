use crate::cache;
use crate::static_ref::StaticRef;
use tock_registers::interfaces::{ReadWriteable, Readable, Writeable};
use tock_registers::register_structs;
use tock_registers::registers::ReadWrite;

register_structs! {
    FioRegisters {
        (0x0000 => fio_ctr: ReadWrite<u32>),
        (0x0004 => _reserved1),
        (0x0080 => fio_dmactr: ReadWrite<u32>),
        (0x0084 => fio_dmaadr: ReadWrite<u32>),
        (0x0088 => _reserved88),
        (0x008C => fio_dmasta: ReadWrite<u32>),
        (0x0090 => _reserved2),
        (0x00a0 => fio_a0: ReadWrite<u32>),
        (0x00a4 => fio_ecc_rpt_sta: ReadWrite<u32>),
        (0x00a8 => _reserved3),
        (0x0120 => flash_ctr: ReadWrite<u32>),
        (0x0124 => _reserved4),
        (0x0128 => flash_timing_0: ReadWrite<u32>),
        (0x012C => flash_timing_1: ReadWrite<u32>),
        (0x0130 => flash_timing_2: ReadWrite<u32>),
        (0x0134 => flash_timing_3: ReadWrite<u32>),
        (0x0138 => flash_timing_4: ReadWrite<u32>),
        (0x013C => flash_timing_5: ReadWrite<u32>),
        (0x0140 => _reserved5),
        (0x0150 => flash_int: ReadWrite<u32>),
        (0x0154 => _reserved6),
        (0x015c => fio_15c: ReadWrite<u32>),
        (0x0200 => @END),
    },
    FdmaRegisters {
        (0x000 => _reserved1),
        (0x200 => fdma_spr_cnt: ReadWrite<u32>),
        (0x204 => fdma_spr_src: ReadWrite<u32>),
        (0x208 => fdma_spr_dst: ReadWrite<u32>),
        (0x20C => fdma_spr_sta: ReadWrite<u32>),
        (0x210 => _reserved2),
        (0x300 => fdma_mn_ctrl: ReadWrite<u32>),
        (0x304 => fdma_mn_mem_addr: ReadWrite<u32>),
        (0x308 => fdma_dst: ReadWrite<u32>),
        (0x30c => fdma_mn_status: ReadWrite<u32>),
        (0x310 => _reserved3),
        (0x3A0 => fdma_dsm_ctrl: ReadWrite<u32>),
        (0x3a4 => _reserved4),
        (0x3F0 => fdma_int: ReadWrite<u32>),
        (0x3F4 => _reserved5),
        (0x400 => @END),
    }
}

const FIO_REGISTERS: StaticRef<FioRegisters> =
    unsafe { StaticRef::new((0xe0001000) as *const FioRegisters) };
const FDMA_REGISTERS: StaticRef<FdmaRegisters> =
    unsafe { StaticRef::new((0xE0012000) as *const FdmaRegisters) };

const SPARE_SIZE: u32 = 0x80;

pub fn nand_init() {
    // Setup flash timings.
    FIO_REGISTERS.flash_timing_0.set(0x5050805);
    FIO_REGISTERS.flash_timing_1.set(0x2020202);
    FIO_REGISTERS.flash_timing_2.set(0x5042708);
    FIO_REGISTERS.flash_timing_3.set(0x5042710);
    FIO_REGISTERS.flash_timing_4.set(0x8041800);
    FIO_REGISTERS.flash_timing_5.set(0x271804);

    // we have a hardcoded SPARE_SIZE here, but typically
    // this would come from RCT strap bit (ECC_SPARE_2X)

    let config_40 = 0x80000094;
    let config_80 = 0x80000095;

    FIO_REGISTERS.fio_a0.set(if SPARE_SIZE == 0x40 {
        config_40
    } else {
        config_80
    });
    FDMA_REGISTERS.fdma_dsm_ctrl.set(if SPARE_SIZE == 0x40 {
        config_40
    } else {
        config_80
    });
    FIO_REGISTERS
        .fio_15c
        .set(if SPARE_SIZE == 0x40 { 0 } else { 1 });
}

pub fn nand_read_block(
    numblocks: u32,
    blocksize: u32,
    dest: &mut [u8],
    eraseblock: u32,
    page: u32,
) {
    FIO_REGISTERS
        .fio_ctr
        .set(if SPARE_SIZE == 0x40 { 0x54 } else { 0x74 });
    FIO_REGISTERS.flash_ctr.set(0x7E00370);

    dest.fill(0xCC);
    cache::clean_d_cache_slice(dest);

    FDMA_REGISTERS.fdma_mn_mem_addr.set(dest.as_ptr() as u32);
    FDMA_REGISTERS
        .fdma_spr_src
        .set(dest[0x800..0x880].as_ptr() as u32);
    FDMA_REGISTERS.fdma_spr_cnt.set(numblocks * SPARE_SIZE);
    FDMA_REGISTERS.fdma_dst.set(dest.as_ptr() as u32);
    FDMA_REGISTERS
        .fdma_spr_dst
        .set(dest[0x800..0x880].as_ptr() as u32);
    FDMA_REGISTERS
        .fdma_mn_ctrl
        .set((0xAEC00000) | (numblocks * blocksize));

    FIO_REGISTERS
        .fio_dmaadr
        .set(((eraseblock << 6) + page) * blocksize);
    FIO_REGISTERS
        .fio_dmactr
        .set(((blocksize + SPARE_SIZE) * numblocks) | 0x86C00000);
    while FIO_REGISTERS.flash_int.get() & 1 == 0 {}
    while FIO_REGISTERS.fio_dmasta.get() & 0x1000000 == 0 {}
    while FDMA_REGISTERS.fdma_int.get() & 1 == 0 {}
    while FDMA_REGISTERS.fdma_mn_status.get() & 0x400000 == 0 {}
    FIO_REGISTERS.flash_int.set(0);
    FIO_REGISTERS.fio_ecc_rpt_sta.set(0);
    FIO_REGISTERS.fio_dmasta.set(0);
    FDMA_REGISTERS.fdma_int.set(0);
    FDMA_REGISTERS.fdma_mn_status.set(0);
    FDMA_REGISTERS.fdma_spr_sta.set(0);
}
