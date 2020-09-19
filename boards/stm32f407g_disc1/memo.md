1. kernel作成

```
$ cd boards/stm32f407g_disc1
$ make all
   Compiling tock-registers v0.5.0 (/Users/dspace/develop/rust/tock/libraries/tock-register-interface)
   Compiling tock-cells v0.1.0 (/Users/dspace/develop/rust/tock/libraries/tock-cells)
   Compiling enum_primitive v0.1.0 (/Users/dspace/develop/rust/tock/libraries/enum_primitive)
   Compiling tock-rt0 v0.1.0 (/Users/dspace/develop/rust/tock/libraries/tock-rt0)
   Compiling stm32f407g_disc1 v0.1.0 (/Users/dspace/develop/rust/tock/boards/stm32f407g_disc1)
   Compiling kernel v0.1.0 (/Users/dspace/develop/rust/tock/kernel)
   Compiling cortexm v0.1.0 (/Users/dspace/develop/rust/tock/arch/cortex-m)
   Compiling capsules v0.1.0 (/Users/dspace/develop/rust/tock/capsules)
   Compiling cortexm4 v0.1.0 (/Users/dspace/develop/rust/tock/arch/cortex-m4)
   Compiling stm32f4xx v0.1.0 (/Users/dspace/develop/rust/tock/chips/stm32f4xx)
   Compiling components v0.1.0 (/Users/dspace/develop/rust/tock/boards/components)
   Compiling stm32f407vg v0.1.0 (/Users/dspace/develop/rust/tock/chips/stm32f407vg)
    Finished release [optimized + debuginfo] target(s) in 19.44s
   text	   data	    bss	    dec	    hex	filename
  65537	   1992	   6200	  73729	  12001	/Users/dspace/develop/rust/tock/target/thumbv7em-none-eabihf/release/stm32f407g_disc1
044bfaf43f9e74ff90e1fa58a8ffdbb9ebe0c5e37c1bfe57c6b9bf1722fdb9ef  /Users/dspace/develop/rust/tock/target/thumbv7em-none-eabihf/release/stm32f407g_disc1.bin
```

2. kernel書き込み

```
$ make flash
    Finished release [optimized + debuginfo] target(s) in 0.03s
   text	   data	    bss	    dec	    hex	filename
  65537	   1992	   6200	  73729	  12001	/Users/dspace/develop/rust/tock/target/thumbv7em-none-eabihf/release/stm32f407g_disc1
openocd -f openocd.cfg -c "init; reset halt; flash write_image erase /Users/dspace/develop/rust/tock/target/thumbv7em-none-eabihf/release/stm32f407g_disc1.elf; verify_image /Users/dspace/develop/rust/tock/target/thumbv7em-none-eabihf/release/stm32f407g_disc1.elf; reset; shutdown"
Open On-Chip Debugger 0.10.0
Licensed under GNU GPL v2
For bug reports, read
	http://openocd.org/doc/doxygen/bugs.html
Info : auto-selecting first available session transport "hla_swd". To override use 'transport select <transport>'.
Info : The selected transport took over low-level target control. The results might differ compared to plain JTAG/SWD
adapter speed: 2000 kHz
adapter_nsrst_delay: 100
none separate
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : clock speed 1800 kHz
Info : STLINK v2 JTAG v37 API v2 SWIM v26 VID 0x0483 PID 0x374B
Info : using stlink api v2
Info : Target voltage: 2.903019
Info : stm32f4x.cpu: hardware has 6 breakpoints, 4 watchpoints
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
adapter speed: 1800 kHz
target halted due to debug-request, current mode: Thread
xPSR: 0x01000000 pc: 0x08006c6e msp: 0x20020000
auto erase enabled
Info : device id = 0x10076413
Info : flash size = 1024kbytes
Info : Padding image section 1 with 456760 bytes
target halted due to breakpoint, current mode: Thread
xPSR: 0x61000000 pc: 0x20000046 msp: 0x20020000
wrote 655360 bytes from file /Users/dspace/develop/rust/tock/target/thumbv7em-none-eabihf/release/stm32f407g_disc1.elf in 18.358028s (34.862 KiB/s)
target halted due to breakpoint, current mode: Thread
xPSR: 0x61000000 pc: 0x2000002e msp: 0x20020000
target halted due to breakpoint, current mode: Thread
xPSR: 0x61000000 pc: 0x2000002e msp: 0x20020000
verified 67529 bytes in 0.700694s (94.116 KiB/s)
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
adapter speed: 1800 kHz
shutdown command invoked
```

