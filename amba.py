import usb1, struct, time, sys

class AmbaRom:
    def __init__(self):
        context = usb1.USBContext()

        self.handle = context.openByVendorIDAndProductID(
            0x4255,
            0x0010
        )

    def read32(self, addr):
        #print("< %08x" % addr)
        return int.from_bytes(self.handle.controlRead(0x40 | 0x80, 0, (addr >> 16) & 0xFFFF, addr & 0xFFFFF, 4), byteorder = 'little')

    def write32(self, addr, data):
        #print("> %08x %08x" % (addr, data))
        cmd = struct.pack("<III", 0x30000000, addr, data)
        self.handle.controlWrite(0x40 | 0x80, 0, 0, 0, cmd)

    def clear32(self, addr, mask):
        self.write32(addr, self.read32(addr) &~ mask)

    def set32(self, addr, mask):
        self.write32(addr, self.read32(addr) | mask)
    
    def rmw32(self, addr, mask, val):
        self.write32(addr, (self.read32(addr) &~ mask) | val)

    def call(self, addr):
        cmd = struct.pack("<III", 0x10000, addr, 0)
        self.handle.controlWrite(0x40 | 0x80, 0, 0, 0, cmd)

    def ramtest(self, start, end):
        for i in range(start, end, 4):
            self.write32(i, i)
        for i in range(start, end, 4):
            assert i == self.read32(i), "miscompare at %08x" % i

    def dump(self, file, addr, size):
        with open(file, "wb") as fo:
            for i in range(0, size, 4):
                fo.write(self.read32(addr + i).to_bytes(byteorder = 'little', length = 4))

    def upload(self, addr, file, maxlen = None):
        with open(file, "rb") as fi:
            while True:
                d = fi.read(4)
                if not d:
                    break
                v = int.from_bytes(d, byteorder = 'little')
                self.write32(addr, v)
                addr += 4
                if fi.tell() == maxlen:
                    break


rom = AmbaRom()

def init_gpios():
    GPIO_BASE = [0xe8009000, 0xe800a000, 0xe800e000, 0xe8010000, 0xe8011000]
    IOPAD_ADDR = [
        [0xE8015080, 0xE8015094, 0xEC170314, 0xEC170318], 
        [0xE8015084, 0xE8015098, 0xEC17031C, 0xEC170320],
        [0xE8015088, 0xE801509C, 0xEC170324, 0xEC170328],
        [0xE801508C, 0xE80150A0, 0xEC17032C, 0xEC170330],
        [0xE8015090, 0xE80150A4, 0xEC170438, 0xEC17043C]
    ]
    
    
    IOMUX_BASE = 0xE8016000
    
    GPIO_DATA = 0x00
    GPIO_DIR = 0x04
    GPIO_IS = 0x08
    GPIO_IBE = 0x10
    GPIO_IEV = 0x14
    GPIO_AFSEL = 0x18
    GPIO_RIS = 0x1C
    GPIO_MIS = 0x20
    GPIO_IC = 0x24
    GPIO_MASK = 0x28
    GPIO_ENABLE = 0x2C

    IOMUX_CTRL_SET = 0xF0
    

    GPIO_CONFIG = struct.unpack("<50I", bytes.fromhex("9E0000301E0003000000000000000000FFFFFFFF000000000000000000000000FFFFFFFF0000000003E001000003000000000000000000A0FFFFFFFF000000A00000000000000000FFFFFFFF000000000000000000FFFF7F0000000057000000FFFFFFFF030000000000000000000000FFFFFFFF0000000000000000F80701000000000000002000FFFFFFFF000000000000000000000000FFFFFFFF0000000000000000000000000000000000000000FFFFFFFF000000000000000000000000FFFFFFFF00000000"))


    for i in range(5):
        iomux0, iomux1, iomux2, dir, mask, data, pull_en, pull_dir, ds0, ds1 = GPIO_CONFIG[i*10:i*10+10]
        
        gpio = GPIO_BASE[i]
        rom.set32(gpio + GPIO_ENABLE, 1) 
        rom.write32(gpio + GPIO_AFSEL, 0)
        rom.write32(IOMUX_BASE + i * 12 + 0, iomux0)
        rom.write32(IOMUX_BASE + i * 12 + 4, iomux1)
        rom.write32(IOMUX_BASE + i * 12 + 8, iomux2)
        rom.set32(IOMUX_BASE + IOMUX_CTRL_SET, 1)
        rom.clear32(IOMUX_BASE + IOMUX_CTRL_SET, 1)
        
        rom.write32(gpio + GPIO_DIR, dir)
        rom.write32(gpio + GPIO_MASK, mask)
        rom.write32(gpio + GPIO_DATA, data)
        rom.write32(IOPAD_ADDR[i][0], pull_en)
        rom.write32(IOPAD_ADDR[i][1], pull_dir)
        rom.write32(IOPAD_ADDR[i][2], ds0)
        rom.write32(IOPAD_ADDR[i][3], ds1)
        rom.write32(gpio + GPIO_IC, 0xFFFFFFFF)


