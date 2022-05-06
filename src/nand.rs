use crate::static_ref::StaticRef;
use crate::{debug, debug_hex32};
use tock_registers::interfaces::{Readable, Writeable};
use tock_registers::registers::ReadWrite;
use tock_registers::{register_bitfields, register_structs};

register_structs! {
    FioRegisters {
        (0x0000 => fio_ctr: ReadWrite<u32>),
        (0x0004 => fio_sta: ReadWrite<u32, FioSta::Register>),
        (0x0008 => _reserved8),
        (0x0080 => fio_dmactr: ReadWrite<u32, FioDmactr::Register>),
        (0x0084 => fio_dmaadr: ReadWrite<u32>),
        (0x0088 => _reserved88),
        (0x008C => fio_dmasta: ReadWrite<u32, FioDmasta::Register>),
        (0x0090 => _reserved90),
        (0x00a0 => fio_dsm_ctr: ReadWrite<u32>),
        (0x00a4 => fio_ecc_rpt_sta: ReadWrite<u32>),
        (0x00a8 => _reserveda8),
        (0x0120 => flash_ctr: ReadWrite<u32>),
        (0x0124 => flash_cmd: ReadWrite<u32>),
        (0x0128 => flash_tim0: ReadWrite<u32>),
        (0x012C => flash_tim1: ReadWrite<u32>),
        (0x0130 => flash_tim2: ReadWrite<u32>),
        (0x0134 => flash_tim3: ReadWrite<u32>),
        (0x0138 => flash_tim4: ReadWrite<u32>),
        (0x013C => flash_tim5: ReadWrite<u32>),
        (0x0140 => flash_sta: ReadWrite<u32>),
        (0x0144 => flash_id: ReadWrite<u32>),
        (0x0148 => flash_cfi: ReadWrite<u32>),
        (0x014c => flash_len: ReadWrite<u32>),
        (0x0150 => flash_int: ReadWrite<u32>),
        (0x0154 => nand_read_cmdword_reg: ReadWrite<u32, Cmdword::Register>),
        (0x0158 => nand_prog_cmdword_reg: ReadWrite<u32, Cmdword::Register>),
        (0x015c => flash_ex_ctr: ReadWrite<u32>),
        (0x0160 => flash_ex_id: ReadWrite<u32>),
        (0x0164 => flash_tim6: ReadWrite<u32>),
        (0x0170 => flash_cc: ReadWrite<u32>),
        (0x0174 => flash_cc_word: ReadWrite<u32>),
        (0x0178 => _reserved178),
        (0x0180 => flash_cc_dat0: ReadWrite<u32>),
        (0x0184 => flash_cc_dat1: ReadWrite<u32>),
        (0x0188 => flash_cc_dat2: ReadWrite<u32>),
        (0x018C => flash_cc_dat3: ReadWrite<u32>),
        (0x0190 => flash_cc_dat4: ReadWrite<u32>),
        (0x0194 => flash_cc_dat5: ReadWrite<u32>),
        (0x0198 => flash_cc_dat6: ReadWrite<u32>),
        (0x019C => flash_cc_dat7: ReadWrite<u32>),
        (0x01A0 => _reserved1a0),

        (0x0200 => @END),
    },
    FdmaRegisters {
        (0x000 => _reserved1),
        (0x200 => fdma_spr_cnt: ReadWrite<u32>),
        (0x204 => fdma_spr_src: ReadWrite<u32>),
        (0x208 => fdma_spr_dst: ReadWrite<u32>),
        (0x20C => fdma_spr_sta: ReadWrite<u32>),
        (0x210 => _reserved2),
        (0x300 => fdma_ctr: ReadWrite<u32>),
        (0x304 => fdma_src: ReadWrite<u32>),
        (0x308 => fdma_dst: ReadWrite<u32>),
        (0x30c => fdma_sta: ReadWrite<u32>),
        (0x310 => _reserved3),
        (0x3A0 => fdma_dsm_ctrl: ReadWrite<u32>),
        (0x3a4 => _reserved4),
        (0x3F0 => fdma_int: ReadWrite<u32>),
        (0x3F4 => _reserved5),
        (0x400 => @END),
    }
}