# app書き込みエラー

```
$ tockloader install --board stm32f407g-disc1 --openocd blink
[INFO   ] Could not find TAB named "blink" locally.

[0]	No
[1]	Yes

Would you like to check the online TAB repository for that app? [0] 1
[INFO   ] Using settings from KNOWN_BOARDS["stm32f407g-disc1"]
[STATUS ] Installing app on the board...
[ERROR  ] ERROR: openocd returned with error code 1
[INFO   ] Open On-Chip Debugger 0.10.0
Licensed under GNU GPL v2
For bug reports, read
	http://openocd.org/doc/doxygen/bugs.html
Info : auto-selecting first available session transport "hla_swd". To override use 'transport select <transport>'.
Info : The selected transport took over low-level target control. The results might differ compared to plain JTAG/SWD
adapter speed: 2000 kHz
adapter_nsrst_delay: 100
none separate
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : clock speed 1800 kHz
Info : STLINK v2 JTAG v37 API v2 SWIM v26 VID 0x0483 PID 0x374B
Info : using stlink api v2
Info : Target voltage: 2.903019
Error: jtag status contains invalid mode value - communication failure
Polling target stm32f4x.cpu failed, trying to reexamine
Examination failed, GDB will be halted. Polling again in 100ms
Info : Previous state query failed, trying to reconnect
Error: jtag status contains invalid mode value - communication failure
Polling target stm32f4x.cpu failed, trying to reexamine
Examination failed, GDB will be halted. Polling again in 300ms
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
adapter speed: 1800 kHz
Error: mem2array: Read @ 0xe0042004, w=4, cnt=1, failed
/usr/local/Cellar/open-ocd/0.10.0/bin/../share/openocd/scripts/mem_helper.tcl:6: Error:
in procedure 'reset'
in procedure 'ocd_bouncer'
in procedure 'ocd_process_reset'
in procedure 'ocd_process_reset_inner' called at file "embedded:startup.tcl", line 248
in procedure 'stm32f4x.cpu' called at file "embedded:startup.tcl", line 299
in procedure 'ocd_bouncer'
in procedure 'mmw'
in procedure 'mrw' called at file "/usr/local/Cellar/open-ocd/0.10.0/bin/../share/openocd/scripts/mem_helper.tcl", line 25
at file "/usr/local/Cellar/open-ocd/0.10.0/bin/../share/openocd/scripts/mem_helper.tcl", line 6

in procedure 'reset'
in procedure 'ocd_bouncer'



[ERROR  ] openocd error
```

```
$ tockloader list
[INFO   ] No device name specified. Using default name "tock".
[INFO   ] No serial port with device name "tock" found.
[INFO   ] Found 2 serial ports.
Multiple serial port options found. Which would you like to use?
[0]	/dev/cu.raspiw1-SerialPort-34 - n/a
[1]	/dev/cu.usbmodem14703 - STM32 STLink

Which option? [0] 1
[INFO   ] Using "/dev/cu.usbmodem14703 - STM32 STLink".
[ERROR  ] Error connecting to bootloader. No "pong" received.
[ERROR  ] Things that could be wrong:
[ERROR  ]   - The bootloader is not flashed on the chip
[ERROR  ]   - The DTR/RTS lines are not working
[ERROR  ]   - The serial port being used is incorrect
[ERROR  ]   - The bootloader API has changed
[ERROR  ]   - There is a bug in this script
[ERROR  ] Could not attach to the bootloader
Exception in thread Thread-1:
Traceback (most recent call last):
  File "/usr/local/Cellar/python@3.8/3.8.4/Frameworks/Python.framework/Versions/3.8/lib/python3.8/threading.py", line 932, in _bootstrap_inner
```


# tockloaderはまだ使えない？

stm32系のボードではアプリを組み込んだカーネルを作成して書き込むような感じになっているので
そのとおりにした。まずは`openocd.cfg`をこれまで単体で使っていた際のもので試したが、エラーが発生。

