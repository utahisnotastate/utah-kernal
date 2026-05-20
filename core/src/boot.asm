.section .multiboot_header, "a"
.align 8

header_start:
    .long 0xe85250d6                       # magic number (multiboot2)
    .long 0                                # architecture 0 (i386 protected mode)
    .long header_end - header_start        # header length
    .long -(0xe85250d6 + 0 + (header_end - header_start)) # checksum

    # Required end tag (type = 0, flags = 0, size = 8)
    .short 0
    .short 0
    .long 8

header_end:
