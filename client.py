#4255:0001

import usb1, sys, struct

context = usb1.USBContext()
handle = context.openByVendorIDAndProductID(
    0x4255,
    0x0001
)

handle.claimInterface(0)

NUMBLOCKS = 0x10

def read_sector(addr):
    req = struct.pack("<BBIH", 0x55, 0, addr, NUMBLOCKS)
    handle.bulkWrite(1, req)
    r = handle.bulkRead(1, 0x880 * NUMBLOCKS)
    assert len(r) == 0x880 * NUMBLOCKS
    return r


with open("out", "wb") as fo:
    for i in range(0, 512 * 1024 // 2, NUMBLOCKS):
        print("reading sectors %08x" % i)
        fo.write(read_sector(i))