```
$ make program
    Finished release [optimized + debuginfo] target(s) in 0.03s
   text	   data	    bss	    dec	    hex	filename
  65537	   1992	   6200	  73729	  12001	/Users/dspace/develop/rust/tock/target/thumbv7em-none-eabihf/release/stm32f407g_disc1
arm-none-eabi-objcopy --update-section .apps=../../../libtock-c/examples/blink/build/cortex-m4/cortex-m4.tbf /Users/dspace/develop/rust/tock/target/thumbv7em-none-eabihf/debug/stm32f407g_disc1.elf /Users/dspace/develop/rust/tock/target/thumbv7em-none-eabihf/debug/stm32f407g_disc1-app.elf
openocd -f openocd.cfg -c "init; reset halt; flash write_image erase /Users/dspace/develop/rust/tock/target/thumbv7em-none-eabihf/debug/stm32f407g_disc1-app.elf; verify_image /Users/dspace/develop/rust/tock/target/thumbv7em-none-eabihf/debug/stm32f407g_disc1-app.elf; reset; shutdown"
Open On-Chip Debugger 0.10.0
Licensed under GNU GPL v2
For bug reports, read
	http://openocd.org/doc/doxygen/bugs.html
Info : auto-selecting first available session transport "hla_swd". To override use 'transport select <transport>'.
Info : The selected transport took over low-level target control. The results might differ compared to plain JTAG/SWD
adapter speed: 2000 kHz
adapter_nsrst_delay: 100
none separate
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : clock speed 1800 kHz
Info : STLINK v2 JTAG v37 API v2 SWIM v26 VID 0x0483 PID 0x374B
Info : using stlink api v2
Info : Target voltage: 2.901598
Error: jtag status contains invalid mode value - communication failure
Polling target stm32f4x.cpu failed, trying to reexamine
Examination failed, GDB will be halted. Polling again in 100ms
Info : Previous state query failed, trying to reconnect
Error: jtag status contains invalid mode value - communication failure
Polling target stm32f4x.cpu failed, trying to reexamine
Examination failed, GDB will be halted. Polling again in 300ms
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
adapter speed: 1800 kHz
Error: mem2array: Read @ 0xe0042004, w=4, cnt=1, failed
/usr/local/Cellar/open-ocd/0.10.0/bin/../share/openocd/scripts/mem_helper.tcl:6: Error:
in procedure 'reset'
in procedure 'ocd_bouncer'
in procedure 'ocd_process_reset'
in procedure 'ocd_process_reset_inner' called at file "embedded:startup.tcl", line 248
in procedure 'stm32f4x.cpu' called at file "embedded:startup.tcl", line 299
in procedure 'ocd_bouncer'
in procedure 'mmw'
in procedure 'mrw' called at file "/usr/local/Cellar/open-ocd/0.10.0/bin/../share/openocd/scripts/mem_helper.tcl", line 25
at file "/usr/local/Cellar/open-ocd/0.10.0/bin/../share/openocd/scripts/mem_helper.tcl", line 6

in procedure 'reset'
in procedure 'ocd_bouncer'


make: *** [program] Error 1
```

`openocd.cfg`を`tock-stm32`の`stm32f4discovery`ボードのそれに合わせたアプリの実行に
成功。ただし、まず上記のエラーが出たのち、再実行して成功している。

