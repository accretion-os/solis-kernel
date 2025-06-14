OUTPUT_FORMAT(elf64-x86-64)
ENTRY(ignis_entry)

/* Define headers to get the right MMU permissions and secure more linking process control */
PHDRS
{
    limine_requests PT_LOAD;
    text PT_LOAD;
    rodata PT_LOAD;
    data PT_LOAD;
}

SECTIONS
{
    /* Load the kernel in the topmost 2GiB of the address space */
    /* Required by the Limine specification and for optimizations */
    . = 0xffffffff80000000;

    /* Define the Limine requests container header */
    .limine_requests : {
        KEEP(*(.limine_requests_start))
        KEEP(*(.limine_requests))
        KEEP(*(.limine_requests_end))
    } :limine_requests

    /* Place the .text section in its own memory page */
    . = ALIGN(CONSTANT(MAXPAGESIZE));
    .text : {
        *(.text .text.*)
    } :text

    /* Place the .rodata section in its own memory page */
    . = ALIGN(CONSTANT(MAXPAGESIZE));
    .rodata : {
        *(.rodata .rodata.*)
    } :rodata

    /* Place the .data section in its own memory page */
    . = ALIGN(CONSTANT(MAXPAGESIZE));
    .data : {
        *(.data .data.*)
    } :data

    /* !!! .bss needs to be the last thing mapped to the :data header */
    /* If it's not, the linker will pad the binary with extra zeros */
    .bss : {
        *(.bss .bss.*)
        *(COMMON)
    } :data

    /* Discard .eh_frame* and .note.* to eliminate related issues on some hardware */
    /DISCARD/ : {
        *(.eh_frame*)
        *(.note .note.*)
    }
}