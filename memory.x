/* Memory regions for the linker script */
/* Address map provided by datasheet: https://datasheets.raspberrypi.com/rp2040/rp2040-datasheet.pdf */
MEMORY {
    /* Define the memory region for the second stage bootloader */
    BOOT2 : ORIGIN = 0x10000000, LENGTH = 0x100

    /* Define the memory region for the application to be loaded next */
    FLASH : ORIGIN = 0x10000100, LENGTH = 2048K - 0x100

    /* Define the memory region for SRAM */
    RAM   : ORIGIN = 0x20000000, LENGTH = 264K
}

/* This is required for the bare metal and PAC bootloader */
SECTIONS {
  /* ### Boot loader */
  .boot_loader ORIGIN(BOOT2) :
  {
    KEEP(*(.boot_loader*));
  } > BOOT2

} INSERT BEFORE .text;
