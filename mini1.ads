# AMBAUSB: Init DRAM controller...

write 0xec1700dc,0x43100000
write 0xec170110,0x3f770000
write 0xec170114,0x00068304
write 0xec1700dc,0x43100001
write 0xec1700dc,0x43100000

#write 0xec170000,0x1f100000
#write 0xec170100,0x3F770000
#write 0xec170104,0x00068300
#write 0xec170000,0x1f100001
#write 0xec170000,0x1f100000
#
#write 0xec170264,0x21100000
#write 0xec17026c,0x3F710000
#write 0xec170270,0x00068300
#write 0xec170264,0x21100001
#write 0xec170264,0x21100000

write 0xec170090,0x00010220
write 0xec170094,0x00010220
write 0xec1700f0,0x00010220
write 0xec1700f4,0x00010220
write 0xec17023c,0x0042B9B1
write 0xec170240,0x0042B9B1
write 0xec170244,0x0042B9B1
write 0xec170248,0x0042B9B1
write 0xec170158,0x12e50000
usleep 2000000

write 0xec1706d0, 0x0000b2c1
write 0xec1706d4, 0x0000b2c1
write 0xec1706d8, 0x0000b2c1
write 0xec1706dc, 0x0000b2c1
usleep 2000000

write 0xdffe082c,0x00000100
write 0xdffe0814,0x00000008
poll 0xdffe0814,0x00000008,0x00000000

write 0xdffe0804,0x48182478
write 0xdffe0808,0xB3535D9E
write 0xdffe080c,0xA7F8B00D
write 0xdffe0810,0xA000018D
write 0xdffe0844,0x017C1600
write 0xdffe0830,0x000000E4
write 0xdffe083C,0x00003E44
write 0xdffe0820,0x000001A0
write 0xdffe0824,0x00020090
write 0xdffe0848,0x020F320F
write 0xdffe084c,0x00984000
write 0xdffe0838,0x00000320

write 0xdffe0800,0x60005208
usleep 1000
write 0xdffe0800,0x6000520C
usleep 1000
write 0xdffe0818,0x01020020
usleep 1000
poll 0xdffe0818,0x80000000,0x00000000
write 0xdffe0818,0x01030000
usleep 1000
poll 0xdffe0818,0x80000000,0x00000000
write 0xdffe0818,0x01010042
usleep 1000
poll 0xdffe0818,0x80000000,0x00000000
write 0xdffe0818,0x01000F15
usleep 1000
poll 0xdffe0818,0x80000000,0x00000000
write 0xdffe0800,0x2000520C
write 0xdffe0814,0x00000010
usleep 1000
poll 0xdffe0814,0x00000010,0x00000000
usleep 1000

write 0xdffe0800,0x4000520C
write 0xdffe0814,0x00000010
usleep 1000
poll 0xdffe0814,0x00000010,0x00000000
usleep 1000

write 0xdffe0800,0x6000520C
write 0xdffe0814,0x00000010
usleep 1000
poll 0xdffe0814,0x00000010,0x00000000
usleep 1000


write 0xdffe0814,0x00000020
usleep 1000
poll 0xdffe0814,0x00000020,0x00000000

write 0xdffe0814,0x00000004
usleep 1000
poll 0xdffe0814,0x00000004,0x00000000

write 0xdffe0820,0x000001a0
write 0xdffe0828,0x00000004
write 0xdffe083c,0x00003e44
write 0xdffe0838,0x00000320
write 0xdffe081C,0x00000001
write 0xdffe0800,0x6000520f
usleep 1000

#DRAM controller is initialized...