```
$ make program
    Finished release [optimized + debuginfo] target(s) in 0.03s
   text	   data	    bss	    dec	    hex	filename
  65537	   1992	   6200	  73729	  12001	/Users/dspace/develop/rust/tock/target/thumbv7em-none-eabihf/release/stm32f407g_disc1
arm-none-eabi-objcopy --update-section .apps=../../../libtock-c/examples/blink/build/cortex-m4/cortex-m4.tbf /Users/dspace/develop/rust/tock/target/thumbv7em-none-eabihf/debug/stm32f407g_disc1.elf /Users/dspace/develop/rust/tock/target/thumbv7em-none-eabihf/debug/stm32f407g_disc1-app.elf
openocd -f openocd.cfg -c "init; reset halt; flash write_image erase /Users/dspace/develop/rust/tock/target/thumbv7em-none-eabihf/debug/stm32f407g_disc1-app.elf; verify_image /Users/dspace/develop/rust/tock/target/thumbv7em-none-eabihf/debug/stm32f407g_disc1-app.elf; reset; shutdown"
Open On-Chip Debugger 0.10.0
Licensed under GNU GPL v2
For bug reports, read
	http://openocd.org/doc/doxygen/bugs.html
Info : The selected transport took over low-level target control. The results might differ compared to plain JTAG/SWD
adapter speed: 2000 kHz
adapter_nsrst_delay: 100
none separate
srst_only separate srst_nogate srst_open_drain connect_deassert_srst
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : clock speed 1800 kHz
Info : STLINK v2 JTAG v37 API v2 SWIM v26 VID 0x0483 PID 0x374B
Info : using stlink api v2
Info : Target voltage: 2.903019
Error: jtag status contains invalid mode value - communication failure
Polling target stm32f4x.cpu failed, trying to reexamine
Examination failed, GDB will be halted. Polling again in 100ms
Info : Previous state query failed, trying to reconnect
Error: jtag status contains invalid mode value - communication failure
Polling target stm32f4x.cpu failed, trying to reexamine
Examination failed, GDB will be halted. Polling again in 300ms
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
adapter speed: 1800 kHz
Error: mem2array: Read @ 0xe0042004, w=4, cnt=1, failed
/usr/local/Cellar/open-ocd/0.10.0/bin/../share/openocd/scripts/mem_helper.tcl:6: Error:
in procedure 'reset'
in procedure 'ocd_bouncer'
in procedure 'ocd_process_reset'
in procedure 'ocd_process_reset_inner' called at file "embedded:startup.tcl", line 248
in procedure 'stm32f4x.cpu' called at file "embedded:startup.tcl", line 299
in procedure 'ocd_bouncer'
in procedure 'mmw'
in procedure 'mrw' called at file "/usr/local/Cellar/open-ocd/0.10.0/bin/../share/openocd/scripts/mem_helper.tcl", line 25
at file "/usr/local/Cellar/open-ocd/0.10.0/bin/../share/openocd/scripts/mem_helper.tcl", line 6

Info : Previous state query failed, trying to reconnect
target halted due to debug-request, current mode: Thread
xPSR: 0x01000000 pc: 0x08008b7c msp: 0x20001000
Polling target stm32f4x.cpu failed, trying to reexamine
Info : stm32f4x.cpu: hardware has 6 breakpoints, 4 watchpoints
auto erase enabled
Info : device id = 0x10076413
Info : flash size = 1024kbytes
Info : Padding image section 1 with 432140 bytes
target halted due to breakpoint, current mode: Thread
xPSR: 0x61000000 pc: 0x20000046 msp: 0x20001000
wrote 655360 bytes from file /Users/dspace/develop/rust/tock/target/thumbv7em-none-eabihf/debug/stm32f407g_disc1-app.elf in 18.533251s (34.533 KiB/s)
target halted due to breakpoint, current mode: Thread
xPSR: 0x61000000 pc: 0x2000002e msp: 0x20001000
target halted due to breakpoint, current mode: Thread
xPSR: 0x61000000 pc: 0x2000002e msp: 0x20001000
target halted due to breakpoint, current mode: Thread
xPSR: 0x61000000 pc: 0x2000002e msp: 0x20001000
verified 94196 bytes in 0.928092s (99.115 KiB/s)
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
adapter speed: 1800 kHz
shutdown command invoked
```

そこで、アプリを変えて`make program`したら今度は最初から成功した。

