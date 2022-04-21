ENTRY(_start)
SECTIONS
{
    . = 0x10000;
    .text.boot : { *(.text.boot) }
    .text : { *(.text*) }
    .data : { *(.data*) }
    .rodata : { *(.rodata*) }
    .bss : { 
        _sbss = .;
        *(.bss*) 
        _ebss = .;
    }

    . = ALIGN(256);
    . = . + 0x100000;
    LD_STACK_PTR = .;
}