register_bitfields! [
    u32,
        FioCtr [
            FIO_CTR_DA OFFSET(17) NUMBITS(1) [],
            FIO_CTR_DR OFFSET(16) NUMBITS(1) [],
            FIO_CTR_SX OFFSET(8) NUMBITS(1) [],
            FIO_CTR_RS OFFSET(4) NUMBITS(1) [],
            FIO_CTR_SE OFFSET(3) NUMBITS(1) [],
            FIO_CTR_CO OFFSET(2) NUMBITS(1) [],
            FIO_CTR_RR OFFSET(1) NUMBITS(1) [],
            FIO_CTR_XD OFFSET(0) NUMBITS(1) [],
        ],

        FioSta [
            FIO_STA_SI OFFSET(3) NUMBITS(1) [],
            FIO_STA_CI OFFSET(2) NUMBITS(1) [],
            FIO_STA_XI OFFSET(1) NUMBITS(1) [],
            FIO_STA_FI OFFSET(0) NUMBITS(1) [],
        ],

        FioDmactr [
            FIO_DMACTR_EN          OFFSET(31) NUMBITS(1) [],
            FIO_DMACTR_RM          OFFSET(30) NUMBITS(1) [],
            FIO_DMACTR_SD          OFFSET(28) NUMBITS(2) [ Fl=0, Xd=1, Cf=2, Sd = 3],
            FIO_DMACTR_BLK  OFFSET(24) NUMBITS(4) [
                Blk8b = 0,
                Blk16b = 1,
                Blk32b = 2,
                Blk64b = 3,
                Blk128b = 4,
                Blk256b = 5,
                Blk512b = 6,
                Blk1024b = 7,
                Blk2048b = 8,
                Blk4096b = 9,
                Blk8192b = 10,
                Blk16384b = 11,
            ],
            FIO_DMACTR_TS OFFSET(22) NUMBITS(2) [
                Ts1b = 0,
                Ts2b = 1,
                Ts4b = 2,
                Ts8b = 3,
            ],
            FIO_DMACTR_COUNT OFFSET(0) NUMBITS(22) [

            ]
        ],

        FioDmasta [
            FIO_DMASTA_RE OFFSET(26) NUMBITS(1) [],
            FIO_DMASTA_AE OFFSET(25) NUMBITS(1) [],
            FIO_DMASTA_DN OFFSET(24) NUMBITS(1) [],
        ],

        Cmdword [
            NAND_CMDWORD1 OFFSET(16) NUMBITS(8) [],
            NAND_CMDWORD2 OFFSET(0) NUMBITS(8) [],
        ]
];

const FIO_REGISTERS: StaticRef<FioRegisters> =
    unsafe { StaticRef::new((0xe0001000) as *const FioRegisters) };
const FDMA_REGISTERS: StaticRef<FdmaRegisters> =
    unsafe { StaticRef::new((0xE0012000) as *const FdmaRegisters) };

const SPARE_SIZE: u32 = 0x80;

