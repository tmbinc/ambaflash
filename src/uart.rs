use crate::static_ref::StaticRef;
use tock_registers::interfaces::{Readable, Writeable};
use tock_registers::register_bitfields;
use tock_registers::register_structs;
use tock_registers::registers::ReadWrite;

register_structs! {
    pub UartRegisters {
        (0x000 => rb_th_dll: ReadWrite<u32>),
        (0x004 => ie_dlh: ReadWrite<u32>),
        (0x008 => ii_fc: ReadWrite<u32>),
        (0x00C => lc: ReadWrite<u32>),
        (0x010 => mc: ReadWrite<u32>),
        (0x014 => ls: ReadWrite<u32, Ls::Register>),
        (0x018 => ms: ReadWrite<u32>),
        (0x01C => sc: ReadWrite<u32>),
    (0x020 => _reserved),
        (0x028 => dmae: ReadWrite<u32>),
    (0x02C => _reserved1),
        (0x040 => dmaf: ReadWrite<u32>),
    (0x044 => _reserved2),
        (0x07C => us: ReadWrite<u32>),
        (0x080 => tfl: ReadWrite<u32>),
        (0x084 => rfl: ReadWrite<u32>),
        (0x088 => srl: ReadWrite<u32>),
    (0x08C => _reserved3),
        (0x100 => @END),
    }
}

register_bitfields! [
    u32,
        Ls [
            FERR OFFSET(7) NUMBITS(1) [],
            TEMT OFFSET(6) NUMBITS(1) [],
            THRE OFFSET(5) NUMBITS(1) [],
            BI OFFSET(4) NUMBITS(1) [],
            FE OFFSET(3) NUMBITS(1) [],
            PE OFFSET(2) NUMBITS(1) [],
            OE OFFSET(1) NUMBITS(1) [],
            DR OFFSET(0) NUMBITS(1) [],
        ]
];

const UART0_BASE: StaticRef<UartRegisters> =
    unsafe { StaticRef::new(0xe800_5000 as *const UartRegisters) };

pub fn write_byte(byte: u8) {
    let uart_regs = &*UART0_BASE;
    uart_regs.rb_th_dll.set(byte as u32);
    while !uart_regs.ls.is_set(Ls::TEMT) {}
}
