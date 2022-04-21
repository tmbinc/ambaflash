// so shit code.

use crate::cache;
use crate::static_ref::StaticRef;
use core::cmp::min;
use core::ptr::addr_of;
use tock_registers::interfaces::{ReadWriteable, Readable, Writeable};
use tock_registers::register_bitfields;
use tock_registers::register_structs;
use tock_registers::registers::{InMemoryRegister, ReadWrite};
use tock_registers::LocalRegisterCopy;

// use crate::{debug, debug_hex32, debug_hex8};

fn debug(_text: &str) {}

fn debug_hex32(_val: u32) {}

fn debug_hex8(_val: u8) {}

register_structs! {
    UsbDevRegisters {
        (0x0000 => cfg: ReadWrite<u32, UsbDevCfg::Register>),
        (0x0004 => ctrl: ReadWrite<u32, UsbDevCtrl::Register>),
        (0x0008 => sts: ReadWrite<u32, UsbDevSts::Register>),
        (0x000c => intr: ReadWrite<u32, UsbDevIntr::Register>),
        (0x0010 => intr_msk: ReadWrite<u32, UsbDevIntMsk::Register>),
        (0x0014 => ep_intr: ReadWrite<u32>),
        (0x0018 => ep_intr_msk: ReadWrite<u32>),
        (0x001c => test_mode: ReadWrite<u32, UsbDevTestMd::Register>),
        (0x0020 => @END),
    },
    UsbEpInRegisters {
        (0x0000 => ctrl: ReadWrite<u32, UsbEpCtrl::Register>),
        (0x0004 => sts: ReadWrite<u32, UsbEpSts::Register>),
        (0x0008 => buf_sz: ReadWrite<u32>),
        (0x000c => max_pkt_sz: ReadWrite<u32, UsbEpMaxPktSz::Register>),
        (0x0010 => _reserved),
        (0x0014 => dat_desc_ptr: ReadWrite<u32>),
        (0x0018 => _reserved2),
        (0x001c => wr_cfm: ReadWrite<u32>),
        (0x0020 => @END),
    },
    UsbEpOutRegisters {
        (0x0000 => ctrl: ReadWrite<u32, UsbEpCtrl::Register>),
        (0x0004 => sts: ReadWrite<u32, UsbEpSts::Register>),
        (0x0008 => pkt_frm_num: ReadWrite<u32>),
        (0x000c => max_pkt_sz: ReadWrite<u32, UsbEpMaxPktSz::Register>),
        (0x0010 => setup_buf_ptr: ReadWrite<u32>),
        (0x0014 => dat_desc_ptr: ReadWrite<u32>),
        (0x0018 => _reserved),
        (0x001c => rd_cfm_zo_reg: ReadWrite<u32>),
        (0x0020 => @END),
    },
    RctRegisters {
        (0x0000 => _reserved),
        (0x0050 => ana_pwr_reg: ReadWrite<u32>),
        (0x0054 => _reserved1),
        (0x02CC => udc_soft_reset_reg: ReadWrite<u32>),
        (0x02D0 => _reserved2),
        (0x1000 => @END),
    },
    UdcRegisters {
        (0x0000 => udc_register: ReadWrite<u32, UsbUdc::Register>),
        (0x0004 => @END),
    }
}

#[repr(C, align(32))]
struct UsbSetupPkt {
    status: InMemoryRegister<u32, DmaStatus::Register>,
    reserved: InMemoryRegister<u32>,
    data0: InMemoryRegister<u32>,
    data1: InMemoryRegister<u32>,
}

impl UsbSetupPkt {
    fn initialize(&mut self) {
        self.status.write(DmaStatus::USB_DMA_BUF_STS::HostRdy);
        self.reserved.set(0xFFFFFFFF);
        self.data0.set(0xdeadbeef);
        self.data1.set(0xffffffff);
        cache::clean_d_cache(&self);
    }

    const fn const_default() -> Self {
        Self {
            status: InMemoryRegister::new(0),
            reserved: InMemoryRegister::new(0),
            data0: InMemoryRegister::new(0),
            data1: InMemoryRegister::new(0),
        }
    }
}

#[repr(C, align(32))]
struct UsbDataDesc {
    status: InMemoryRegister<u32, DmaStatus::Register>,
    reserved: InMemoryRegister<u32>,
    data_ptr: InMemoryRegister<u32>,
    next_desc_ptr: InMemoryRegister<u32>,
}

impl UsbDataDesc {
    fn initialize(&mut self, dma_addr: u32) {
        self.status
            .write(DmaStatus::USB_DMA_BUF_STS::DmaDone + DmaStatus::USB_DMA_LAST::SET);
        self.reserved.set(0xFFFFFFFF);
        self.data_ptr.set(dma_addr);
        self.next_desc_ptr.set(addr_of!(self) as u32);
        cache::clean_d_cache(&self);
    }

    const fn const_default() -> Self {
        Self {
            status: InMemoryRegister::new(0),
            reserved: InMemoryRegister::new(0),
            data_ptr: InMemoryRegister::new(0),
            next_desc_ptr: InMemoryRegister::new(0),
        }
    }
}