pub fn nand_init() {
    // Setup flash timings.

    // From Mavic Mini 1 (loader):
    // FIO_REGISTERS.flash_tim0.set(0x5050805);
    // FIO_REGISTERS.flash_tim1.set(0x2020202);
    // FIO_REGISTERS.flash_tim2.set(0x5042708);
    // FIO_REGISTERS.flash_tim3.set(0x5042710);
    // FIO_REGISTERS.flash_tim4.set(0x8041800);
    // FIO_REGISTERS.flash_tim5.set(0x271804);

    // Dumpedfrom DJI Mini 2 at runtime:
    FIO_REGISTERS.flash_tim0.set(0x05050905);
    FIO_REGISTERS.flash_tim1.set(0x03030303);
    FIO_REGISTERS.flash_tim2.set(0x06052909);
    FIO_REGISTERS.flash_tim3.set(0x05052911);
    FIO_REGISTERS.flash_tim4.set(0x0A051900);
    FIO_REGISTERS.flash_tim5.set(0x00291805);

    // we have a hardcoded SPARE_SIZE here, but typically
    // this would come from RCT strap bit (ECC_SPARE_2X)

    let config_40 = 0x80000094; // FIO_DSM_EN | FIO_DSM_MAJP_2KB | FIO_DSM_SPJP_64B
    let config_80 = 0x80000095; // FIO_DSM_EN | FIO_DSM_MAJP_2KB | FIO_DSM_SPJP_128B

    FIO_REGISTERS.fio_dsm_ctr.set(if SPARE_SIZE == 0x40 {
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
        .flash_ex_ctr
        .set(if SPARE_SIZE == 0x40 { 0 } else { 1 });

    // Just disable WP and be done.
    nand_disable_wp();
}

pub fn nand_read_block(
    numblocks: u32,
    blocksize: u32,
    dest: &mut [u8],
    dest_spare: &mut [u8],
    flash_addr: usize,
) {
    FIO_REGISTERS.flash_int.set(0);
    FIO_REGISTERS.fio_ecc_rpt_sta.set(0);
    FIO_REGISTERS.fio_dmasta.set(0);
    FDMA_REGISTERS.fdma_int.set(0);
    FDMA_REGISTERS.fdma_sta.set(0);
    FDMA_REGISTERS.fdma_spr_sta.set(0);

    FIO_REGISTERS.fio_ctr.set(if SPARE_SIZE == 0x40 {
        0x54 | 0x80
    } else {
        0x74 | 0x80
    });

    FIO_REGISTERS.flash_ctr.set(0x7E00170);

    FDMA_REGISTERS.fdma_src.set(dest.as_ptr() as u32);
    FDMA_REGISTERS.fdma_spr_src.set(dest_spare.as_ptr() as u32);
    FDMA_REGISTERS.fdma_spr_cnt.set(numblocks * SPARE_SIZE);
    FDMA_REGISTERS.fdma_dst.set(dest.as_ptr() as u32);
    FDMA_REGISTERS.fdma_spr_dst.set(dest_spare.as_ptr() as u32);
    FDMA_REGISTERS
        .fdma_ctr
        .set((0xAEC00000) | (numblocks * blocksize));

    FIO_REGISTERS.fio_dmaadr.set(flash_addr as u32);
    FIO_REGISTERS.fio_dmactr.write(
        FioDmactr::FIO_DMACTR_COUNT.val((blocksize + SPARE_SIZE) * numblocks)
            + FioDmactr::FIO_DMACTR_EN::SET
            + FioDmactr::FIO_DMACTR_BLK::Blk512b
            + FioDmactr::FIO_DMACTR_TS::Ts8b,
    );
    while FIO_REGISTERS.flash_int.get() & 1 == 0 {}
    while FIO_REGISTERS.fio_dmasta.get() & 0x1000000 == 0 {}
    while FDMA_REGISTERS.fdma_int.get() & 1 == 0 {}
    while FDMA_REGISTERS.fdma_sta.get() & 0x400000 == 0 {}
    FIO_REGISTERS.flash_int.set(0);
    FIO_REGISTERS.fio_ecc_rpt_sta.set(0);
    FIO_REGISTERS.fio_dmasta.set(0);
    FDMA_REGISTERS.fdma_int.set(0);
    FDMA_REGISTERS.fdma_sta.set(0);
    FDMA_REGISTERS.fdma_spr_sta.set(0);
}

pub fn nand_disable_wp() {
    FIO_REGISTERS
        .flash_ctr
        .set(FIO_REGISTERS.flash_ctr.get() & !0x200);
}

pub fn nand_wait_cmd_done() {
    while FIO_REGISTERS.flash_int.get() & 1 == 0 {
        debug("flash int ");
        debug_hex32(FIO_REGISTERS.flash_int.get());
        debug("\r");
    }
    FIO_REGISTERS.flash_int.set(0);
}

pub fn nand_program(
    flash_address: usize,
    numblocks: usize,
    src_data: &[u8],
    src_spare: &[u8],
) -> Result<(), ()> {
    let data_size = numblocks * 0x800;
    let spare_size: usize = numblocks * (SPARE_SIZE as usize);

    FIO_REGISTERS.flash_int.set(0);
    FIO_REGISTERS.fio_ecc_rpt_sta.set(0);
    FIO_REGISTERS.fio_dmasta.set(0);
    FDMA_REGISTERS.fdma_int.set(0);
    FDMA_REGISTERS.fdma_sta.set(0);
    FDMA_REGISTERS.fdma_spr_sta.set(0);

    FIO_REGISTERS.fio_ctr.set(if SPARE_SIZE == 0x40 {
        0x54 | 0x80
    } else {
        0x74 | 0x80
    });

    FIO_REGISTERS.flash_ctr.set(0x7E00170);

    FIO_REGISTERS
        .nand_prog_cmdword_reg
        .write(Cmdword::NAND_CMDWORD1.val(0x80) + Cmdword::NAND_CMDWORD2.val(0x10));

    // Set source of spare data, target scratch space

    FDMA_REGISTERS.fdma_spr_sta.set(0);
    FDMA_REGISTERS.fdma_spr_cnt.set(spare_size as u32);
    FDMA_REGISTERS.fdma_spr_src.set(src_spare.as_ptr() as u32);
    FDMA_REGISTERS.fdma_spr_dst.set(0xe0030000);

    // Set source of data, target scratch space

    FDMA_REGISTERS.fdma_sta.set(0);
    FDMA_REGISTERS.fdma_src.set(src_data.as_ptr() as u32);
    FDMA_REGISTERS.fdma_dst.set(0xe0030000);
    FDMA_REGISTERS
        .fdma_ctr
        .set(0x18C00000 | 0x80000000 | 0x06000000 | (data_size as u32));

    // Set target for NAND write data.

    FIO_REGISTERS.fio_dmasta.set(0);
    FIO_REGISTERS.fio_dmaadr.set(flash_address as u32);

    FIO_REGISTERS.fio_dmactr.write(
        FioDmactr::FIO_DMACTR_COUNT.val((data_size + spare_size) as u32)
            + FioDmactr::FIO_DMACTR_EN::SET
            + FioDmactr::FIO_DMACTR_RM::SET
            + FioDmactr::FIO_DMACTR_BLK::Blk512b
            + FioDmactr::FIO_DMACTR_TS::Ts8b,
    );

    // Wait for DMA to be done.

    while FDMA_REGISTERS.fdma_int.get() & 1 == 0 {}

    // Wait for BUSY being de-asserted.

    nand_wait_cmd_done();

    nand_get_cmd_response();

    FIO_REGISTERS.flash_int.set(0);
    FIO_REGISTERS.fio_dmasta.set(0);
    FDMA_REGISTERS.fdma_int.set(0);
    FDMA_REGISTERS.fdma_sta.set(0);

    // Read status
    FIO_REGISTERS.flash_cmd.set(0xC);

    let status = nand_get_cmd_response();

    // Check for "Chip Status1" == PASS
    if (status & 1) != 0 {
        return Err(());
    } else {
        return Ok(());
    }
}

fn nand_get_cmd_response() -> u32 {
    while (FIO_REGISTERS.flash_cmd.get() & 0xF) != 0 {
        debug("*");
    }

    FIO_REGISTERS.flash_sta.get()
}

pub fn nand_erase_block(flash_addr: usize) -> Result<(), ()> {
    FIO_REGISTERS.fio_dmaadr.set(flash_addr as u32);
    FIO_REGISTERS.flash_int.set(0);
    FIO_REGISTERS
        .flash_ctr
        .set(FIO_REGISTERS.flash_ctr.get() & !0x30000000);

    FIO_REGISTERS
        .flash_cmd
        .set(((flash_addr as u32) & 0xFFFFFFF0) | 9);

    nand_wait_cmd_done();

    // Wait until BUSY is de-asserted.
    nand_get_cmd_response();

    // "READ STATUS"
    FIO_REGISTERS.flash_cmd.set(0xC); // readstatus
    let status = nand_get_cmd_response();

    debug("status1: ");
    debug_hex32(status);
    debug("\n");

    // Ensure "Chip Status 1" == Pass
    if (status & 1) != 0 {
        return Err(());
    } else {
        return Ok(());
    }
}
