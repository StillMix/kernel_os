ENTRY(_start)

SECTIONS {
    . = 1M;

    .multiboot_header : {
        *(.multiboot_header)
    }

    . = ALIGN(4096);
    .text : {
        *(.text*)
    }

    . = ALIGN(4096);
    .rodata : {
        *(.rodata*)
    }

    . = ALIGN(4096);
    .data : {
        *(.data*)
    }

    . = ALIGN(4096);
    .bss : {
        *(.bss*)
        *(COMMON)
    }
}