register_bitfields! [
    u32,
        UsbEpCtrl [
            USB_EP_STALL                OFFSET(0) NUMBITS(1) [],
            USB_EP_FLUSH                OFFSET(1) NUMBITS(1) [],
            USB_EP_SNOOP                OFFSET(2) NUMBITS(1) [],
            USB_EP_POLL_DEMAND          OFFSET(3) NUMBITS(1) [],
            USB_EP_TYPE                 OFFSET(4) NUMBITS(2) [
                Ctrl = 0,
                Iso = 1,
                Bulk = 2,
                Intr = 3,
            ],
            USB_EP_NAK_STS              OFFSET(6) NUMBITS(1) [],
            USB_EP_SET_NAK              OFFSET(7) NUMBITS(1) [],
            USB_EP_CLR_NAK              OFFSET(8) NUMBITS(1) [],
            USB_EP_RCV_RDY              OFFSET(9) NUMBITS(1) [],
        ],
        UsbEpSts [
            USB_EP_OUT_PKT_OUT         OFFSET(4) NUMBITS(1) [],
            USB_EP_OUT_PKT_SETUP       OFFSET(5) NUMBITS(1) [],
            USB_EP_IN_PKT               OFFSET(6) NUMBITS(1) [],
            USB_EP_BUF_NOT_AVAIL        OFFSET(7) NUMBITS(1) [],
            USB_EP_HOST_ERR             OFFSET(9) NUMBITS(1) [],
            USB_EP_TRN_DMA_CMPL         OFFSET(10) NUMBITS(1) [],
            USB_EP_RX_PKT_SZ            OFFSET(11) NUMBITS(12) [],
            USB_EP_RCV_CLR_STALL        OFFSET(25) NUMBITS(1) [],
            USB_EP_RCV_SET_STALL        OFFSET(26) NUMBITS(1) [],
            USB_EP_TXFIFO_EMPTY         OFFSET(27) NUMBITS(1) [],
        ],
        UsbEpBufSz [
            USB_EP_TXFIFO_DEPTH         OFFSET(16) NUMBITS(16) [],
            USB_EP_FRM_NUM        OFFSET(0) NUMBITS(16) [],

        ],
        UsbEpMaxPktSz [
            USB_EP_RXFIFO_DEPTH         OFFSET(16) NUMBITS(16) [],
            USB_EP_MAX_PKT_SZ           OFFSET(0) NUMBITS(16) [],

        ],
        UsbDevCfg [

            USB_DEV_SPD                 OFFSET(0) NUMBITS(2) [
                Hi = 0,
                Fu = 1,
                Lo = 2,
                Fu48 = 3,
            ],

            USB_DEV_REMOTE_WAKEUP_EN    OFFSET(2) NUMBITS(1) [],
            USB_DEV_POWER    OFFSET(3) NUMBITS(1) [
                Bus = 0,
                Sel = 1,
            ],
            USB_DEV_SYNC_FRM_EN    OFFSET(4) NUMBITS(1) [],

            USB_DEV_PHY OFFSET(5) NUMBITS(1) [
                Bit16 = 0,
                Bit8 = 1,
            ],

            USB_DEV_UTMI_DIR OFFSET(6) NUMBITS(1) [
                Uni = 0,
                Bi = 1,
            ],
            USB_DEV_STS_OUT_NONZERO OFFSET(7) NUMBITS(2) [ ],
            USB_DEV_PHY_ERR OFFSET(9) NUMBITS(1) [],
            USB_DEV_SPD_FU_TIMEOUT OFFSET(10) NUMBITS(3) [],
            USB_DEV_SPD_HI_TIMEOUT OFFSET(13) NUMBITS(3) [],
            USB_DEV_HALT OFFSET(16) NUMBITS(1) [
                Ack = 0,
                Stall = 1,
            ],
            USB_DEV_CSR_PRG_EN OFFSET(17) NUMBITS(1) [],
            USB_DEV_SET_DESC OFFSET(18) NUMBITS(1) [
                Stall = 0,
                Ack = 1,
            ],
            USB_DEV_DR OFFSET(19) NUMBITS(1) [
                Sdr = 0,
                Ddr = 1,
            ]
        ],
        UsbDevCtrl [
            USB_DEV_REMOTE_WAKEUP    OFFSET(0) NUMBITS(1) [],
            USB_DEV_RCV_DMA_EN    OFFSET(2) NUMBITS(1) [],
            USB_DEV_TRN_DMA_EN    OFFSET(3) NUMBITS(1) [],
            USB_DEV_DESC_UPD OFFSET(4) NUMBITS(1) [
                Pyl = 0,
                Pkt = 1
            ],
            USB_DEV_ENDN OFFSET(5) NUMBITS(1) [
                Little =0,
                Big = 1,
            ],
            USB_DEV_BUF_FIL_MD OFFSET(6) NUMBITS(1) [],
            USB_DEV_THRESH_EN OFFSET(7) NUMBITS(1) [],
            USB_DEV_BURST_EN OFFSET(8) NUMBITS(1) [],
            USB_DEV_DMA_MD OFFSET(9) NUMBITS(1) [],
            USB_DEV_SOFT_DISCON OFFSET(10) NUMBITS(1) [],
            USB_DEV_TIMER_SCALE_DOWN OFFSET(11) NUMBITS(1) [],
            USB_DEV_NAK OFFSET(12) NUMBITS(1) [],
            USB_DEV_CSR_DONE OFFSET(13) NUMBITS(1) [],
            USB_DEV_BURST_LEN OFFSET(16) NUMBITS(3) [],
            USB_DEV_THRESH_LEN OFFSET(24) NUMBITS(4) [],
        ],
        UsbDevSts
        [
            USB_DEV_CFG_NUM OFFSET(0) NUMBITS(4) [],
            USB_DEV_INTF_NUM OFFSET(4) NUMBITS(4) [],
            USB_DEV_ALT_SET OFFSET(8) NUMBITS(4) [],
            USB_DEV_SUSP_STS OFFSET(12) NUMBITS(1) [],
            USB_DEV_ENUM_SPD OFFSET(13) NUMBITS(2) [
                Hi = 0,
                Fu = 1,
                Lo = 2,
                Fu48 = 3,
            ],
            USB_DEV_RXFIFO_EMPTY_STS OFFSET(15) NUMBITS(1) [],
            USB_DEV_PHY_ERR_STS OFFSET(16) NUMBITS(1) [],
            USB_DEV_FRM_NUM OFFSET(18) NUMBITS(14) [],

        ],
        UsbDevIntr [
            USB_DEV_SET_CFG        OFFSET(0) NUMBITS(1) [],
            USB_DEV_SET_INTF    OFFSET(1) NUMBITS(1) [],
            USB_DEV_IDLE_3MS    OFFSET(2) NUMBITS(1) [],
            USB_DEV_RESET        OFFSET(3) NUMBITS(1) [],
            USB_DEV_SUSP        OFFSET(4) NUMBITS(1) [],
            USB_DEV_SOF        OFFSET(5) NUMBITS(1) [],
            USB_DEV_ENUM_CMPL    OFFSET(6) NUMBITS(1) [],
        ],
        UsbDevIntMsk [
            USB_DEV_MSK_SET_CFG    OFFSET(0) NUMBITS(1) [],
            USB_DEV_MSK_SET_INTF    OFFSET(1) NUMBITS(1) [],
            USB_DEV_MSK_IDLE_3MS    OFFSET(2) NUMBITS(1) [],
            USB_DEV_MSK_RESET    OFFSET(3) NUMBITS(1) [],
            USB_DEV_MSK_SUSP    OFFSET(4) NUMBITS(1) [],
            USB_DEV_MSK_SOF        OFFSET(5) NUMBITS(1) [],
            USB_DEV_MSK_SPD_ENUM_CMPL    OFFSET(6) NUMBITS(1) [],

        ],
        UsbDevEpIntr [
            USB_DEV_EP_IN        OFFSET(0) NUMBITS(6) [],

            USB_DEV_EP_OUT        OFFSET(16) NUMBITS(6) [],
        ],
        UsbDevEpIntrMsk [
            USB_DEV_MSK_EP_IN    OFFSET(0) NUMBITS(6) [],
            USB_DEV_MSK_EP_OUT    OFFSET(16) NUMBITS(6) [],

        ],
        UsbDevTestMd [
            USB_DEV_TEST_MD        OFFSET(0) NUMBITS(1) [],
        ],
        UsbUdc [
            USB_UDC_EP_NUM  OFFSET(0) NUMBITS(3) [
                Ep0 = 0,
                Ep1 = 1,
                Ep2 = 2,
                Ep3 = 3,
                Ep4 = 4,
                Ep5 = 5,
            ],

            USB_UDC_DIRECTION  OFFSET(4) NUMBITS(1) [
                Out = 0,
                In = 1
            ],

            USB_UDC_TYPE OFFSET(5) NUMBITS(2) [
                Ctrl = 0,
                Iso = 1,
                Bulk = 2,
                Intr = 3
            ],

            USB_UDC_CFG_NUM OFFSET(7) NUMBITS(1) [
            ],

            USB_UDC_MAX_PKT_SZ OFFSET(19) NUMBITS(13) [ ],
        ],

        DmaStatus [
            USB_DMA_RXTX_BYTES    OFFSET(0) NUMBITS(16) [] ,
            USB_DMA_CFG_STS OFFSET(0) NUMBITS(16) [],
            USB_DMA_CFG_NUM OFFSET(0) NUMBITS(16) [],
            USB_DMA_INTF_NUM    OFFSET(20) NUMBITS(4) [],
            USB_DMA_ALT_SET    OFFSET(16) NUMBITS(4) [],
            USB_DMA_FRM_NUM    OFFSET(16) NUMBITS(11) [],
            USB_DMA_LAST    OFFSET(27) NUMBITS(1) [],

            USB_DMA_RXTX_STS    OFFSET(28) NUMBITS(2) [
                Succ = 0,
                DesErr = 1,
                BufErr =3
            ] ,

            USB_DMA_BUF_STS OFFSET(30) NUMBITS(2) [
                HostRdy = 0,
                DmaBusy = 1,
                DmaDone = 2,
                HostBusy = 3
            ],

        ]

];