```
$ make program
    Finished release [optimized + debuginfo] target(s) in 0.03s
   text	   data	    bss	    dec	    hex	filename
  65537	   1992	   6200	  73729	  12001	/Users/dspace/develop/rust/tock/target/thumbv7em-none-eabihf/release/stm32f407g_disc1
arm-none-eabi-objcopy --update-section .apps=../../../libtock-c/examples/buttons/build/cortex-m4/cortex-m4.tbf /Users/dspace/develop/rust/tock/target/thumbv7em-none-eabihf/debug/stm32f407g_disc1.elf /Users/dspace/develop/rust/tock/target/thumbv7em-none-eabihf/debug/stm32f407g_disc1-app.elf
openocd -f openocd.cfg -c "init; reset halt; flash write_image erase /Users/dspace/develop/rust/tock/target/thumbv7em-none-eabihf/debug/stm32f407g_disc1-app.elf; verify_image /Users/dspace/develop/rust/tock/target/thumbv7em-none-eabihf/debug/stm32f407g_disc1-app.elf; reset; shutdown"
Open On-Chip Debugger 0.10.0
Licensed under GNU GPL v2
For bug reports, read
	http://openocd.org/doc/doxygen/bugs.html
Info : The selected transport took over low-level target control. The results might differ compared to plain JTAG/SWD
adapter speed: 2000 kHz
adapter_nsrst_delay: 100
none separate
srst_only separate srst_nogate srst_open_drain connect_deassert_srst
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : clock speed 1800 kHz
Info : STLINK v2 JTAG v37 API v2 SWIM v26 VID 0x0483 PID 0x374B
Info : using stlink api v2
Info : Target voltage: 2.903019
Info : stm32f4x.cpu: hardware has 6 breakpoints, 4 watchpoints
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
adapter speed: 1800 kHz
target halted due to debug-request, current mode: Thread
xPSR: 0x01000000 pc: 0x08002144 msp: 0x20001000
auto erase enabled
Info : device id = 0x10076413
Info : flash size = 1024kbytes
Info : Padding image section 1 with 432140 bytes
target halted due to breakpoint, current mode: Thread
xPSR: 0x61000000 pc: 0x20000046 msp: 0x20001000
wrote 655360 bytes from file /Users/dspace/develop/rust/tock/target/thumbv7em-none-eabihf/debug/stm32f407g_disc1-app.elf in 18.413626s (34.757 KiB/s)
target halted due to breakpoint, current mode: Thread
xPSR: 0x61000000 pc: 0x2000002e msp: 0x20001000
target halted due to breakpoint, current mode: Thread
xPSR: 0x61000000 pc: 0x2000002e msp: 0x20001000
target halted due to breakpoint, current mode: Thread
xPSR: 0x61000000 pc: 0x2000002e msp: 0x20001000
verified 93172 bytes in 0.920910s (98.803 KiB/s)
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
adapter speed: 1800 kHz
shutdown command invoked
```

## tockloaderでも`uninstall`以外、問題なく動いた。`openocd.cfg`の設定の問題だったか。

```
$ cd libtock-c・examples/
$ tockloader install --board stm32f407g-disc1 --openocd blink/build/blink.tab
[INFO   ] Using settings from KNOWN_BOARDS["stm32f407g-disc1"]
[STATUS ] Installing app on the board...
[INFO   ] Flashing app blink binary to board.
[INFO   ] Finished in 3.546 seconds
```

ここでリセットボタンを押すとアプリが実行。その他のコマンドを試してみる。

### `tockloader list`

```
$ tockloader list --board stm32f407g-disc1 --openocd
[INFO   ] Using settings from KNOWN_BOARDS["stm32f407g-disc1"]
┌──────────────────────────────────────────────────┐
│ App 0                                            |
└──────────────────────────────────────────────────┘
  Name:                  blink
  Enabled:               True
  Sticky:                False
  Total Size in Flash:   2048 bytes


[INFO   ] Finished in 0.251 seconds
```

### `tockloader enable-app`

```
$ tockloader enable-app --board stm32f407g-disc1 --openocd blink
[INFO   ] Using settings from KNOWN_BOARDS["stm32f407g-disc1"]
[STATUS ] Enabling apps...
[INFO   ] Reading app blink binary from board.
[INFO   ] Flashing app blink binary to board.
[INFO   ] Set flag "enable" to "True" for apps: blink
[INFO   ] Finished in 3.838 seconds
```

### `tockloader uninstall` => エラー

```
$ tockloader uninstall --board stm32f407g-disc1 --openocd blink
[INFO   ] Using settings from KNOWN_BOARDS["stm32f407g-disc1"]
[STATUS ] Removing app(s) blink from board...
[STATUS ] Attempting to uninstall:
[STATUS ]   - blink
[ERROR  ] ERROR: openocd returned with error code 1
[INFO   ] Open On-Chip Debugger 0.10.0
Licensed under GNU GPL v2
For bug reports, read
	http://openocd.org/doc/doxygen/bugs.html
Info : auto-selecting first available session transport "hla_swd". To override use 'transport select <transport>'.
Info : The selected transport took over low-level target control. The results might differ compared to plain JTAG/SWD
adapter speed: 2000 kHz
adapter_nsrst_delay: 100
none separate
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : clock speed 1800 kHz
Info : STLINK v2 JTAG v37 API v2 SWIM v26 VID 0x0483 PID 0x374B
Info : using stlink api v2
Info : Target voltage: 2.901302
Info : stm32f4x.cpu: hardware has 6 breakpoints, 4 watchpoints
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
adapter speed: 1800 kHz
target halted due to debug-request, current mode: Thread
xPSR: 0x01000000 pc: 0x08008b7c msp: 0x20001000
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
adapter speed: 4000 kHz
Info : device id = 0x10076413
Info : flash size = 1024kbytes
target halted due to breakpoint, current mode: Thread
xPSR: 0x61000000 pc: 0x20000046 msp: 0x20001000
Error: Verification error address 0x08080000, read back 0x02, expected 0xff


[ERROR  ] openocd error
```

