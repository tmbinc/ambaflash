import usb1, sys, struct, time

import argparse

context = usb1.USBContext()
handle = context.openByVendorIDAndProductID(
    0x4255,
    0x0001
)

if handle is None:
    print("AmbaFlash device not found; please run amba.py first!")
    exit()
handle.claimInterface(0)

# blocks per USB transfer; don't change this.
NUMBLOCKS = 0x10
FLASH_SIZE = 256 * 1024 * 1024
SECTOR_SIZE = 2048
SPARE_SIZE = 128

# eraseblock size of the flash
ERASEBLOCK_SIZE = 64 # 128k

def read_sector(addr):
    req = struct.pack("<BBIH", 0x55, 0, addr, NUMBLOCKS)
    handle.bulkWrite(1, req)
    r = handle.bulkRead(1, (SECTOR_SIZE + SPARE_SIZE) * NUMBLOCKS)
    assert len(r) == (SECTOR_SIZE + SPARE_SIZE) * NUMBLOCKS
    return r

def erase_sector(addr):
    req = struct.pack("<BBIH", 0x55, 1, addr, 0)
    handle.bulkWrite(1, req)
    r = handle.bulkRead(1, (SECTOR_SIZE + SPARE_SIZE) * NUMBLOCKS)

def program_pages(addr, data):
    req = struct.pack("<BBIH", 0x55, 2, addr, NUMBLOCKS)
    handle.bulkWrite(1, req)
    r = handle.bulkRead(1, (SECTOR_SIZE + SPARE_SIZE) * NUMBLOCKS)
    handle.bulkWrite(1, data)
    r = handle.bulkRead(1, (SECTOR_SIZE + SPARE_SIZE) * NUMBLOCKS)


def write_image(fn):
    with open(fn, "rb") as fi:
        for blk in range(0, FLASH_SIZE // SECTOR_SIZE, ERASEBLOCK_SIZE):
            data = fi.read((SECTOR_SIZE + SPARE_SIZE) * ERASEBLOCK_SIZE)
            assert len(data) == (SECTOR_SIZE + SPARE_SIZE) * ERASEBLOCK_SIZE, "Short input file"

            readback = b""
            for i in range(ERASEBLOCK_SIZE // NUMBLOCKS):
                readback += read_sector(blk + i * NUMBLOCKS)

            if readback == data:
                print("[%08x] *" % blk)
                continue

            erase_sector(blk)
            print("[%08x] REPROGRAM" % blk)
            for i in range(ERASEBLOCK_SIZE // NUMBLOCKS):
                program_pages(blk + i * NUMBLOCKS, data[i * NUMBLOCKS * (SECTOR_SIZE + SPARE_SIZE): (i+1) * NUMBLOCKS * (SECTOR_SIZE + SPARE_SIZE)])


parser = argparse.ArgumentParser()

parser.add_argument("--read")
parser.add_argument("--write")
args = parser.parse_args()

if args.write is not None:
    write_image(args.write)

if args.read:
    with open(args.read, "wb") as fo:
        for i in range(0, 256 * 1024 // 2, NUMBLOCKS):
            print("reading sectors %08x of %08x" % (i, 256 * 1024 // 2))
            fo.write(read_sector(i))

