.globl _start
.extern LD_STACK_PTR

.section ".text.boot"

_start:

    // Enable the data and instruction cache
    mrs x0, SCTLR_EL1 // read SCTLR_EL1
    orr x0, x0, #(0x1 << 12) // set bit 12 to enable the I-cache
    orr x0, x0, #(0x1 << 2) // set bit 2 to enable the D-cache
    msr SCTLR_EL1, x0 // write back to SCTLR_EL1
    isb // isb to sync the change to memory

    ldr     x30, =LD_STACK_PTR
    mov     sp, x30
    bl      Reset

.global clean_d_cache
clean_d_cache:
ADD             X1, X1, X0
ADD             X1, X1, X0
mrs     x3, ctr_el0
LSR             X3, X3, #0x10
AND             X3, X3, #0xF
MOV             X2, #4
LSL             X2, X2, X3
SUB             X3, X2, #1
BIC             X0, X0, X3
1:
dc      cvac, x0

ADD             X0, X0, X2
CMP             X0, X1
B.CC            1b
DSB             SY
RET

.global _clean_flush_d_cache

_clean_flush_d_cache:
    stp     x8, x9, [sp, #-16]!
    dsb     sy
    mov     x0, #0x0                   
a:
    mrs     x7, clidr_el1
    lsl     x9, x0, #1
    add     x9, x9, x0
    lsr     x1, x7, x9
    and     x1, x1, #0x7
    cbz     x1, e
    cmp     x1, #0x2
    b.lt    d
    lsl     x9, x0, #1
    msr     csselr_el1, x9
    isb
    mrs     x7, ccsidr_el1
    and     x2, x7, #0x7
    add     x2, x2, #0x4
    lsr     x3, x7, #3
    and     x3, x3, #0x3ff
    clz     x5, x3
    sub     x5, x5, #0x20
    lsr     x4, x7, #13
    and     x4, x4, #0x7fff
b:
    mov     x6, x3
c:
    lsl     x8, x6, x5
    orr     x8, x8, x0, lsl #1
    lsl     x9, x4, x2
    orr     x8, x8, x9
    dc      cisw, x8
    subs    x6, x6, #0x1
    b.ge    c
    subs    x4, x4, #0x1
    b.ge    b
d:
    add     x0, x0, #0x1
    b       a
e:
    mov     x9, #0x0                   
    msr     csselr_el1, x9
    dsb     sy
    isb
    ldp     x8, x9, [sp], #16
    ret