const USBDC_BASE: usize = 0xE0006000;

fn usb_ep_in_registers(ep_num: usize) -> StaticRef<UsbEpInRegisters> {
    let ep_in_base = USBDC_BASE + 0x20 * ep_num;
    unsafe { StaticRef::new(ep_in_base as *const UsbEpInRegisters) }
}

fn usb_ep_out_registers(ep_num: usize) -> StaticRef<UsbEpOutRegisters> {
    let ep_out_base = USBDC_BASE + 0x200 + 0x20 * ep_num;
    unsafe { StaticRef::new(ep_out_base as *const UsbEpOutRegisters) }
}

fn usb_dev_registers() -> StaticRef<UsbDevRegisters> {
    unsafe { StaticRef::new((USBDC_BASE + 0x400) as *const UsbDevRegisters) }
}

fn usb_udc_register(ep_num: usize) -> StaticRef<UdcRegisters> {
    let ep_out_base = USBDC_BASE + 0x504 + 4 * ep_num;
    unsafe { StaticRef::new(ep_out_base as *const UdcRegisters) }
}

fn rct_registers() -> StaticRef<RctRegisters> {
    unsafe { StaticRef::new((0xEC170000) as *const RctRegisters) }
}

#[derive(PartialEq)]
enum UsbSpeed {
    Hs = 0,
    _Fs = 1,
    _Ls = 2,
    _Fs48 = 3,
}

#[derive(PartialEq)]
enum Direction {
    In,
    Out,
}

#[derive(PartialEq)]
enum EndpointType {
    Ctrl,
    Iso,
    Bulk,
    Intr,
}

enum SetupRequest {
    GetStatus = 0,
    ClearFeature = 1,
    SetFeature = 3,
    SetAddress = 5,
    GetDescriptor = 6,
    SetDescriptor = 7,
    GetConfiguration = 8,
    SetConfiguration = 9,
    GetInterface = 10,
    SetInterface = 11,
    SynchFrame = 12,
    Unsupported = 100,
}

impl From<u32> for SetupRequest {
    fn from(num: u32) -> Self {
        match num {
            0 => SetupRequest::GetStatus,
            1 => SetupRequest::ClearFeature,
            3 => SetupRequest::SetFeature,
            5 => SetupRequest::SetAddress,
            6 => SetupRequest::GetDescriptor,
            7 => SetupRequest::SetDescriptor,
            8 => SetupRequest::GetConfiguration,
            9 => SetupRequest::SetConfiguration,
            10 => SetupRequest::GetInterface,
            11 => SetupRequest::SetInterface,
            12 => SetupRequest::SynchFrame,
            _ => SetupRequest::Unsupported,
        }
    }
}

struct UsbReceiveBuffers<'a> {
    data_read_buffer: Option<&'a mut [u8]>,
    data_write_buffer: Option<&'a [u8]>,
    data_read_index: usize,
    data_write_index: usize,
    data_read_done: bool,
    data_write_done: bool,
}

