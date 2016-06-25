ENTRY(start)

SECTIONS {
	. = 1M;
	
	.boot :
	{
		/* ensure that the multiboot header is tat the beginning */
		KEEP(*(.multiboot_header))
	}

	.text :
	{
		*(.text)
	}
}