### `tockloader info`

```
$ tockloader info --board stm32f407g-disc1 --openocd
[INFO   ] Using settings from KNOWN_BOARDS["stm32f407g-disc1"]
tockloader version: 1.6.0-dev
[STATUS ] Showing all properties of the board...
Apps:
┌──────────────────────────────────────────────────┐
│ App 0                                            |
└──────────────────────────────────────────────────┘
  Name:                  blink
  Enabled:               True
  Sticky:                False
  Total Size in Flash:   2048 bytes
  Address in Flash:      0x8080000
    version               : 2
    header_size           :         44         0x2c
    total_size            :       2048        0x800
    checksum              :              0x6e4c75d5
    flags                 :          1          0x1
      enabled             : Yes
      sticky              : No
    TLV: Main (1)
      init_fn_offset      :         41         0x29
      protected_size      :          0          0x0
      minimum_ram_size    :       4596       0x11f4
    TLV: Package Name (3)
      package_name        : blink


No bootloader.
[INFO   ] Finished in 0.342 seconds
```

### `tockloader disable-app`

```
$ tockloader disable-app --board stm32f407g-disc1 --openocd blink
[INFO   ] Using settings from KNOWN_BOARDS["stm32f407g-disc1"]
[STATUS ] Disabling apps...
[INFO   ] Reading app blink binary from board.
[INFO   ] Flashing app blink binary to board.
[INFO   ] Set flag "enable" to "False" for apps: blink
[INFO   ] Finished in 3.907 seconds
```

### `tockloader uninstall` => disable-app後もエラー

```
$ tockloader uninstall --board stm32f407g-disc1 --openocd blink
[INFO   ] Using settings from KNOWN_BOARDS["stm32f407g-disc1"]
[STATUS ] Removing app(s) blink from board...
[STATUS ] Attempting to uninstall:
[STATUS ]   - blink
[ERROR  ] ERROR: openocd returned with error code 1
[INFO   ] Open On-Chip Debugger 0.10.0
Licensed under GNU GPL v2
For bug reports, read
	http://openocd.org/doc/doxygen/bugs.html
Info : auto-selecting first available session transport "hla_swd". To override use 'transport select <transport>'.
Info : The selected transport took over low-level target control. The results might differ compared to plain JTAG/SWD
adapter speed: 2000 kHz
adapter_nsrst_delay: 100
none separate
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : clock speed 1800 kHz
Info : STLINK v2 JTAG v37 API v2 SWIM v26 VID 0x0483 PID 0x374B
Info : using stlink api v2
Info : Target voltage: 2.901302
Info : stm32f4x.cpu: hardware has 6 breakpoints, 4 watchpoints
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
adapter speed: 1800 kHz
target halted due to debug-request, current mode: Thread
xPSR: 0x01000000 pc: 0x08008b7c msp: 0x20001000
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
adapter speed: 4000 kHz
Info : device id = 0x10076413
Info : flash size = 1024kbytes
target halted due to breakpoint, current mode: Thread
xPSR: 0x61000000 pc: 0x20000046 msp: 0x20001000
Error: Verification error address 0x08080000, read back 0x02, expected 0xff


[ERROR  ] openocd error
```

### `tockloader update`

```
dspace@mini:~/develop/rust/libtock-c/examples$ tockloader update --board stm32f407g-disc1 --openocd blink/build/blink.tab
[INFO   ] Using settings from KNOWN_BOARDS["stm32f407g-disc1"]
[STATUS ] Updating application on the board...
[INFO   ] Flashing app blink binary to board.
[INFO   ] Finished in 3.782 seconds
```