impl<'a> UsbReceiveBuffers<'a> {
    fn new() -> Self {
        UsbReceiveBuffers {
            data_read_buffer: None,
            data_write_buffer: None,
            data_read_index: 0,
            data_write_index: 0,
            data_read_done: true,
            data_write_done: true,
        }
    }
    fn set_data_read_buffer(&mut self, buf: Option<&'a mut [u8]>) -> Option<&'a mut [u8]> {
        let old = self.data_read_buffer.take();
        self.data_read_buffer = buf;
        self.data_read_index = 0;
        self.data_read_done = false;
        return old;
    }

    fn set_data_write_buffer(&mut self, buf: Option<&'a [u8]>) -> Option<&'a [u8]> {
        let old = self.data_write_buffer.take();
        self.data_write_buffer = buf;
        self.data_write_index = 0;
        self.data_write_done = false;
        return old;
    }

    fn need_more_read(&self) -> bool {
        match self.data_read_buffer {
            None => false,
            Some(ref v) => v.len() > self.data_read_index,
        }
    }
}

#[repr(align(32))]
struct DmaBuffer {
    buf: [u8; 512],
}

struct Endpoint {
    index: usize,
    ep_num: usize,
    direction: Direction,
    speed: UsbSpeed,
    ep_type: EndpointType,
    usb_setup_pkt: UsbSetupPkt,
    usb_data_desc: UsbDataDesc,
    dma_buf: DmaBuffer,
}

use crate::descr::*;

impl Endpoint {
    fn new(
        index: usize,
        ep_num: usize,
        direction: Direction,
        speed: UsbSpeed,
        ep_type: EndpointType,
    ) -> Self {
        Endpoint {
            index: index,
            ep_num: ep_num,
            direction: direction,
            speed: speed,
            ep_type: ep_type,
            usb_setup_pkt: UsbSetupPkt::const_default(),
            usb_data_desc: UsbDataDesc::const_default(),
            dma_buf: DmaBuffer { buf: [0; 512] },
        }
    }

    fn in_regs(&mut self) -> Option<StaticRef<UsbEpInRegisters>> {
        match self.direction {
            Direction::In => Some(usb_ep_in_registers(self.ep_num)),
            Direction::Out => None,
        }
    }

    fn out_regs(&mut self) -> Option<StaticRef<UsbEpOutRegisters>> {
        match self.direction {
            Direction::In => None,
            Direction::Out => Some(usb_ep_out_registers(self.ep_num)),
        }
    }

    fn init_in_endpoint(&mut self) {
        let regs_ep_in = self.in_regs().unwrap();

        debug("writing ep ctrl\n");

        regs_ep_in.ctrl.write(match self.ep_type {
            EndpointType::Ctrl => UsbEpCtrl::USB_EP_TYPE::Ctrl,
            EndpointType::Iso => UsbEpCtrl::USB_EP_TYPE::Iso,
            EndpointType::Bulk => UsbEpCtrl::USB_EP_TYPE::Bulk,
            EndpointType::Intr => UsbEpCtrl::USB_EP_TYPE::Intr,
        });

        // Set internal buffer size - 64 bytes for ctrl endpoint,
        // 512 for bulk endpoint.
        if self.index == 0 {
            regs_ep_in.buf_sz.set(64 / 4);
        } else {
            regs_ep_in.buf_sz.set(512 / 4);
        }

        regs_ep_in
            .max_pkt_sz
            .write(UsbEpMaxPktSz::USB_EP_MAX_PKT_SZ.val(64));

        debug("desc init\n");
        self.usb_data_desc
            .initialize(self.dma_buf.buf.as_ptr() as u32);
        debug("set desc ptr\n");
        regs_ep_in
            .dat_desc_ptr
            .set(addr_of!(self.usb_data_desc) as u32);
    }

    fn init_out_endpoint(&mut self) {
        debug("get regs ctrl\n");
        let regs_ep_out = self.out_regs().unwrap();

        debug("set ep out ctrl\n");
        regs_ep_out.ctrl.write(match self.ep_type {
            EndpointType::Ctrl => UsbEpCtrl::USB_EP_TYPE::Ctrl,
            EndpointType::Iso => UsbEpCtrl::USB_EP_TYPE::Iso,
            EndpointType::Bulk => UsbEpCtrl::USB_EP_TYPE::Bulk,
            EndpointType::Intr => UsbEpCtrl::USB_EP_TYPE::Intr,
        });

        regs_ep_out
            .max_pkt_sz
            .write(UsbEpMaxPktSz::USB_EP_MAX_PKT_SZ.val(64));

        // Specifically for the first CTRL OUT EP, we setup the SETUP PKT.
        if self.ep_type == EndpointType::Ctrl && self.ep_num == 0 {
            self.initialize_setup_pkt();
        }

        self.usb_data_desc.initialize(addr_of!(self.dma_buf) as u32);
        regs_ep_out
            .dat_desc_ptr
            .set(addr_of!(self.usb_data_desc) as u32);
    }

    fn init_endpoint(&mut self) {
        match self.direction {
            Direction::In => self.init_in_endpoint(),
            Direction::Out => self.init_out_endpoint(),
        }
    }

    fn set_endpoint_stall(&mut self, stall: bool) {
        /* this is obviously broken */
        match self.direction {
            Direction::In => self
                .in_regs()
                .unwrap()
                .ctrl
                .modify(UsbEpCtrl::USB_EP_SET_NAK.val(stall as u32)),
            Direction::Out => self
                .out_regs()
                .unwrap()
                .ctrl
                .modify(UsbEpCtrl::USB_EP_SET_NAK.val(stall as u32)),
        }
    }

    fn set_endpoint_nak(&mut self, nak: bool) {
        match self.direction {
            Direction::In => self.in_regs().unwrap().ctrl.modify(
                UsbEpCtrl::USB_EP_SET_NAK.val(nak as u32)
                    + UsbEpCtrl::USB_EP_CLR_NAK.val(!nak as u32),
            ),
            Direction::Out => self.out_regs().unwrap().ctrl.modify(
                UsbEpCtrl::USB_EP_SET_NAK.val(nak as u32)
                    + UsbEpCtrl::USB_EP_CLR_NAK.val(!nak as u32),
            ),
        }
    }

    fn get_descriptor(&mut self, ep_in: &mut Endpoint, setup_pkt: &[u8; 8]) {
        let w_value: u16 = u16::from_le_bytes(setup_pkt[2..4].try_into().unwrap());
        let _w_index: u16 = u16::from_le_bytes(setup_pkt[4..6].try_into().unwrap());
        let w_length: u16 = u16::from_le_bytes(setup_pkt[6..8].try_into().unwrap());

        let typ: u8 = ((w_value & 0xFF00) >> 8) as u8;
        let index: u8 = (w_value & 0xFF) as u8;

        let descriptor: Option<&[u8]> = match typ {
            1 => {
                // USB_DEVICE_DESCRIPTOR
                Some(&DATA_USB_DEVICE_DESCRIPTOR)
            }
            2 => {
                // USB_CONFIGURATION_DESCRIPTOR
                Some(&DATA_USB_CONFIGURATION_DESCRIPTOR)
            }
            3 => {
                // USB_STRING_DESCRIPTOR
                match index {
                    0 => Some(&DATA_USB_STRING_DESCRIPTOR_0),
                    1 => Some(&DATA_USB_STRING_DESCRIPTOR_1),
                    2 => Some(&DATA_USB_STRING_DESCRIPTOR_2),
                    3 => Some(&DATA_USB_STRING_DESCRIPTOR_3),
                    _ => {
                        debug("Unknown string descriptor\n");
                        None
                    }
                }
            }
            6 => {
                // USB_DEVICE_QUALIFIER
                Some(&DATA_USB_DEVICE_QUALIFIER)
            }
            _ => {
                debug("unknown descriptor type\n");
                None
            }
        };

        match descriptor {
            None => {
                debug("stalling\n");
                ep_in.set_endpoint_stall(true);
            }
            Some(descriptor) => {
                let tx_size = min(w_length as usize, descriptor.len());

                ep_in.dma_buf.buf[0..tx_size].clone_from_slice(&descriptor[0..tx_size]);
                ep_in.start_tx(tx_size as u32);
                self.start_rx();
            }
        }
    }

    fn start_tx(&mut self, len: u32) {
        self.usb_data_desc
            .data_ptr
            .set(addr_of!(self.dma_buf) as u32);
        debug("going to TX ");
        debug_hex32(len as u32);
        debug(" << numbytes\n");

        debug("OLD descriptor status: ");
        cache::clean_d_cache(&self.usb_data_desc);
        debug_hex32(self.usb_data_desc.status.get());
        debug("\n");

        self.usb_data_desc.status.write(
            DmaStatus::USB_DMA_BUF_STS::HostRdy
                + DmaStatus::USB_DMA_LAST::SET
                + DmaStatus::USB_DMA_RXTX_BYTES.val(len),
        );

        cache::clean_d_cache(&self.usb_data_desc);
        cache::clean_d_cache_slice(&self.dma_buf.buf);

        let regs_ep_in = self.in_regs().unwrap();
        self.set_endpoint_nak(false);

        // issue polling bit
        regs_ep_in.ctrl.modify(UsbEpCtrl::USB_EP_POLL_DEMAND::SET);
    }

    fn start_rx(&mut self) {
        self.usb_data_desc
            .data_ptr
            .set(addr_of!(self.dma_buf.buf) as u32);

        self.usb_data_desc.status.write(
            DmaStatus::USB_DMA_BUF_STS::HostRdy
                + DmaStatus::USB_DMA_LAST::SET
                + DmaStatus::USB_DMA_RXTX_BYTES.val(self.dma_buf.buf.len() as u32),
        );

        cache::clean_d_cache(&self.usb_data_desc);
        cache::clean_d_cache_slice(&self.dma_buf.buf);

        let usb_dev_regs = usb_dev_registers();

        usb_dev_regs
            .ctrl
            .modify(UsbDevCtrl::USB_DEV_RCV_DMA_EN::SET);

        self.set_endpoint_nak(false);
    }

    fn handle_vendor_request(&mut self, ep_in: &mut Endpoint, setup_pkt: &[u8; 8]) {
        let _w_value: u16 = u16::from_le_bytes(setup_pkt[2..4].try_into().unwrap());
        let _w_index: u16 = u16::from_le_bytes(setup_pkt[4..6].try_into().unwrap());
        let w_length: u16 = min(
            u16::from_le_bytes(setup_pkt[6..8].try_into().unwrap()),
            ep_in.dma_buf.buf.len() as u16,
        );

        /* if setup_pkt[0] & 0x80 == 0x80 {
            let tx_size = w_length as usize;

            let data_receiver = &mut ep_in.data_receiver.as_mut().unwrap();
            let tx_size_valid = data_receiver.read(setup_pkt, &mut ep_in.dma_buf.buf[0..tx_size]);
            ep_in.start_tx(tx_size_valid as u32);
            self.start_rx();
        } else {
            debug("receive .. dunno how exactly this works.\n");
            ep_in
                .data_receiver
                .as_mut()
                .unwrap()
                .prepare_write(setup_pkt);
            self.start_rx();
        } */
    }

    fn decode_request(&mut self, ep_in: &mut Endpoint) {
        let mut setup_pkt: [u8; 8] = [0; 8];

        setup_pkt[0..4].clone_from_slice(&mut self.usb_setup_pkt.data0.get().to_le_bytes());
        setup_pkt[4..8].clone_from_slice(&mut self.usb_setup_pkt.data1.get().to_le_bytes());

        debug("SETUP: ");
        debug_hex32(self.usb_setup_pkt.data0.get());
        debug(" ");
        debug_hex32(self.usb_setup_pkt.data1.get());
        debug("\n");

        if setup_pkt[0] == 0x80 && setup_pkt[1] == 0x06 {
            debug("GET DESCRTIPTOR\n");
            self.get_descriptor(ep_in, &setup_pkt);
        } else if (setup_pkt[0] & 0x40) == 0x40 {
            self.handle_vendor_request(ep_in, &setup_pkt);
        }

        self.initialize_setup_pkt();
    }

    fn handle_ep0_setup_packet(&mut self, ep_in: &mut Endpoint) {
        self.decode_request(ep_in);
    }

    fn initialize_setup_pkt(&mut self) {
        self.usb_setup_pkt.initialize();
        let regs_ep_out = self.out_regs().unwrap();

        regs_ep_out
            .setup_buf_ptr
            .set(addr_of!(self.usb_setup_pkt) as u32);
    }

    fn handle_ep0_out_packet(&mut self, ep_in: &mut Endpoint) {
        debug("EP0 out packet!\n");
        debug_hex32(self.usb_data_desc.status.get());
        debug(" <<< DESC status\n");

        if self
            .usb_data_desc
            .status
            .read(DmaStatus::USB_DMA_RXTX_BYTES)
            == 0
        {
            debug("SETUP done, reinit...!\n");
            self.initialize_setup_pkt();
            let usb_dev_regs = usb_dev_registers();
            usb_dev_regs
                .ctrl
                .modify(UsbDevCtrl::USB_DEV_RCV_DMA_EN::SET);
        } else {
            debug("NOT a setup packet.\n");

            let rx_data: &[u8] = &self.dma_buf.buf[0..self
                .usb_data_desc
                .status
                .read(DmaStatus::USB_DMA_RXTX_BYTES)
                as usize];

            match self
                .usb_data_desc
                .status
                .read_as_enum(DmaStatus::USB_DMA_BUF_STS)
            {
                Some(DmaStatus::USB_DMA_BUF_STS::Value::DmaDone) => {
                    debug("DMA DONE (on RX)!\n");
                    /* ep_in.data_receiver.as_mut().unwrap().write(rx_data); */

                    // re-setup etc.

                    self.initialize_setup_pkt();
                    let usb_dev_regs = usb_dev_registers();
                    usb_dev_regs
                        .ctrl
                        .modify(UsbDevCtrl::USB_DEV_RCV_DMA_EN::SET);

                    ep_in.start_tx(0);
                }
                _ => {
                    debug("other status\n");
                }
            }
        }
    }

    fn handle_ep0_out_interrupt(&mut self, ep_in: &mut Endpoint) {
        let regs_ep_out = self.out_regs().unwrap();

        let status = regs_ep_out.sts.extract();

        debug("EP0 EP OUT status: ");
        debug_hex32(status.get());
        debug("\n");

        if status.is_set(UsbEpSts::USB_EP_OUT_PKT_OUT) {
            debug("EP0 OUT packet\n");
            regs_ep_out.sts.write(UsbEpSts::USB_EP_OUT_PKT_OUT::SET);
            self.handle_ep0_out_packet(ep_in);
        }

        if status.is_set(UsbEpSts::USB_EP_OUT_PKT_SETUP) {
            regs_ep_out.sts.write(UsbEpSts::USB_EP_OUT_PKT_SETUP::SET);
            debug("EP0 SETUP packet\n");
            self.handle_ep0_setup_packet(ep_in);
        }
    }

    fn handle_ep0_in_interrupt(&mut self) {
        let regs_ep_in = self.in_regs().unwrap();

        let int_status = regs_ep_in.sts.extract();

        debug("EP0 EP IN status: ");
        debug_hex32(int_status.get());
        debug(" -> ");

        if int_status.is_set(UsbEpSts::USB_EP_TRN_DMA_CMPL) {
            debug("TRN DMA CMPL -> ");
            regs_ep_in.sts.write(UsbEpSts::USB_EP_TRN_DMA_CMPL::SET);

            debug("DESC STATUS: ");
            debug_hex32(self.usb_data_desc.status.get());
            debug("\n");
        }

        if int_status.is_set(UsbEpSts::USB_EP_IN_PKT) {
            debug("USB_EP_IN_PKT\n");
            regs_ep_in.sts.write(UsbEpSts::USB_EP_IN_PKT::SET);
        }

        if int_status.is_set(UsbEpSts::USB_EP_BUF_NOT_AVAIL) {
            debug("USB_EP_BUF_NOT_AVAIL\n");
            regs_ep_in.sts.write(UsbEpSts::USB_EP_BUF_NOT_AVAIL::SET);
        }

        if int_status.is_set(UsbEpSts::USB_EP_HOST_ERR) {
            debug("USB_EP_HOST_ERR\n");
            regs_ep_in.sts.write(UsbEpSts::USB_EP_HOST_ERR::SET);
        }
    }

    fn bulk_in_handler(&mut self, consumer: &mut UsbReceiveBuffers) {
        debug("BULK IN HANDLER\n");
        if consumer.data_write_index == consumer.data_write_buffer.unwrap().len() {
            debug("DONE with bulk in\n");
            self.set_endpoint_nak(true);
            consumer.data_write_done = true;
        } else {
            debug("not done, schedule next.\n");
            self.resume_transmit(consumer);
        }
    }

    fn process_rx_data(&mut self, rx_size: usize, consumer: &mut UsbReceiveBuffers) {
        let rx_data: &[u8] = &self.dma_buf.buf[0..rx_size];

        for x in rx_data {
            debug_hex8(*x);
        }
        debug("\n");

        if let Some(ref mut data_read_buffer) = consumer.data_read_buffer {
            debug("read index ");
            debug_hex32(consumer.data_read_index as u32);
            debug("\n");
            debug("read length ");
            debug_hex32(data_read_buffer.len() as u32);
            debug("\n");
            debug("rx size ");
            debug_hex32(rx_size as u32);
            debug("\n");

            let remaining = &mut data_read_buffer[consumer.data_read_index..];
            let remaining_rx_data = min(rx_data.len(), remaining.len());

            remaining[..remaining_rx_data].clone_from_slice(&rx_data[..remaining_rx_data]);

            if remaining_rx_data < rx_data.len() {
                debug("received more data than buffer size, ignoring remainder.\n");
            }

            consumer.data_read_index += remaining_rx_data;

            if consumer.data_read_index == data_read_buffer.len() {
                debug("data_read_done!\n");
                consumer.data_read_done = true;
                self.set_endpoint_nak(true);
            }
        } else {
            debug("received data without read buffer, ignoring. HALT.\n");
            loop {}
        }
    }

    fn handle_bulk_ep_out_pkt(&mut self, consumer: &mut UsbReceiveBuffers) {
        debug("BULK EP OUT PKT received\n");

        match self
            .usb_data_desc
            .status
            .read_as_enum(DmaStatus::USB_DMA_BUF_STS)
        {
            Some(DmaStatus::USB_DMA_BUF_STS::Value::DmaDone) => {
                debug("DMA DONE (on BULK RX)!\n");

                self.process_rx_data(
                    self.usb_data_desc
                        .status
                        .read(DmaStatus::USB_DMA_RXTX_BYTES) as usize,
                    consumer,
                );

                if consumer.need_more_read() {
                    // restart RX
                    self.start_rx();
                }

                let usb_dev_regs = usb_dev_registers();
                usb_dev_regs
                    .ctrl
                    .modify(UsbDevCtrl::USB_DEV_RCV_DMA_EN::SET);
            }
            _ => {
                debug("other status\n");
            }
        }
    }

    fn handle_bulk_interrupt(&mut self, consumer: &mut UsbReceiveBuffers) {
        debug("bulk interrupt\n");

        if self.direction == Direction::In {
            let regs_ep_in = self.in_regs().unwrap();

            let int_status = regs_ep_in.sts.extract();

            debug("BULK IN status: ");
            debug_hex32(int_status.get());
            debug("\n");

            if int_status.is_set(UsbEpSts::USB_EP_TRN_DMA_CMPL) {
                debug("  TRN DMA CMPL -> ");
                regs_ep_in.sts.write(UsbEpSts::USB_EP_TRN_DMA_CMPL::SET);

                debug("DESC STATUS: ");
                debug_hex32(self.usb_data_desc.status.get());
                debug("\n");
                if self.usb_data_desc.status.read(DmaStatus::USB_DMA_BUF_STS) != 2 {
                    debug("TRN COMPLETE BUT DESCR STATUS NOT DMADONE\n");
                    loop {}
                }
            }

            if int_status.is_set(UsbEpSts::USB_EP_IN_PKT) {
                debug("  USB_EP_IN_PKT\n");
                regs_ep_in.sts.write(UsbEpSts::USB_EP_IN_PKT::SET);

                self.bulk_in_handler(consumer);
            }

            if int_status.is_set(UsbEpSts::USB_EP_BUF_NOT_AVAIL) {
                debug("  USB_EP_BUF_NOT_AVAIL\n");
                regs_ep_in.sts.write(UsbEpSts::USB_EP_BUF_NOT_AVAIL::SET);
            }

            if int_status.is_set(UsbEpSts::USB_EP_HOST_ERR) {
                debug("  USB_EP_HOST_ERR\n");
                regs_ep_in.sts.write(UsbEpSts::USB_EP_HOST_ERR::SET);
            }

            if int_status.is_set(UsbEpSts::USB_EP_TXFIFO_EMPTY) {
                debug("  USB_EP_TXFIFO_EMPTY\n");
                regs_ep_in.sts.write(UsbEpSts::USB_EP_TXFIFO_EMPTY::SET);
            }

            debug("  REMAINING STATUS: ");
            debug_hex32(regs_ep_in.sts.get());
            debug("\n");
        } else {
            let regs_ep_out = self.out_regs().unwrap();

            let int_status = regs_ep_out.sts.extract();

            debug("BULK OUT status: ");
            debug_hex32(int_status.get());
            debug("\n");

            if int_status.is_set(UsbEpSts::USB_EP_TRN_DMA_CMPL) {
                debug("TRN DMA CMPL\n");
                regs_ep_out.sts.write(UsbEpSts::USB_EP_TRN_DMA_CMPL::SET);

                debug("DESC STATUS: ");
                debug_hex32(self.usb_data_desc.status.get());
                debug("\n");

                // self.done_size += self.usb_data_desc.usbr
            }

            if int_status.is_set(UsbEpSts::USB_EP_OUT_PKT_OUT) {
                debug("USB_EP_OUT_PKT\n");
                // handle received data
                self.handle_bulk_ep_out_pkt(consumer);
                regs_ep_out.sts.write(UsbEpSts::USB_EP_OUT_PKT_OUT::SET);
            }

            if int_status.is_set(UsbEpSts::USB_EP_BUF_NOT_AVAIL) {
                debug("USB_EP_BUF_NOT_AVAIL\n");
                regs_ep_out.sts.write(UsbEpSts::USB_EP_BUF_NOT_AVAIL::SET);
            }

            if int_status.is_set(UsbEpSts::USB_EP_HOST_ERR) {
                debug("USB_EP_HOST_ERR\n");
                regs_ep_out.sts.write(UsbEpSts::USB_EP_HOST_ERR::SET);
            }
        }
    }

    fn handle_interrupt(&mut self, ep_in: &mut Endpoint, consumer: &mut UsbReceiveBuffers) {
        // lol.
        cache::_clean_flush_d_cache();

        debug("EP interrupt -> ");
        if self.ep_num == 0 && self.direction == Direction::Out {
            debug("EP0 out interrupt! -> ");
            self.handle_ep0_out_interrupt(ep_in);
        } else if self.ep_num == 0 && self.direction == Direction::In {
            debug("EP0 in interrupt! -> ");
            self.handle_ep0_in_interrupt();
        } else if self.ep_num == 1 {
            self.handle_bulk_interrupt(consumer);
        } else {
            debug("EP interrupt for unknown endpoint\n");
        }
    }

    fn setup_udc(&mut self) {
        let usb_udc_register = usb_udc_register(self.index);
        usb_udc_register.udc_register.write(
            UsbUdc::USB_UDC_EP_NUM.val(self.ep_num as u32)
                + match self.direction {
                    Direction::In => UsbUdc::USB_UDC_DIRECTION::In,
                    Direction::Out => UsbUdc::USB_UDC_DIRECTION::Out,
                }
                + match self.ep_num {
                    0 => UsbUdc::USB_UDC_TYPE::Ctrl,
                    1 | _ => UsbUdc::USB_UDC_TYPE::Bulk,
                }
                + UsbUdc::USB_UDC_CFG_NUM::SET
                + UsbUdc::USB_UDC_MAX_PKT_SZ.val(64),
        );
    }

    fn resume_transmit(&mut self, consumer: &mut UsbReceiveBuffers) {
        let data_write_buffer = consumer.data_write_buffer.unwrap();

        let tx_size: usize = min(
            data_write_buffer.len() - consumer.data_write_index,
            self.dma_buf.buf.len(),
        );

        debug("resume transmit - index ");
        debug_hex32(consumer.data_write_index as u32);
        debug("\n");
        debug("resume transmit - len ");
        debug_hex32(data_write_buffer.len() as u32);
        debug("\n");
        debug("resume transmit - ");
        debug_hex32(tx_size as u32);
        debug("\n");

        self.dma_buf.buf[0..tx_size].clone_from_slice(
            &data_write_buffer[consumer.data_write_index..consumer.data_write_index + tx_size],
        );

        self.start_tx(tx_size as u32);

        consumer.data_write_index += tx_size;
    }
}

struct Usb {
    usb_ctrl_in: Endpoint,
    usb_ctrl_out: Endpoint,
    usb_bulk_in: Endpoint,
    usb_bulk_out: Endpoint,
    usb_dev_regs: StaticRef<UsbDevRegisters>,
    enum_complete: bool,
}

impl Usb {
    fn new() -> Self {
        Self {
            usb_ctrl_in: Endpoint::new(0, 0, Direction::In, UsbSpeed::Hs, EndpointType::Ctrl),
            usb_ctrl_out: Endpoint::new(1, 0, Direction::Out, UsbSpeed::Hs, EndpointType::Ctrl),
            usb_bulk_in: Endpoint::new(2, 1, Direction::In, UsbSpeed::Hs, EndpointType::Bulk),
            usb_bulk_out: Endpoint::new(3, 1, Direction::Out, UsbSpeed::Hs, EndpointType::Bulk),
            usb_dev_regs: usb_dev_registers(),
            enum_complete: false,
        }
    }

    fn set_softdisc(&mut self) {
        self.usb_dev_regs
            .ctrl
            .modify(UsbDevCtrl::USB_DEV_SOFT_DISCON::SET);
        self.usb_dev_regs
            .cfg
            .modify(UsbDevCfg::USB_DEV_REMOTE_WAKEUP_EN::SET);
    }

    fn initialize(&mut self) {
        debug("set softdisc");
        //self.set_softdisc();

        self.rct_reset();

        debug("init ctrl in\n");
        self.usb_ctrl_in.init_endpoint();
        debug("init ctrl out\n");
        self.usb_ctrl_out.init_endpoint();
        debug("init bulk in\n");
        self.usb_bulk_in.init_endpoint();
        debug("init bulk out\n");
        self.usb_bulk_out.init_endpoint();

        self.usb_dev_regs.cfg.write(
            UsbDevCfg::USB_DEV_SPD::Hi
                + UsbDevCfg::USB_DEV_POWER::Sel
                + UsbDevCfg::USB_DEV_PHY::Bit8
                + UsbDevCfg::USB_DEV_UTMI_DIR::Bi
                + UsbDevCfg::USB_DEV_HALT::Ack
                + UsbDevCfg::USB_DEV_SET_DESC::Stall
                + UsbDevCfg::USB_DEV_DR::Ddr
                + UsbDevCfg::USB_DEV_REMOTE_WAKEUP_EN::SET,
        );

        self.usb_dev_regs.ctrl.write(
            UsbDevCtrl::USB_DEV_RCV_DMA_EN::SET
                + UsbDevCtrl::USB_DEV_TRN_DMA_EN::SET
                + UsbDevCtrl::USB_DEV_DESC_UPD::Pyl
                + UsbDevCtrl::USB_DEV_ENDN::Little
                + UsbDevCtrl::USB_DEV_DMA_MD::SET,
        );

        self.usb_dev_regs.intr.set(0x0000007f);
        self.usb_dev_regs.ep_intr.set(0xffffffff);
        self.usb_dev_regs.intr_msk.set(0x0000007f);
        self.usb_dev_regs.ep_intr_msk.set(0xffffffff);

        self.usb_dev_regs.intr_msk.modify(
            UsbDevIntMsk::USB_DEV_MSK_SET_CFG::CLEAR
                + UsbDevIntMsk::USB_DEV_MSK_SET_INTF::CLEAR
                + UsbDevIntMsk::USB_DEV_MSK_SPD_ENUM_CMPL::CLEAR,
        );
        self.usb_dev_regs.ep_intr_msk.set(0xfff0fff0);

        self.usb_dev_regs
            .ctrl
            .modify(UsbDevCtrl::USB_DEV_REMOTE_WAKEUP::SET);
    }

    fn handle_ep_intr(
        &mut self,
        direction: Direction,
        ep_num: usize,
        consumer: &mut UsbReceiveBuffers,
    ) {
        if ep_num == 0 && direction == Direction::Out {
            self.usb_ctrl_out
                .handle_interrupt(&mut self.usb_ctrl_in, consumer);
        } else if ep_num == 0 && direction == Direction::In {
            self.usb_ctrl_in
                .handle_interrupt(&mut self.usb_ctrl_out, consumer);
        } else if ep_num == 1 && direction == Direction::Out {
            self.usb_bulk_out
                .handle_interrupt(&mut self.usb_bulk_in, consumer);
        } else if ep_num == 1 && direction == Direction::In {
            self.usb_bulk_in
                .handle_interrupt(&mut self.usb_bulk_out, consumer);
        } else {
            debug("Spurious EP interrupt!: ");
            debug_hex32(ep_num as u32);
            debug("\n");
        }
    }

    fn rct_reset(&mut self) {
        let rct_regs = rct_registers();

        rct_regs.ana_pwr_reg.set(rct_regs.ana_pwr_reg.get() | 4);
        debug("delay1");
        rct_regs
            .udc_soft_reset_reg
            .set(rct_regs.udc_soft_reset_reg.get() | 2);
        debug("delay2");
        rct_regs
            .udc_soft_reset_reg
            .set(rct_regs.udc_soft_reset_reg.get() & !2);
        debug("delay3");

        rct_regs.ana_pwr_reg.set(rct_regs.ana_pwr_reg.get() & !4);
        debug("delay4");
        //        rct_regs.ana_pwr_reg.set(rct_regs.ana_pwr_reg.get() | 0x3006);
    }

    fn handle_enum_cmpl(&mut self) {
        for ep in [
            &mut self.usb_ctrl_in,
            &mut self.usb_ctrl_out,
            &mut self.usb_bulk_in,
            &mut self.usb_bulk_out,
        ] {
            ep.setup_udc();
        }

        self.usb_dev_regs
            .cfg
            .modify(UsbDevCfg::USB_DEV_CSR_PRG_EN::SET);

        match self
            .usb_dev_regs
            .sts
            .read_as_enum(UsbDevSts::USB_DEV_ENUM_SPD)
        {
            Some(UsbDevSts::USB_DEV_ENUM_SPD::Value::Hi) => {
                debug("enum as high speed\n");

                self.usb_bulk_in
                    .in_regs()
                    .unwrap()
                    .max_pkt_sz
                    .write(UsbEpMaxPktSz::USB_EP_MAX_PKT_SZ.val(512));
                usb_udc_register(self.usb_bulk_in.index)
                    .udc_register
                    .modify(UsbUdc::USB_UDC_MAX_PKT_SZ.val(512));

                self.usb_bulk_out
                    .out_regs()
                    .unwrap()
                    .max_pkt_sz
                    .write(UsbEpMaxPktSz::USB_EP_MAX_PKT_SZ.val(512));
                usb_udc_register(self.usb_bulk_out.index)
                    .udc_register
                    .modify(UsbUdc::USB_UDC_MAX_PKT_SZ.val(512));
            }
            Some(UsbDevSts::USB_DEV_ENUM_SPD::Value::Fu) => {
                debug("enum as full speed - halt, not supported.\n");
                loop {}
            }
            _ => {
                debug("enum as unknown speed - halt, not supported\n");
                loop {}
            }
        }
    }

    fn handle_dev_set_cfg_set_intf(&mut self, consumer: &mut UsbReceiveBuffers) {
        let regs_ep_in = self.usb_ctrl_in.in_regs().unwrap();

        debug("set config/interface\n");

        regs_ep_in.ctrl.modify(UsbEpCtrl::USB_EP_STALL::CLEAR);

        let usb_dev_regs = usb_dev_registers();
        usb_dev_regs.ctrl.modify(UsbDevCtrl::USB_DEV_CSR_DONE::SET);

        // If reading data, and it was pending on enum, start it now.
        if consumer.need_more_read() {
            self.usb_bulk_out.start_rx();
        }

        self.enum_complete = true;

        self.usb_bulk_in
            .in_regs()
            .unwrap()
            .ctrl
            .modify(UsbEpCtrl::USB_EP_STALL::CLEAR);
        self.usb_bulk_out
            .out_regs()
            .unwrap()
            .ctrl
            .modify(UsbEpCtrl::USB_EP_STALL::CLEAR);
    }

    fn handle_device_interrupt(
        &mut self,
        intr: LocalRegisterCopy<u32, UsbDevIntr::Register>,
        consumer: &mut UsbReceiveBuffers,
    ) {
        if intr.is_set(UsbDevIntr::USB_DEV_SET_CFG) {
            debug("USB DEV SET CFG\n");
            self.handle_dev_set_cfg_set_intf(consumer);
        }

        if intr.is_set(UsbDevIntr::USB_DEV_SET_INTF) {
            debug("USB_DEV_SET_INTF\n");
            self.handle_dev_set_cfg_set_intf(consumer);
        }

        if intr.is_set(UsbDevIntr::USB_DEV_IDLE_3MS) {
            debug("USB_DEV_IDLE_3MS\n");
        }

        if intr.is_set(UsbDevIntr::USB_DEV_RESET) {
            debug("USB_DEV_RESET\n");
        }

        if intr.is_set(UsbDevIntr::USB_DEV_SUSP) {
            debug("USB_DEV_SUSP\n");
        }

        if intr.is_set(UsbDevIntr::USB_DEV_SOF) {
            debug("USB_DEV_SOF\n");
        }

        if intr.is_set(UsbDevIntr::USB_DEV_ENUM_CMPL) {
            self.handle_enum_cmpl();
            debug("USB_DEV_ENUM_CMPL\n");
        }
    }

    fn run(&mut self, consumer: &mut UsbReceiveBuffers) {
        while !consumer.data_read_done || !consumer.data_write_done {
            let intr = self.usb_dev_regs.intr.extract();
            let ep_intr = self.usb_dev_regs.ep_intr.extract();

            if intr.get() != 0 || ep_intr.get() != 0 {
                debug_hex32(intr.get());
                debug(" ");
                debug_hex32(ep_intr.get());
                debug("\n");
            }

            if intr.get() != 0 {
                self.usb_dev_regs.intr.set(intr.get());
                self.handle_device_interrupt(intr, consumer);
            }

            // Ack device interrupt
            self.usb_dev_regs.ep_intr.set(ep_intr.get());

            for ep_num in 0..6 {
                if ep_intr.get() & (1 << ep_num) != 0 {
                    self.handle_ep_intr(Direction::In, ep_num, consumer);
                }

                if ep_intr.get() & (0x10000 << ep_num) != 0 {
                    self.handle_ep_intr(Direction::Out, ep_num, consumer);
                }
            }
        }
    }
    fn read_data<'b>(&mut self, buf: &'b mut [u8]) {
        let mut consumer = UsbReceiveBuffers::new();
        consumer.set_data_read_buffer(Some(buf));

        // If enum is complete, no RX will be in progress, so start it.
        // If enum is not complete, RX will be started once enum is complete.
        if self.enum_complete {
            self.usb_bulk_out.start_rx();
        }
        self.run(&mut consumer);
    }

    fn write_data<'b>(&mut self, buf: &'b [u8]) {
        let mut consumer = UsbReceiveBuffers::new();
        consumer.set_data_write_buffer(Some(buf));
        // refill DMA buffer
        self.usb_bulk_in.resume_transmit(&mut consumer);
        self.run(&mut consumer);
    }
}