REF_CLK_FREQ = 24000000

def PLL_CTRL_INTPROG(x):
    return ((x >> 24) & 0x7F)
def PLL_CTRL_SOUT(x):
    return ((x >> 16) & 0xF)
def PLL_CTRL_SDIV(x):
    return ((x >> 12) & 0xF)
def PLL_CTRL_FRAC_MODE(x):
    return x & 0x8
def PLL_FRAC_VAL(f):
    return f & 0x7FFFFFFF
def PLL_FRAC_VAL_NEGA(f):
    return f & 0x80000000
def PLL_SCALER_JDIV(x):
    return (((x >> 4) & 0xF) + 1)

def readl(addr):
    return rom.read32(addr)



def rct_get_integer_pll_freq(c, pres, posts):

    if c & 0x20:
        return 0
    if c & 0x00200004:
        intprog = REF_CLK_FREQ
        intprog //= pres
        intprog //= posts
        return intprog
    intprog = PLL_CTRL_INTPROG(c) + 1
    sout = PLL_CTRL_SOUT(c) + 1
    sdiv = PLL_CTRL_SDIV(c) + 1

    intprog *= REF_CLK_FREQ
    intprog //= sout
    intprog *= sdiv
    intprog //= pres
    intprog //= posts

    return intprog

def rct_get_frac_pll_freq(c, f, pres, posts):
    if c & 0x20:
        return 0
    if c & 0x00200004:
        intprog = REF_CLK_FREQ
        intprog //= pres
        intprog //= posts
        return intprog

    intprog = PLL_CTRL_INTPROG(c) + 1
    sout = PLL_CTRL_SOUT(c) + 1
    sdiv = PLL_CTRL_SDIV(c) + 1
    frac = PLL_FRAC_VAL(f)

    intprog *= REF_CLK_FREQ
    intprog //= sout
    intprog *= sdiv
    intprog //= pres
    intprog //= posts

    if PLL_CTRL_FRAC_MODE(c):
        if PLL_FRAC_VAL_NEGA(f):
            frac = 0x80000000 - frac
        frac >>= 16
        frac *= (REF_CLK_FREQ >> 8)
        frac //= sout
        frac //= pres
        frac //= posts
        frac >>= 8
        frac *= sdiv

        if PLL_FRAC_VAL_NEGA(f):
            intprog -= frac
        else:
            intprog += frac
    return intprog

# / # busybox devmem 0xec170000
# 0x21101000
# / # busybox devmem 0xec1700dc
# 0x57100000
# / # busybox devmem 0xec170264
# 0x29100000
# / # busybox devmem 0xec170268
# 0x00000000
# / # busybox devmem 0xec1706f8
# 0x00000000
# / # busybox devmem 0xec1704ac
# 0x18000000


RCT_BASE = 0xEC170000
PLL_CORE_CTRL_REG = RCT_BASE + 0
PLL_DDR_CTRL_REG = RCT_BASE + 0xDC
PLL_CORTEX_CTRL_REG = RCT_BASE + 0x264
PLL_CORTEX_FRAC_REG = RCT_BASE + 0x268
PLL_NAND_CTRL_REG = RCT_BASE + 0x6F8
PLL_SD_CTRL_REG = RCT_BASE + 0x4AC

def get_core_bus_freq_hz():
    return rct_get_integer_pll_freq(readl(PLL_CORE_CTRL_REG), 1, 1) // 2

def get_ahb_bus_freq_hz():
    return get_core_bus_freq_hz() // 2

def get_apb_bus_freq_hz():
    return get_ahb_bus_freq_hz() // 2

def get_ddr_freq_hz():
    return rct_get_integer_pll_freq(readl(PLL_DDR_CTRL_REG), 1, 1) // 2

def get_cortex_freq_hz():
    return rct_get_frac_pll_freq(readl(PLL_CORTEX_CTRL_REG), readl(PLL_CORTEX_FRAC_REG), 1, 1)

def get_nand_freq_hz():
    return rct_get_integer_pll_freq(readl(PLL_NAND_CTRL_REG), 1, 1)

