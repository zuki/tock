/* Memory layout for the STM32F407VG
 * Flash  = 1000KB (LENGTH = 0x00100000)
 * kernel = 512KB
 * user   = 512KB
 * ram    = 128KB */

MEMORY
{
  rom (rx)  : ORIGIN = 0x08000000, LENGTH = 0x00080000
  prog (rx) : ORIGIN = 0x08080000, LENGTH = 0x00100000
  ram (rwx) : ORIGIN = 0x20000000, LENGTH = 0x00020000
}

MPU_MIN_ALIGN = 8K;
PAGE_SIZE = 2K;