use crate::nand;

pub fn usb_test() {
    debug("Construct USB...\n");
    let mut usb: Usb = Usb::new();

    debug("init usb...\n");
    usb.initialize();

    let mut counter = 0_u32;
    let mut s_buf: [u8; 0x880 * 16] = [0xCC; 0x880 * 16];

    loop {
        let mut sync: [u8; 8] = [0; 8];
        usb.read_data(&mut sync);

        if sync[0] != 0x55 {
            debug("invalid sync, retry\n");
            continue;
        }

        let addr: u32 = u32::from_le_bytes(sync[2..6].try_into().unwrap());
        let numblocks: u16 = u16::from_le_bytes(sync[6..8].try_into().unwrap());

        debug("reading data from nand...\n");

        let dst_addr: u32 = 0x400000;
        let dst_slice =
            unsafe { core::slice::from_raw_parts_mut(dst_addr as *mut u8, 0x880 as usize) };

        for i in 0..(numblocks as usize) {
            debug("read ");
            debug_hex32(i as u32);
            debug("\n");
            debug("now\n");
            nand::nand_read_block(1, 0x800, dst_slice, 0, addr + (i as u32));
            debug("copy\n");
            s_buf[i * 0x880..(i + 1) * 0x880].clone_from_slice(dst_slice);
        }
        debug("done\n");
        usb.write_data(&s_buf);
        counter += 1;
    }
}