def get_sd_freq_hz():
    return rct_get_integer_pll_freq(readl(PLL_SD_CTRL_REG), 1, 1)

def show_clocks():
    print("CORE ", get_core_bus_freq_hz() // 1000 // 1000)
    print("AHB", get_ahb_bus_freq_hz() // 1000 // 1000)
    print("APB", get_apb_bus_freq_hz() // 1000 // 1000)
    print("DDR", get_ddr_freq_hz() // 1000 // 1000)
    print("CORTEX", get_core_bus_freq_hz() // 1000 // 1000)
    print("NAND", get_nand_freq_hz() // 1000 // 1000)
    print("SD", get_sd_freq_hz() // 1000 // 1000)


def clocks_setup(rom):
    rom.write32(0xDFFE0008, rom.read32(0xDFFE0008) & 0xFFFFFFF8 | 3)
    rom.write32(0xDFFE0080, 20)
    rom.write32(0xDFFE0084, 1794)
    rom.write32(0xDFFE0088, 5)
    rom.write32(0xDFFE008C, 6)
    rom.write32(0xDFFE0090, 5)
    rom.write32(0xDFFE0094, 0)
    rom.write32(0xDFFE0098, 5)
    rom.write32(0xDFFE009C, 5)
    rom.write32(0xDFFE00A0, 5)
    rom.write32(0xDFFE00A4, 5)
    rom.write32(0xDFFE00A8, 6)
    rom.write32(0xDFFE00AC, 6)
    rom.write32(0xDFFE00B0, 5)
    rom.write32(0xDFFE01FC, 512)
    rom.write32(0xDFFE0200, 480)
    
    rom.clear32(0xEC170264, 8)
    rom.clear32(0xEC170000, 8)
    rom.clear32(0xEC1700DC, 8)
    rom.clear32(0xEC1700E4, 8)
    rom.clear32(0xEC1704AC, 8)
    rom.clear32(0xEC1700E4, 0xF000)
    rom.clear32(0xEC1700E4, 0xFF000)
    rom.clear32(0xEC1701F4, 0xF0)

    rom.set32(0xec170054, 8)
    rom.set32(0xEC170748, 8)
    rom.set32(0xEC170024, 8)
    rom.set32(0xEC1706BC, 8)
    rom.set32(0xEC170164, 8)
    rom.set32(0xEC1700C0, 8)
    
    # set UART clock source
    rom.rmw32(0xec1701c8, 3, 1)

def init_serial(base):
    rom.write32(base + 4, 0)
    
    rom.write32(base + 0x88, 8)
    rom.write32(base + 0x88, 0)

    div = 48
    
    rom.write32(base + 0xC, 0x80)
    rom.write32(base + 0, div & 0xFF)
    rom.write32(base + 4, (div >> 8) & 0xFF)
    rom.write32(base + 0xC, 3)
    
    for i in b"HELLO\r\n":
        rom.write32(base + 0, i)


def run_ram_init(rom, ads):
    for i in open(ads):
        i = i.strip()
        if i.startswith("#"):
            continue
        
        if not i:
            continue
        cmd, args = i.split(None, 1)
        
        if cmd == "write":
            addr, data = [int(x, 0) for x in args.split(",")]
            #print(">>> %08x %08x" % (addr, data))
            rom.write32(addr, data)
        elif cmd == "usleep":
            time.sleep(int(args) / 1e6)
        elif cmd == "poll":
            addr, mask, data = [int(x, 0) for x in args.split(",")]
            #print("<<< %08x %08x %08x" % (addr, mask, data))
            while True:
                if rom.read32(addr) & mask == data:
                    break
        else:
            assert False, "unsupported command: %s" % i

def run_affr():
    init_gpios()
    clocks_setup(rom)

    uarts = [0xe8005000, 0xe0032000, 0xe0033000]

    # gclk-uart0 - 0xec170038:4 0xec1701c8:4
    #     rom.rmw32(0xEC1701C8, 3, 0)

    init_serial(uarts[0])

    print("[*] test basic communication")
    rom.ramtest(0xe0030000, 0xe0031000)

    print("[*] run DRAM init")
    run_ram_init(rom, "mini1.ads")
    print("[*] Verify RAM...")
    rom.ramtest(0, 0x1000)
    print("[*] upload payload")
    rom.upload(0x10000, "affr.bin")
    show_clocks()
    print("[*] call")
    rom.call(0x10000) # 000000)

# This does not work, unfortunately.
def run_nand():
    clocks_setup(rom)
    rom.upload(0xe0030000, sys.argv[1], 0x800)
    rom.call(0xe0030000)

#run_nand()
run_affr()
