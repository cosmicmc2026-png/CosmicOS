OUTPUT_FORMAT("elf64-x86-64")
OUTPUT_ARCH("i386:x86-64")
ENTRY(kmain)

/* Kernel loads at -2GiB (higher half) per Limine default */
KERNEL_BASE = 0xffffffff80000000;

PHDRS {
    text    PT_LOAD FLAGS((1 << 0) | (1 << 2));   /* R+X */
    rodata  PT_LOAD FLAGS((1 << 2));               /* R   */
    data    PT_LOAD FLAGS((1 << 1) | (1 << 2));   /* R+W */
}

SECTIONS {
    . = KERNEL_BASE;

    /* ── Executable code ───────────────────────────────────────────────── */
    .text : {
        *(.text._start)
        *(.text .text.*)
    } :text

    . = ALIGN(CONSTANT(MAXPAGESIZE));

    /* ── Read-only data + Limine requests ──────────────────────────────── */
    .rodata : {
        *(.rodata .rodata.*)
    } :rodata

    .requests : {
        KEEP(*(.requests_start_marker))
        KEEP(*(.requests))
        KEEP(*(.requests_end_marker))
    } :rodata

    . = ALIGN(CONSTANT(MAXPAGESIZE));

    /* ── Writable data ─────────────────────────────────────────────────── */
    .data : {
        *(.data .data.*)
        *(.got .got.plt)
    } :data

    .bss : {
        *(COMMON)
        *(.bss .bss.*)
    } :data

    /* ── Discard ────────────────────────────────────────────────────────── */
    /DISCARD/ : {
        *(.eh_frame)
        *(.note .note.*)
        *(.comment)
    }
}
