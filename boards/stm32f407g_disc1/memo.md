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

ロジックを見た感じでは、登録済みのアプリから指定したアプリを除いたリストを作成し、そのリストを元にアプリを再書き込みをしている。
リストがからの場合の処理がなにもないので、

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

## 2つアプリをインストールすると全部消えてしまう。

```
$ tockloader install --board stm32f407g-disc1 --openocd blink/build/blink.tab
[INFO   ] Using settings from KNOWN_BOARDS["stm32f407g-disc1"]
[STATUS ] Installing app on the board...
[INFO   ] Flashing app blink binary to board.
[INFO   ] Finished in 3.589 seconds

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
[INFO   ] Finished in 0.378 seconds

$ tockloader install --board stm32f407g-disc1 --openocd buttons/build/buttons.tab
[INFO   ] Using settings from KNOWN_BOARDS["stm32f407g-disc1"]
[STATUS ] Installing app on the board...
[INFO   ] Flashing app buttons binary to board.
[INFO   ] Finished in 3.652 seconds

$ tockloader info --board stm32f407g-disc1 --openocd
[INFO   ] Using settings from KNOWN_BOARDS["stm32f407g-disc1"]
tockloader version: 1.6.0-dev
[STATUS ] Showing all properties of the board...
Apps:
[INFO   ] No found apps.
No bootloader.
[INFO   ] Finished in 0.254 seconds
```

### 複数アプリをインストールした場合の挙動

1. まず、blinkをinstallすると0x8080000にロードされ、問題なく動く。
2. 次に、buttonsをinstallすると0x8080800にロードされ、0x8080000にあったblinkが消える。buttonsは0x8080800にあるので実行されない。
3. もう一度、buttonsをinstallすると今度は0x8080000にロードされ、問題なく動く。

```
dspace@mini:~/develop/rust/libtock-c/examples$ tockloader install --board stm32f407g-disc1 --openocd blink/build/blink.tab
[INFO   ] Using settings from KNOWN_BOARDS["stm32f407g-disc1"]
[STATUS ] Installing app on the board...
[INFO   ] Flashing app blink binary to board.
[INFO   ] Finished in 3.589 seconds
dspace@mini:~/develop/rust/libtock-c/examples$ tockloader info --board stm32f407g-disc1 --openocd
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
[INFO   ] Finished in 0.378 seconds
dspace@mini:~/develop/rust/libtock-c/examples$ tockloader install --board stm32f407g-disc1 --openocd buttons/build/buttons.tab
[INFO   ] Using settings from KNOWN_BOARDS["stm32f407g-disc1"]
[STATUS ] Installing app on the board...
[INFO   ] Flashing app buttons binary to board.
[INFO   ] Finished in 3.652 seconds
dspace@mini:~/develop/rust/libtock-c/examples$ tockloader info --board stm32f407g-disc1 --openocd
[INFO   ] Using settings from KNOWN_BOARDS["stm32f407g-disc1"]
tockloader version: 1.6.0-dev
[STATUS ] Showing all properties of the board...
Apps:
[INFO   ] No found apps.
No bootloader.
[INFO   ] Finished in 0.254 seconds
dspace@mini:~/develop/rust/libtock-c/examples$ tockloader read --board stm32f407g-disc1 --openocd 0x8080000 0x1000
[INFO   ] Using settings from KNOWN_BOARDS["stm32f407g-disc1"]
[STATUS ] Reading flash from the board...
[STATUS ]   Address: 0x8080000
[STATUS ]   Length:  4096 bytes
08080000  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080010  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080020  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080030  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080040  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080050  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080060  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080070  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080080  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080090  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080800a0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080800b0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080800c0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080800d0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080800e0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080800f0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080100  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080110  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080120  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080130  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080140  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080150  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080160  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080170  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080180  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080190  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080801a0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080801b0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080801c0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080801d0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080801e0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080801f0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080200  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080210  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080220  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080230  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080240  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080250  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080260  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080270  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080280  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080290  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080802a0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080802b0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080802c0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080802d0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080802e0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080802f0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080300  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080310  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080320  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080330  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080340  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080350  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080360  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080370  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080380  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080390  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080803a0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080803b0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080803c0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080803d0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080803e0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080803f0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080400  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080410  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080420  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080430  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080440  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080450  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080460  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080470  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080480  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080490  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080804a0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080804b0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080804c0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080804d0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080804e0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080804f0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080500  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080510  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080520  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080530  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080540  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080550  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080560  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080570  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080580  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080590  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080805a0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080805b0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080805c0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080805d0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080805e0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080805f0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080600  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080610  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080620  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080630  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080640  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080650  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080660  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080670  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080680  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080690  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080806a0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080806b0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080806c0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080806d0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080806e0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080806f0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080700  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080710  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080720  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080730  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080740  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080750  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080760  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080770  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080780  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080790  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080807a0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080807b0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080807c0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080807d0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080807e0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080807f0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080800  02 00 2c 00 00 04 00 00  01 00 00 00 cd 0e 20 74  |..,........... t|
08080810  01 00 0c 00 29 00 00 00  00 00 00 00 e8 11 00 00  |....)...........|
08080820  03 00 07 00 62 75 74 74  6f 6e 73 00 34 02 00 00  |....buttons.4...|
08080830  00 08 00 00 1c 00 00 00  50 02 00 00 1c 08 00 00  |........P.......|
08080840  70 00 00 00 8c 08 00 00  5c 01 00 00 c0 02 00 00  |p.......\.......|
08080850  00 08 00 00 44 6a 04 f1  07 04 0c 44 07 25 24 ea  |....Dj.....D.%$.|
08080860  05 04 85 69 c6 69 35 44  0d 44 06 00 0f 00 9d 42  |...i.i5D.D.....B|
08080870  01 dc a5 46 e9 46 00 20  29 00 04 df 0a 20 21 00  |...F.F. ).... !.|
08080880  04 df 0b 20 29 00 04 df  a5 46 e9 46 30 00 39 00  |... )....F.F0.9.|
08080890  00 f0 2e f8 ff be 01 29  01 d1 00 f0 69 b8 70 47  |.......)....i.pG|
080808a0  38 b5 09 4b 00 21 59 f8  03 00 00 f0 0f f8 00 f0  |8..K.!Y.........|
080808b0  13 f8 00 24 05 46 ac 42  01 db 00 20 38 bd 20 46  |...$.F.B... 8. F|
080808c0  00 f0 10 f8 01 34 f6 e7  00 00 00 00 02 46 0b 46  |.....4.......F.F|
080808d0  03 20 00 21 00 f0 7e b8  00 23 1a 46 19 46 03 20  |. .!..~..#.F.F. |
080808e0  00 f0 7a b8 02 46 00 23  01 21 03 20 00 f0 74 b8  |..z..F.#.!. ..t.|
080808f0  04 46 40 68 22 68 08 b5  0d 46 08 44 22 44 00 23  |.F@h"h...F.D"D.#|
08080900  a1 68 b3 eb 91 0f 19 d3  d4 e9 03 10 62 69 21 44  |.h..........bi!D|
08080910  28 44 00 f0 63 f8 d4 e9  06 02 00 21 28 44 00 f0  |(D..c......!(D..|
08080920  66 f8 21 6a 21 44 00 22  0e 1d 0b 68 b2 eb 93 0f  |f.!j!D."...h....|
08080930  12 d3 ff f7 b5 ff 00 f0  21 f8 fc e7 52 f8 23 10  |........!...R.#.|
08080940  00 29 52 f8 23 10 b6 bf  01 f1 00 41 49 19 09 19  |.)R.#......AI...|
08080950  40 f8 23 10 01 33 d3 e7  56 f8 22 00 2b 58 00 2b  |@.#..3..V.".+X.+|
08080960  b6 bf 03 f1 00 43 5b 19  1b 19 2b 50 02 32 dc e7  |.....C[...+P.2..|
08080970  02 46 00 23 03 21 02 20  00 f0 2e b8 10 b5 12 4b  |.F.#.!. .......K|
08080980  12 4a 59 f8 03 10 59 f8  02 20 0b 68 12 68 93 42  |.JY...Y.. .h.h.B|
08080990  17 d0 0f 4a 59 f8 02 40  14 22 5a 43 01 33 a0 18  |...JY..@."ZC.3..|
080809a0  a4 58 5a 42 02 f0 0f 02  03 f0 0f 03 58 bf 53 42  |.XZB........X.SB|
080809b0  0b 60 a4 46 d0 e9 03 23  bd e8 10 40 d0 e9 01 01  |.`.F...#...@....|
080809c0  60 47 00 df 10 bd 00 bf  04 00 00 00 08 00 00 00  |`G..............|
080809d0  0c 00 00 00 01 df 70 47  02 df 70 47 00 23 10 b5  |......pG..pG.#..|
080809e0  9a 42 00 d1 10 bd cc 5c  c4 54 01 33 f8 e7 03 00  |.B.....\.T.3....|
080809f0  82 18 93 42 00 d1 70 47  19 70 01 33 f9 e7 ff ff  |...B..pG.p.3....|
08080a00  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080a10  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080a20  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080a30  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080a40  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080a50  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080a60  6b 00 00 80 8c 08 00 00  90 08 00 00 94 08 00 00  |k...............|
08080a70  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080a80  f4 01 00 80 14 02 00 80  d4 01 00 80 00 00 00 00  |................|
08080a90  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080aa0  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080ab0  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080ac0  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080ad0  00 00 00 00 00 00 00 00  00 00 00 00 1c 08 00 00  |................|
08080ae0  1c 08 00 00 01 00 00 00  00 00 00 00 28 00 00 00  |............(...|
08080af0  20 08 00 00 02 a5 00 00  24 08 00 00 02 ae 00 00  | .......$.......|
08080b00  28 08 00 00 02 9f 00 00  7c 08 00 00 02 03 00 00  |(.......|.......|
08080b10  80 08 00 00 02 03 00 00  00 00 00 00 00 00 00 00  |................|
08080b20  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080b30  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080b40  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080b50  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080b60  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080b70  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080b80  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080b90  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080ba0  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080bb0  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080bc0  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080bd0  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080be0  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080bf0  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080c00  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080c10  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080c20  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080c30  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080c40  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080c50  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080c60  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080c70  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080c80  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080c90  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080ca0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080cb0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080cc0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080cd0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080ce0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080cf0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080d00  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080d10  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080d20  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080d30  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080d40  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080d50  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080d60  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080d70  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080d80  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080d90  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080da0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080db0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080dc0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080dd0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080de0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080df0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080e00  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080e10  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080e20  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080e30  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080e40  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080e50  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080e60  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080e70  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080e80  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080e90  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080ea0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080eb0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080ec0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080ed0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080ee0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080ef0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080f00  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080f10  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080f20  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080f30  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080f40  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080f50  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080f60  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080f70  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080f80  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080f90  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080fa0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080fb0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080fc0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080fd0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080fe0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080ff0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
[INFO   ] Finished in 0.157 seconds
dspace@mini:~/develop/rust/libtock-c/examples$ tockloader install --board stm32f407g-disc1 --openocd blink/build/blink.tab
[INFO   ] Using settings from KNOWN_BOARDS["stm32f407g-disc1"]
[STATUS ] Installing app on the board...
[INFO   ] Flashing app blink binary to board.
[INFO   ] Finished in 3.591 seconds
dspace@mini:~/develop/rust/libtock-c/examples$ tockloader info --board stm32f407g-disc1 --openocd
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
[INFO   ] Finished in 0.373 seconds
dspace@mini:~/develop/rust/libtock-c/examples$ tockloader read --board stm32f407g-disc1 --openocd 0x8080000 0x1000
[INFO   ] Using settings from KNOWN_BOARDS["stm32f407g-disc1"]
[STATUS ] Reading flash from the board...
[STATUS ]   Address: 0x8080000
[STATUS ]   Length:  4096 bytes
08080000  02 00 2c 00 00 08 00 00  01 00 00 00 d5 75 4c 6e  |..,..........uLn|
08080010  01 00 0c 00 29 00 00 00  00 00 00 00 f4 11 00 00  |....)...........|
08080020  03 00 05 00 62 6c 69 6e  6b 00 00 00 04 04 00 00  |....blink.......|
08080030  00 08 00 00 24 00 00 00  28 04 00 00 24 08 00 00  |....$...(...$...|
08080040  70 00 00 00 94 08 00 00  60 01 00 00 98 04 00 00  |p.......`.......|
08080050  00 08 00 00 44 6a 04 f1  07 04 0c 44 07 25 24 ea  |....Dj.....D.%$.|
08080060  05 04 85 69 c6 69 35 44  0d 44 06 00 0f 00 9d 42  |...i.i5D.D.....B|
08080070  01 dc a5 46 e9 46 00 20  29 00 04 df 0a 20 21 00  |...F.F. ).... !.|
08080080  04 df 0b 20 29 00 04 df  a5 46 e9 46 30 00 39 00  |... )....F.F0.9.|
08080090  00 f0 c0 f8 ff be 70 b5  00 f0 fc f8 00 25 06 46  |......p......%.F|
080800a0  00 24 b4 42 04 db fa 20  00 f0 9e f8 01 35 f7 e7  |.$.B... .....5..|
080800b0  45 fa 04 f3 db 07 20 46  03 d5 00 f0 f1 f8 01 34  |E..... F.......4|
080800c0  ef e7 00 f0 f3 f8 fa e7  01 22 1a 70 70 47 00 23  |.........".ppG.#|
080800d0  1a 46 02 21 18 46 00 f0  55 b9 ff ff f8 b5 1c 46  |.F.!.F..U......F|
080800e0  0f 46 16 46 05 46 ff f7  f2 ff 00 22 c4 e9 00 05  |.F.F.F....."....|
080800f0  c4 e9 02 76 c4 e9 04 22  14 4b 59 f8 03 10 0b 68  |...v...".KY....h|
08080100  5b b9 0c 60 12 4b 00 21  59 f8 03 00 00 f0 3c f9  |[..`.K.!Y.....<.|
08080110  60 68 bd e8 f8 40 00 f0  3d b9 0e 46 58 68 28 1a  |`h...@..=..FXh(.|
08080120  00 28 09 da 34 60 c4 e9  04 32 5c 61 0b 68 9c 42  |.(..4`...2\a.h.B|
08080130  e8 d0 f8 bd 03 46 f1 e7  18 69 03 f1 10 06 1a 46  |.....F...i.....F|
08080140  00 28 f7 d1 1c 61 c4 e9  04 03 ef e7 08 00 00 00  |.(...a..........|
08080150  04 00 00 00 73 b5 13 4b  59 f8 03 50 2c 68 00 26  |....s..KY..P,h.&|
08080160  0c b9 02 b0 70 bd ff f7  b2 ff d4 e9 00 32 01 46  |....p........2.F|
08080170  d0 1a cb 1a 98 42 05 d9  10 46 02 b0 bd e8 70 40  |.....B...F....p@|
08080180  00 f0 08 b9 2b 68 23 b1  18 69 28 60 00 b1 46 61  |....+h#..i(`..Fa|
08080190  1e 61 a0 68 20 b1 e3 68  00 93 00 23 00 f0 8c f8  |.a.h ..h...#....|
080801a0  2c 68 dd e7 08 00 00 00  2d e9 f0 4d 04 46 0e 46  |,h......-..M.F.F|
080801b0  17 46 98 46 00 f0 f4 f8  4f f4 7a 73 b4 fb f3 f5  |.F.F....O.zs....|
080801c0  00 fb 05 fa 03 fb 15 45  b0 fb f3 f0 45 43 ff f7  |.......E....EC..|
080801d0  7e ff 50 44 08 f1 0c 03  3a 46 31 46 28 44 bd e8  |~.PD....:F1F(D..|
080801e0  f0 4d ff f7 7b bf ff ff  10 b5 8a b0 00 23 8d f8  |.M..{........#..|
080801f0  03 30 07 4b 0d f1 03 04  59 f8 03 10 22 46 01 ab  |.0.K....Y..."F..|
08080200  ff f7 d2 ff 20 46 00 f0  b3 f8 0a b0 10 bd 00 bf  |.... F..........|
08080210  00 00 00 00 04 46 40 68  22 68 08 b5 0d 46 08 44  |.....F@h"h...F.D|
08080220  22 44 00 23 a1 68 b3 eb  91 0f 19 d3 d4 e9 03 10  |"D.#.h..........|
08080230  62 69 21 44 28 44 00 f0  b9 f8 d4 e9 06 02 00 21  |bi!D(D.........!|
08080240  28 44 00 f0 bc f8 21 6a  21 44 00 22 0e 1d 0b 68  |(D....!j!D."...h|
08080250  b2 eb 93 0f 12 d3 ff f7  1e ff 00 f0 5d f8 fc e7  |............]...|
08080260  52 f8 23 10 00 29 52 f8  23 10 b6 bf 01 f1 00 41  |R.#..)R.#......A|
08080270  49 19 09 19 40 f8 23 10  01 33 d3 e7 56 f8 22 00  |I...@.#..3..V.".|
08080280  2b 58 00 2b b6 bf 03 f1  00 43 5b 19 1b 19 2b 50  |+X.+.....C[...+P|
08080290  02 32 dc e7 00 23 1a 46  19 46 02 20 00 f0 72 b8  |.2...#.F.F. ..r.|
080802a0  02 46 00 23 01 21 02 20  00 f0 6c b8 02 21 02 46  |.F.#.!. ..l..!.F|
080802b0  00 23 08 46 00 f0 66 b8  f0 b5 07 46 13 48 59 f8  |.#.F..f....F.HY.|
080802c0  00 c0 dc f8 00 e0 0e f1  01 06 74 42 04 f0 0f 04  |..........tB....|
080802d0  06 f0 0f 00 58 bf 60 42  0d 4c 59 f8 04 40 24 68  |....X.`B.LY..@$h|
080802e0  84 42 0f d0 0b 4c 14 25  59 f8 04 60 cc f8 00 00  |.B...L.%Y..`....|
080802f0  05 fb 0e f5 74 19 c4 e9  01 12 e3 60 77 51 05 9b  |....t......`wQ..|
08080300  23 61 f0 bd 4f f0 ff 30  fb e7 00 bf 10 00 00 00  |#a..O..0........|
08080310  0c 00 00 00 14 00 00 00  10 b5 12 4b 12 4a 59 f8  |...........K.JY.|
08080320  03 10 59 f8 02 20 0b 68  12 68 93 42 17 d0 0f 4a  |..Y.. .h.h.B...J|
08080330  59 f8 02 40 14 22 5a 43  01 33 a0 18 a4 58 5a 42  |Y..@."ZC.3...XZB|
08080340  02 f0 0f 02 03 f0 0f 03  58 bf 53 42 0b 60 a4 46  |........X.SB.`.F|
08080350  d0 e9 03 23 bd e8 10 40  d0 e9 01 01 60 47 00 df  |...#...@....`G..|
08080360  10 bd 00 bf 0c 00 00 00  10 00 00 00 14 00 00 00  |................|
08080370  10 b5 04 46 23 78 03 b1  10 bd ff f7 cd ff f9 e7  |...F#x..........|
08080380  01 df 70 47 02 df 70 47  0b 46 00 21 02 46 08 46  |..pG..pG.F.!.F.F|
08080390  ff f7 f6 bf 00 23 02 46  04 21 18 46 ff f7 f2 bf  |.....#.F.!.F....|
080803a0  00 23 1a 46 01 21 18 46  ff f7 ec bf 00 23 10 b5  |.#.F.!.F.....#..|
080803b0  9a 42 00 d1 10 bd cc 5c  c4 54 01 33 f8 e7 03 00  |.B.....\.T.3....|
080803c0  82 18 93 42 00 d1 70 47  19 70 01 33 f9 e7 ff ff  |...B..pG.p.3....|
080803d0  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
080803e0  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
080803f0  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080400  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080410  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080420  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080430  9d 00 00 80 29 01 00 80  94 08 00 00 98 08 00 00  |....)...........|
08080440  9c 08 00 00 a0 08 00 00  00 00 00 00 00 00 00 00  |................|
08080450  00 00 00 00 00 00 00 00  c4 03 00 80 e4 03 00 80  |................|
08080460  a4 03 00 80 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080470  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080480  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080490  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
080804a0  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
080804b0  00 00 00 00 24 08 00 00  24 08 00 00 01 00 00 00  |....$...$.......|
080804c0  00 00 00 00 28 00 00 00  28 08 00 00 02 be 00 00  |....(...(.......|
080804d0  2c 08 00 00 02 c8 00 00  30 08 00 00 02 b7 00 00  |,.......0.......|
080804e0  84 08 00 00 02 03 00 00  88 08 00 00 02 03 00 00  |................|
080804f0  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080500  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080510  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080520  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080530  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080540  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080550  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080560  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080570  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080580  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080590  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
080805a0  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
080805b0  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
080805c0  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
080805d0  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
080805e0  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
080805f0  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080600  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080610  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080620  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080630  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080640  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080650  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080660  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080670  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080680  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080690  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
080806a0  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
080806b0  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
080806c0  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
080806d0  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
080806e0  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
080806f0  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080700  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080710  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080720  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080730  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080740  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080750  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080760  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080770  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080780  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080790  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
080807a0  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
080807b0  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
080807c0  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
080807d0  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
080807e0  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
080807f0  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080800  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080810  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080820  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080830  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080840  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080850  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080860  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080870  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080880  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080890  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080808a0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080808b0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080808c0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080808d0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080808e0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080808f0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080900  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080910  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080920  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080930  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080940  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080950  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080960  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080970  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080980  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080990  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080809a0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080809b0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080809c0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080809d0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080809e0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080809f0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080a00  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080a10  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080a20  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080a30  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080a40  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080a50  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080a60  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080a70  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080a80  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080a90  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080aa0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080ab0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080ac0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080ad0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080ae0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080af0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080b00  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080b10  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080b20  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080b30  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080b40  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080b50  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080b60  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080b70  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080b80  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080b90  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080ba0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080bb0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080bc0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080bd0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080be0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080bf0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080c00  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080c10  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080c20  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080c30  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080c40  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080c50  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080c60  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080c70  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080c80  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080c90  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080ca0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080cb0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080cc0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080cd0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080ce0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080cf0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080d00  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080d10  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080d20  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080d30  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080d40  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080d50  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080d60  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080d70  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080d80  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080d90  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080da0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080db0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080dc0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080dd0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080de0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080df0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080e00  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080e10  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080e20  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080e30  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080e40  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080e50  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080e60  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080e70  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080e80  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080e90  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080ea0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080eb0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080ec0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080ed0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080ee0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080ef0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080f00  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080f10  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080f20  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080f30  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080f40  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080f50  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080f60  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080f70  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080f80  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080f90  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080fa0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080fb0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080fc0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080fd0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080fe0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080ff0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
[INFO   ] Finished in 0.156 seconds
```

## dmaを有効にして`example/c_hello`を実行

```
$ tockloader install --board stm32f407g-disc1 --openocd c_hello/build/c_hello.tab
[INFO   ] Using settings from KNOWN_BOARDS["stm32f407g-disc1"]
[STATUS ] Installing app on the board...
[INFO   ] Flashing app c_hello binary to board.
[INFO   ] Finished in 3.553 seconds

$ tockloader listen
[INFO   ] No device name specified. Using default name "tock".
[INFO   ] No serial port with device name "tock" found.
[INFO   ] Found 3 serial ports.
Multiple serial port options found. Which would you like to use?
[0]	/dev/cu.raspiw1-SerialPort-34 - n/a
[1]	/dev/cu.usbmodem14603 - STM32 STLink
[2]	/dev/cu.usbserial-AI057C9L - FT232R USB UART

Which option? [0] 2
[INFO   ] Using "/dev/cu.usbserial-AI057C9L - FT232R USB UART".
[INFO   ] Listening for serial output.

# ここでリセットボタンを押すと

Exception in thread rx:
Traceback (most recent call last):
  File "/Users/dspace/Library/Python/3.8/lib/python/site-packages/serial/serialposix.py", line 500, in read
    raise SerialException(
serial.serialutil.SerialException: device reports readiness to read but returned no data (device disconnected or multiple access on port?)

During handling of the above exception, another exception occurred:

Traceback (most recent call last):
  File "/usr/local/Cellar/python@3.8/3.8.4/Frameworks/Python.framework/Versions/3.8/lib/python3.8/threading.py", line 932, in _bootstrap_inner
    self.run()
  File "/usr/local/Cellar/python@3.8/3.8.4/Frameworks/Python.framework/Versions/3.8/lib/python3.8/threading.py", line 870, in run
    self._target(*self._args, **self._kwargs)
  File "/Users/dspace/Library/Python/3.8/lib/python/site-packages/serial/tools/miniterm.py", line 445, in reader
    data = self.serial.read(self.serial.in_waiting or 1)
  File "/Users/dspace/Library/Python/3.8/lib/python/site-packages/serial/serialposix.py", line 509, in read
    raise SerialException('read failed: {}'.format(e))
serial.serialutil.SerialException: read failed: device reports readiness to read but returned no data (device disconnected or multiple access on port?)
Exception in thread Thread-1:
Traceback (most recent call last):
  File "/usr/local/Cellar/python@3.8/3.8.4/Frameworks/Python.framework/Versions/3.8/lib/python3.8/threading.py", line 932, in _bootstrap_inner
```

### minicomの画面

リセットボタンを押した段階で`hello wolrd!`を出力。

`tockloader listen`は不要だった。終了後も、リセットボタンを押すとアプリが再実行され`hello wolrd!`を出力してアプリは終了（無限ループに入る）。これが繰り返される。

```
Welcome to minicom 2.7.1

OPTIONS:
Compiled on Aug 20 2018, 10:22:42.
Port /dev/cu.usbserial-AI057C9L, 09:08:26

Press CTRL-A Z for help on special keys

Initialization complete. Entering main loop

Hello World!
```

### デバッグモードで実行

```
dspace@mini:~/develop/rust/libtock-c/examples$ tockloader install --board stm32f407g-disc1 --openocd blink/build/blink.tab --debug
[INFO   ] Using settings from KNOWN_BOARDS["stm32f407g-disc1"]
[STATUS ] Installing app on the board...
[DEBUG  ] Using read command: "dump_image {{binary}} {address:#x} {length};"
[DEBUG  ] Expanded read command: "dump_image {binary} 0x8080000 200;"
[DEBUG  ] Running "openocd -c "interface hla;                                                    hla_layout stlink;                                                    hla_device_desc "ST-LINK/V2-1";                                                    hla_vid_pid 0x0483 0x374b;                                                    set WORKAREASIZE 0x40000;                                                    source [find target/stm32f4x.cfg];  init; reset init; halt; dump_image /var/folders/rc/r22l0fy5611279w4ljf1vbn40000gn/T/tmpyga8nrha.bin 0x8080000 200;  exit"".  ## 200 byte: c_hello
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
Info : Target voltage: 2.899586
Info : stm32f4x.cpu: hardware has 6 breakpoints, 4 watchpoints
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
adapter speed: 1800 kHz
target halted due to debug-request, current mode: Thread
xPSR: 0x01000000 pc: 0x08007e5c msp: 0x20001000
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
adapter speed: 4000 kHz
dumped 200 bytes in 0.001512s (129.175 KiB/s)

[DEBUG  ] Using read command: "dump_image {{binary}} {address:#x} {length};"
[DEBUG  ] Expanded read command: "dump_image {binary} 0x8080400 200;"
[DEBUG  ] Running "openocd -c "interface hla;                                                    hla_layout stlink;                                                    hla_device_desc "ST-LINK/V2-1";                                                    hla_vid_pid 0x0483 0x374b;                                                    set WORKAREASIZE 0x40000;                                                    source [find target/stm32f4x.cfg];  init; reset init; halt; dump_image /var/folders/rc/r22l0fy5611279w4ljf1vbn40000gn/T/tmp3ml8qk91.bin 0x8080400 200;  exit"".   ## 200 byte; 0xff
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
Info : Target voltage: 2.899586
Info : stm32f4x.cpu: hardware has 6 breakpoints, 4 watchpoints
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
adapter speed: 1800 kHz
target halted due to debug-request, current mode: Thread
xPSR: 0x01000000 pc: 0x08007e5c msp: 0x20001000
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
adapter speed: 4000 kHz
dumped 200 bytes in 0.001588s (122.993 KiB/s)

[DEBUG  ] Found 1 app on the board.
[DEBUG  ]   1. c_hello
[INFO   ] Reading app c_hello binary from board.
[DEBUG  ] Using read command: "dump_image {{binary}} {address:#x} {length};"
[DEBUG  ] Expanded read command: "dump_image {binary} 0x8080000 1024;"
[DEBUG  ] Running "openocd -c "interface hla;                                                    hla_layout stlink;                                                    hla_device_desc "ST-LINK/V2-1";                                                    hla_vid_pid 0x0483 0x374b;                                                    set WORKAREASIZE 0x40000;                                                    source [find target/stm32f4x.cfg];  init; reset init; halt; dump_image /var/folders/rc/r22l0fy5611279w4ljf1vbn40000gn/T/tmpuyu86a0d.bin 0x8080000 1024;  exit"".  # 1024 byte: c_hello
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
Info : Target voltage: 2.899586
Info : stm32f4x.cpu: hardware has 6 breakpoints, 4 watchpoints
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
adapter speed: 1800 kHz
target halted due to debug-request, current mode: Thread
xPSR: 0x01000000 pc: 0x08007e5c msp: 0x20001000
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
adapter speed: 4000 kHz
dumped 1024 bytes in 0.006485s (154.202 KiB/s)

[INFO   ] Flashing app blink binary to board.
[DEBUG  ] Using program command: "program {{binary}} verify {address:#x};"
[DEBUG  ] Expanded program command: "program {binary} verify 0x8080000;"
[DEBUG  ] Running "openocd -c "interface hla;                                                    hla_layout stlink;                                                    hla_device_desc "ST-LINK/V2-1";                                                    hla_vid_pid 0x0483 0x374b;                                                    set WORKAREASIZE 0x40000;                                                    source [find target/stm32f4x.cfg];  init; reset init; halt; program /var/folders/rc/r22l0fy5611279w4ljf1vbn40000gn/T/tmplmrh5gyf.bin verify 0x8080000;  exit"".  ## 2048 byte: blink
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
Info : Target voltage: 2.899586
Info : stm32f4x.cpu: hardware has 6 breakpoints, 4 watchpoints
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
adapter speed: 1800 kHz
target halted due to debug-request, current mode: Thread
xPSR: 0x01000000 pc: 0x08007e5c msp: 0x20001000
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
adapter speed: 4000 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
adapter speed: 1800 kHz
target halted due to debug-request, current mode: Thread
xPSR: 0x01000000 pc: 0x08007e5c msp: 0x20001000
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
adapter speed: 4000 kHz
** Programming Started **
auto erase enabled
Info : device id = 0x10076413
Info : flash size = 1024kbytes
target halted due to breakpoint, current mode: Thread
xPSR: 0x61000000 pc: 0x20000046 msp: 0x20001000
wrote 131072 bytes from file /var/folders/rc/r22l0fy5611279w4ljf1vbn40000gn/T/tmplmrh5gyf.bin in 3.079415s (41.566 KiB/s)    ## 2048 byte: blink
** Programming Finished **
** Verify Started **
target halted due to breakpoint, current mode: Thread
xPSR: 0x61000000 pc: 0x2000002e msp: 0x20001000
verified 2048 bytes in 0.034753s (57.549 KiB/s)
** Verified OK **

[INFO   ] Flashing app c_hello binary to board.
[DEBUG  ] Using program command: "program {{binary}} verify {address:#x};"
[DEBUG  ] Expanded program command: "program {binary} verify 0x8080800;"
[DEBUG  ] Running "openocd -c "interface hla;                                                    hla_layout stlink;                                                    hla_device_desc "ST-LINK/V2-1";                                                    hla_vid_pid 0x0483 0x374b;                                                    set WORKAREASIZE 0x40000;                                                    source [find target/stm32f4x.cfg];  init; reset init; halt; program /var/folders/rc/r22l0fy5611279w4ljf1vbn40000gn/T/tmpt0c5iti7.bin verify 0x8080800;  exit"".  ## 1024 byte: c_hello
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
Info : Target voltage: 2.899586
Info : stm32f4x.cpu: hardware has 6 breakpoints, 4 watchpoints
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
adapter speed: 1800 kHz
target halted due to debug-request, current mode: Thread
xPSR: 0x01000000 pc: 0x08007e5c msp: 0x20001000
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
adapter speed: 4000 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
adapter speed: 1800 kHz
target halted due to debug-request, current mode: Thread
xPSR: 0x01000000 pc: 0x08007e5c msp: 0x20001000
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
adapter speed: 4000 kHz
** Programming Started **
auto erase enabled
Info : device id = 0x10076413
Info : flash size = 1024kbytes
Warn : Adding extra erase range, 0x00080000 to 0x000807ff
target halted due to breakpoint, current mode: Thread
xPSR: 0x61000000 pc: 0x20000046 msp: 0x20001000
wrote 129024 bytes from file /var/folders/rc/r22l0fy5611279w4ljf1vbn40000gn/T/tmpt0c5iti7.bin in 3.060769s (41.166 KiB/s)  ## 1024 byte: c_hello
** Programming Finished **
** Verify Started **
target halted due to breakpoint, current mode: Thread
xPSR: 0x61000000 pc: 0x2000002e msp: 0x20001000
verified 1024 bytes in 0.032619s (30.657 KiB/s)
** Verified OK **

[DEBUG  ] Erasing page at address 0x8080c00
[DEBUG  ] Using erase command: "flash fillb 0x8080c00 0xff 512;"
[DEBUG  ] Expanded erase command: "flash fillb 0x8080c00 0xff 512;"
[DEBUG  ] Running "openocd -c "interface hla;                                                    hla_layout stlink;                                                    hla_device_desc "ST-LINK/V2-1";                                                    hla_vid_pid 0x0483 0x374b;                                                    set WORKAREASIZE 0x40000;                                                    source [find target/stm32f4x.cfg];  init; reset init; halt; flash fillb 0x8080c00 0xff 512;  exit"".
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
Info : Target voltage: 2.899586
Info : stm32f4x.cpu: hardware has 6 breakpoints, 4 watchpoints
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
adapter speed: 1800 kHz
target halted due to debug-request, current mode: Thread
xPSR: 0x01000000 pc: 0x08007e5c msp: 0x20001000
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
adapter speed: 4000 kHz
Info : device id = 0x10076413
Info : flash size = 1024kbytes
target halted due to breakpoint, current mode: Thread
xPSR: 0x61000000 pc: 0x20000046 msp: 0x20001000
wrote 512 bytes to 0x08080c00 in 0.041670s (11.999 KiB/s)

[INFO   ] Finished in 7.120 seconds

dspace@mini:~/develop/rust/libtock-c/examples$ tockloader read --board stm32f407g-disc1 --openocd 0x8080000 0x100
[INFO   ] Using settings from KNOWN_BOARDS["stm32f407g-disc1"]
[STATUS ] Reading flash from the board...
[STATUS ]   Address: 0x8080000
[STATUS ]   Length:  256 bytes
08080000  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080010  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080020  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080030  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080040  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080050  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080060  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080070  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080080  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080090  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080800a0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080800b0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080800c0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080800d0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080800e0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080800f0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
[INFO   ] Finished in 0.094 seconds
dspace@mini:~/develop/rust/libtock-c/examples$ tockloader read --board stm32f407g-disc1 --openocd 0x8080800 0x100
[INFO   ] Using settings from KNOWN_BOARDS["stm32f407g-disc1"]
[STATUS ] Reading flash from the board...
[STATUS ]   Address: 0x8080800
[STATUS ]   Length:  256 bytes
08080800  02 00 2c 00 00 04 00 00  01 00 00 00 db 26 20 65  |..,..........& e|
08080810  01 00 0c 00 29 00 00 00  00 00 00 00 fc 11 00 00  |....)...........|
08080820  03 00 07 00 63 5f 68 65  6c 6c 6f 00 34 02 00 00  |....c_hello.4...|
08080830  00 08 00 00 20 00 00 00  54 02 00 00 20 08 00 00  |.... ...T... ...|
08080840  80 00 00 00 a0 08 00 00  5c 01 00 00 d4 02 00 00  |........\.......|
08080850  00 08 00 00 44 6a 04 f1  07 04 0c 44 07 25 24 ea  |....Dj.....D.%$.|
08080860  05 04 85 69 c6 69 35 44  0d 44 06 00 0f 00 9d 42  |...i.i5D.D.....B|
08080870  01 dc a5 46 e9 46 00 20  29 00 04 df 0a 20 21 00  |...F.F. ).... !.|
08080880  04 df 0b 20 29 00 04 df  a5 46 e9 46 30 00 39 00  |... )....F.F0.9.|
08080890  00 f0 31 f8 ff be 70 47  08 b5 06 4b 59 f8 03 20  |..1...pG...KY.. |
080808a0  05 4b 0f 21 59 f8 03 00  00 23 00 f0 07 f8 00 20  |.K.!Y....#..... |
080808b0  08 bd 00 bf 00 00 00 00  10 00 00 00 70 b5 0c 46  |............p..F|
080808c0  1e 46 0b 46 01 21 15 46  02 46 08 46 00 f0 84 f8  |.F.F.!.F.F.F....|
080808d0  00 28 0f db 01 21 33 46  2a 46 08 46 00 f0 78 f8  |.(...!3F*F.F..x.|
080808e0  00 28 07 db 01 21 22 46  00 23 bd e8 70 40 08 46  |.(...!"F.#..p@.F|
080808f0  00 f0 70 b8 70 bd 04 46  40 68 22 68 08 b5 0d 46  |..p.p..F@h"h...F|
[INFO   ] Finished in 0.130 seconds
dspace@mini:~/develop/rust/libtock-c/examples$ tockloader install --board stm32f407g-disc1 --openocd blink/build/blink.tab --debug
[INFO   ] Using settings from KNOWN_BOARDS["stm32f407g-disc1"]
[STATUS ] Installing app on the board...
[DEBUG  ] Using read command: "dump_image {{binary}} {address:#x} {length};"
[DEBUG  ] Expanded read command: "dump_image {binary} 0x8080000 200;"
[DEBUG  ] Running "openocd -c "interface hla;                                                    hla_layout stlink;                                                    hla_device_desc "ST-LINK/V2-1";                                                    hla_vid_pid 0x0483 0x374b;                                                    set WORKAREASIZE 0x40000;                                                    source [find target/stm32f4x.cfg];  init; reset init; halt; dump_image /var/folders/rc/r22l0fy5611279w4ljf1vbn40000gn/T/tmpc_vu6_u3.bin 0x8080000 200;  exit"".  ## 200 byte: 0xff
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
Info : Target voltage: 2.899586
Info : stm32f4x.cpu: hardware has 6 breakpoints, 4 watchpoints
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
adapter speed: 1800 kHz
target halted due to debug-request, current mode: Thread
xPSR: 0x01000000 pc: 0x08007e5c msp: 0x20001000
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
adapter speed: 4000 kHz
dumped 200 bytes in 0.001541s (126.744 KiB/s)

[DEBUG  ] Found 0 apps on the board.
[INFO   ] Flashing app blink binary to board.
[DEBUG  ] Using program command: "program {{binary}} verify {address:#x};"
[DEBUG  ] Expanded program command: "program {binary} verify 0x8080000;"
[DEBUG  ] Running "openocd -c "interface hla;                                                    hla_layout stlink;                                                    hla_device_desc "ST-LINK/V2-1";                                                    hla_vid_pid 0x0483 0x374b;                                                    set WORKAREASIZE 0x40000;                                                    source [find target/stm32f4x.cfg];  init; reset init; halt; program /var/folders/rc/r22l0fy5611279w4ljf1vbn40000gn/T/tmp1f8l_app.bin verify 0x8080000;  exit"". ## 2048 byte: blink
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
Info : Target voltage: 2.899586
Info : stm32f4x.cpu: hardware has 6 breakpoints, 4 watchpoints
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
adapter speed: 1800 kHz
target halted due to debug-request, current mode: Thread
xPSR: 0x01000000 pc: 0x08007e5c msp: 0x20001000
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
adapter speed: 4000 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
adapter speed: 1800 kHz
target halted due to debug-request, current mode: Thread
xPSR: 0x01000000 pc: 0x08007e5c msp: 0x20001000
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
adapter speed: 4000 kHz
** Programming Started **
auto erase enabled
Info : device id = 0x10076413
Info : flash size = 1024kbytes
target halted due to breakpoint, current mode: Thread
xPSR: 0x61000000 pc: 0x20000046 msp: 0x20001000
wrote 131072 bytes from file /var/folders/rc/r22l0fy5611279w4ljf1vbn40000gn/T/tmp1f8l_app.bin in 3.064064s (41.775 KiB/s)  ## 2048 byte: blink
** Programming Finished **
** Verify Started **
target halted due to breakpoint, current mode: Thread
xPSR: 0x61000000 pc: 0x2000002e msp: 0x20001000
verified 2048 bytes in 0.035201s (56.817 KiB/s)
** Verified OK **

[DEBUG  ] Erasing page at address 0x8080800
[DEBUG  ] Using erase command: "flash fillb 0x8080800 0xff 512;"
[DEBUG  ] Expanded erase command: "flash fillb 0x8080800 0xff 512;"
[DEBUG  ] Running "openocd -c "interface hla;                                                    hla_layout stlink;                                                    hla_device_desc "ST-LINK/V2-1";                                                    hla_vid_pid 0x0483 0x374b;                                                    set WORKAREASIZE 0x40000;                                                    source [find target/stm32f4x.cfg];  init; reset init; halt; flash fillb 0x8080800 0xff 512;  exit"".
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
Info : Target voltage: 2.898167
Info : stm32f4x.cpu: hardware has 6 breakpoints, 4 watchpoints
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
adapter speed: 1800 kHz
target halted due to debug-request, current mode: Thread
xPSR: 0x01000000 pc: 0x08007e5c msp: 0x20001000
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
adapter speed: 4000 kHz
Info : device id = 0x10076413
Info : flash size = 1024kbytes
target halted due to breakpoint, current mode: Thread
xPSR: 0x61000000 pc: 0x20000046 msp: 0x20001000
wrote 512 bytes to 0x08080800 in 0.041343s (12.094 KiB/s)

[INFO   ] Finished in 3.573 seconds
dspace@mini:~/develop/rust/libtock-c/examples$ tockloader read --board stm32f407g-disc1 --openocd 0x8080000 0x100
[INFO   ] Using settings from KNOWN_BOARDS["stm32f407g-disc1"]
[STATUS ] Reading flash from the board...
[STATUS ]   Address: 0x8080000
[STATUS ]   Length:  256 bytes
08080000  02 00 2c 00 00 08 00 00  01 00 00 00 d5 75 4c 6e  |..,..........uLn|
08080010  01 00 0c 00 29 00 00 00  00 00 00 00 f4 11 00 00  |....)...........|
08080020  03 00 05 00 62 6c 69 6e  6b 00 00 00 04 04 00 00  |....blink.......|
08080030  00 08 00 00 24 00 00 00  28 04 00 00 24 08 00 00  |....$...(...$...|
08080040  70 00 00 00 94 08 00 00  60 01 00 00 98 04 00 00  |p.......`.......|
08080050  00 08 00 00 44 6a 04 f1  07 04 0c 44 07 25 24 ea  |....Dj.....D.%$.|
08080060  05 04 85 69 c6 69 35 44  0d 44 06 00 0f 00 9d 42  |...i.i5D.D.....B|
08080070  01 dc a5 46 e9 46 00 20  29 00 04 df 0a 20 21 00  |...F.F. ).... !.|
08080080  04 df 0b 20 29 00 04 df  a5 46 e9 46 30 00 39 00  |... )....F.F0.9.|
08080090  00 f0 c0 f8 ff be 70 b5  00 f0 fc f8 00 25 06 46  |......p......%.F|
080800a0  00 24 b4 42 04 db fa 20  00 f0 9e f8 01 35 f7 e7  |.$.B... .....5..|
080800b0  45 fa 04 f3 db 07 20 46  03 d5 00 f0 f1 f8 01 34  |E..... F.......4|
080800c0  ef e7 00 f0 f3 f8 fa e7  01 22 1a 70 70 47 00 23  |.........".ppG.#|
080800d0  1a 46 02 21 18 46 00 f0  55 b9 ff ff f8 b5 1c 46  |.F.!.F..U......F|
080800e0  0f 46 16 46 05 46 ff f7  f2 ff 00 22 c4 e9 00 05  |.F.F.F....."....|
080800f0  c4 e9 02 76 c4 e9 04 22  14 4b 59 f8 03 10 0b 68  |...v...".KY....h|
[INFO   ] Finished in 0.092 seconds
dspace@mini:~/develop/rust/libtock-c/examples$ tockloader read --board stm32f407g-disc1 --openocd 0x8080800 0x100
[INFO   ] Using settings from KNOWN_BOARDS["stm32f407g-disc1"]
[STATUS ] Reading flash from the board...
[STATUS ]   Address: 0x8080800
[STATUS ]   Length:  256 bytes
08080800  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080810  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080820  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080830  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080840  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080850  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080860  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080870  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080880  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080890  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080808a0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080808b0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080808c0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080808d0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080808e0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
080808f0  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
[INFO   ] Finished in 0.127 seconds
```

## 抜き出し

```
tockloader install --board stm32f407g-disc1 --openocd blink/build/blink.tab --debug

1. 0x8080000 から 200バイト読み出し => c_helloあり

Expanded read command: "dump_image {binary} 0x8080000 200;"
[DEBUG  ] Running "openocd -c "interface hla; hla_layout stlink; hla_device_desc "ST-LINK/V2-1";
            hla_vid_pid 0x0483 0x374b; set WORKAREASIZE 0x40000; source [find target/stm32f4x.cfg];
init; reset init; halt; dump_image /var/folders/rc/r22l0fy5611279w4ljf1vbn40000gn/T/tmpyga8nrha.bin 0x8080000 200;  exit""
## 200 byte: c_hello

2. 0x8080400 から 200バイト読み出し => データなし

Expanded read command: "dump_image {binary} 0x8080400 200;"
init; reset init; halt; dump_image /var/folders/rc/r22l0fy5611279w4ljf1vbn40000gn/T/tmp3ml8qk91.bin 0x8080400 200;  exit""
## 200 byte; 0xff

3. app1: c_hello を発見。改めて、0x8080000 から 1024 (0x400) バイト読み出し => c_hello

[DEBUG  ] Found 1 app on the board.
[DEBUG  ]   1. c_hello
[INFO   ] Reading app c_hello binary from board.
[DEBUG  ] Expanded read command: "dump_image {binary} 0x8080000 1024;"
init; reset init; halt; dump_image /var/folders/rc/r22l0fy5611279w4ljf1vbn40000gn/T/tmpuyu86a0d.bin 0x8080000 1024;  exit""
# 1024 byte: c_hello

4. blink を書き出し。0x8080000 から 2048 (0x800) バイト書き出し
[INFO   ] Flashing app blink binary to board.
[DEBUG  ] Expanded program command: "program {binary} verify 0x8080000;"
¥init; reset init; halt; program /var/folders/rc/r22l0fy5611279w4ljf1vbn40000gn/T/tmplmrh5gyf.bin verify 0x8080000;  exit"".
## 2048 byte: blink

5. 書き出しOK

wrote 131072 bytes from file /var/folders/rc/r22l0fy5611279w4ljf1vbn40000gn/T/tmplmrh5gyf.bin in 3.079415s (41.566 KiB/s)
## 2048 byte: blink

6. c_hello を書き出し。 0x8080800 から 1024 (0x400) バイト書き出し

[INFO   ] Flashing app c_hello binary to board.
[DEBUG  ] Expanded program command: "program {binary} verify 0x8080800;"
			init; reset init; halt; program /var/folders/rc/r22l0fy5611279w4ljf1vbn40000gn/T/tmpt0c5iti7.bin verify 0x8080800;  exit"".
## 1024 byte: c_hello

7. 書き出しOK

wrote 129024 bytes from file /var/folders/rc/r22l0fy5611279w4ljf1vbn40000gn/T/tmpt0c5iti7.bin in 3.060769s (41.166 KiB/s)
## 1024 byte: c_hello
** Verified OK **

8. 0x8080c00 から512 バイトを0ffで消去

[DEBUG  ] Erasing page at address 0x8080c00
[DEBUG  ] Using erase command: "flash fillb 0x8080c00 0xff 512;"
[DEBUG  ] Expanded erase command: "flash fillb 0x8080c00 0xff 512;"
			init; reset init; halt; flash fillb 0x8080c00 0xff 512;  exit""

9. 消去OK
wrote 512 bytes to 0x08080c00 in 0.041670s (11.999 KiB/s)

10. install 終了
[INFO   ] Finished in 7.120 seconds
```


## tblファイルの解析

### c_hello

```
00000000: 0200 2c00 0004 0000 0100 0000 db26 2065  ..,..........& e
00000010: 0100 0c00 2900 0000 0000 0000 fc11 0000  ....)...........
00000020: 0300 0700 635f 6865 6c6c 6f00 3402 0000  ....c_hello.4...
00000030: 0008 0000 2000 0000 5402 0000 2008 0000  .... ...T... ...
00000040: 8000 0000 a008 0000 5c01 0000 d402 0000  ........\.......
00000050: 0008 0000 446a 04f1 0704 0c44 0725 24ea  ....Dj.....D.%$.
00000060: 0504 8569 c669 3544 0d44 0600 0f00 9d42  ...i.i5D.D.....B
00000070: 01dc a546 e946 0020 2900 04df 0a20 2100  ...F.F. ).... !.
00000080: 04df 0b20 2900 04df a546 e946 3000 3900  ... )....F.F0.9.

1. TbfHeader
  version:      0002
  header_size:  002c      # 44 byte
  total_size:   00000400  # 1024 byte
  flags:        00000001
  checksum:     652026db

2. Main
  base:             0001: TbfHeaderMain, 000c: 12 byte
  init_fn_offset:   00000029
  protected_size:   00000000
  minimum_ram_size: 000011fc: 4604 byte

3. Package Name
  base:             0003: TbfHeaderPackageName, 0007: 7 byte
  package_name:     c_hello


                                        3402 0000  4...
00000030: 0008 0000 2000 0000 5402 0000 2008 0000  .... ...T... ...
00000040: 8000 0000 a008 0000 5c01 0000 d402 0000  ........\.......
00000050: 0008 0000

Disassembly of section .crt0_header:

80000000 <.crt0_header>:
80000000:   00000234    andeq   r0, r0, r4, lsr r2
80000004:   00000800    andeq   r0, r0, r0, lsl #16
80000008:   00000020    andeq   r0, r0, r0, lsr #32
8000000c:   00000254    andeq   r0, r0, r4, asr r2
80000010:   00000820    andeq   r0, r0, r0, lsr #16
80000014:   00000080    andeq   r0, r0, r0, lsl #1
80000018:   000008a0    andeq   r0, r0, r0, lsr #17
8000001c:   0000015c    andeq   r0, r0, ip, asr r1
80000020:   000002d4    ldrdeq  r0, [r0], -r4
80000024:   00000800    andeq   r0, r0, r0, lsl #16


以下、コード。
                    446a 04f1 0704 0c44 0725 24ea  ....Dj.....D.%$.
00000060: 0504 8569 c669 3544 0d44 0600 0f00 9d42  ...i.i5D.D.....B
00000070: 01dc a546 e946 0020 2900 04df 0a20 2100  ...F.F. ).... !.

Disassembly of section .text:

80000028 <_start>:
80000028:   6a44        ldr r4, [r0, #36]   ; 0x24
8000002a:   f104 0407   add.w   r4, r4, #7
8000002e:   440c        add r4, r1
80000030:   2507        movs    r5, #7
80000032:   ea24 0405   bic.w   r4, r4, r5
80000036:   6985        ldr r5, [r0, #24]
80000038:   69c6        ldr r6, [r0, #28]
```

## STM32F407のFlashメモリ

Flashメモリはセクターごとの書き換えしかできない。12のセクターがある。現在アプリに割り当てている
0x08080000以降には128KB(0x20000)のセクタが4つのみ。したがって登録できるアプリは4つまで。
`const NUM_PROCS: usize = 4;`と定義されているのでちょうどよい。

|セクタ| 長さ | 先頭アドレス |
|:---:|-----:|:-----------|
|  0 |  16 KB | 0x0800 0000 |
|  1 |  16 KB | 0x0800 4000 |
|  2 |  16 KB | 0x0800 8000 |
|  3 |  16 KB | 0x0800 C000 |
|  4 |  64 KB | 0x0801 0000 |
|  5 | 128 KB | 0x0802 0000 |
|  6 | 128 KB | 0x0804 0000 |
|  7 | 128 KB | 0x0806 0000 |
|  8 | 128 KB | 0x0808 0000 |
|  9 | 128 KB | 0x800A 0000 |
| 10 | 128 KB | 0x080C 0000 |
| 11 | 128 KB | 0x080E 0000 |


# すべて解決

解決する方法は次の2つある。

1. アプリのサイズを128KBにする。
2. アプリのインストール時にインストール済みのアプリと新規インストールアプリを結合してから一つのファイルとしてインストールする。

(1)の方法を取るには`elf2tab`の`main.rs`でファイルの最小サイズを`512`としているのを`128KB`に修正すればいけそう。
(2)の方法はすでに実装されているのではないかと調査したところ`bundle-apps`という実行時パラメタがあり、これが(2)を
実装しているようだった。特定のボード向けの実行時パラメタは`tockloader.py`で設定されており、これに習って、
`stm32f407g-disc1`用にも実行時パラメタを設定して実行したところ、想定どうりの処理が行われた。

実際に3つのアプリをインストールし、すべてのアプリが実行されていることが確認できた。

```diff
diff --git a/tockloader/tockloader.py b/tockloader/tockloader.py
index 614d094..0c5418c 100644
--- a/tockloader/tockloader.py
+++ b/tockloader/tockloader.py
@@ -72,6 +72,8 @@ class TockLoader:
        'edu-ciaa': {'cmd_flags': {'bundle_apps': True,
                                   'openocd': True}},
        'arty': {'size_minimum': 0x10000},
+       'stm32f407g-disc1': {'cmd_flags': {'bundle_apps': True,
+                                          'openocd': True}},
    }


```

## (1) blinkをインストール

```
$ tockloader install --board stm32f407g-disc1 --openocd blink/build/blink.tab --debug
[INFO   ] Hardcoding command line argument "bundle_apps" to "True" for board stm32f407g-disc1.
[INFO   ] Hardcoding command line argument "openocd" to "True" for board stm32f407g-disc1.
[INFO   ] Using settings from KNOWN_BOARDS["stm32f407g-disc1"]
[STATUS ] Installing app on the board...
[DEBUG  ] Using read command: "dump_image {{binary}} {address:#x} {length};"
[DEBUG  ] Expanded read command: "dump_image {binary} 0x8080000 200;"
[DEBUG  ] Running "openocd -c "interface hla;                                                    hla_layout stlink;                                                    hla_device_desc "ST-LINK/V2-1";                                                    hla_vid_pid 0x0483 0x374b;                                                    set WORKAREASIZE 0x40000;                                                    source [find target/stm32f4x.cfg];  init; reset init; halt; dump_image /var/folders/rc/r22l0fy5611279w4ljf1vbn40000gn/T/tmpt6tja5gu.bin 0x8080000 200;  exit"".
[INFO   ] Open On-Chip Debugger 0.10.0+dev-01408-g762ddcb74-dirty (2020-09-24-11:11)
Licensed under GNU GPL v2
For bug reports, read
	http://openocd.org/doc/doxygen/bugs.html
DEPRECATED! use 'adapter driver' not 'interface'
Info : auto-selecting first available session transport "hla_swd". To override use 'transport select <transport>'.
Info : The selected transport took over low-level target control. The results might differ compared to plain JTAG/SWD
Info : clock speed 2000 kHz
Info : STLINK V2J37M26 (API v2) VID:PID 0483:374B
Info : Target voltage: 2.901302
Info : stm32f4x.cpu: hardware has 6 breakpoints, 4 watchpoints
Info : starting gdb server for stm32f4x.cpu on 3333
Info : Listening on port 3333 for gdb connections
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
target halted due to debug-request, current mode: Thread
xPSR: 0x01000000 pc: 0x08007e5c msp: 0x20001000
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
Info : Unable to match requested speed 8000 kHz, using 4000 kHz

[DEBUG  ] Using read command: "dump_image {{binary}} {address:#x} {length};"
[DEBUG  ] Expanded read command: "dump_image {binary} 0x8080800 200;"
[DEBUG  ] Running "openocd -c "interface hla;                                                    hla_layout stlink;                                                    hla_device_desc "ST-LINK/V2-1";                                                    hla_vid_pid 0x0483 0x374b;                                                    set WORKAREASIZE 0x40000;                                                    source [find target/stm32f4x.cfg];  init; reset init; halt; dump_image /var/folders/rc/r22l0fy5611279w4ljf1vbn40000gn/T/tmp4b9lf5u1.bin 0x8080800 200;  exit"".
[INFO   ] Open On-Chip Debugger 0.10.0+dev-01408-g762ddcb74-dirty (2020-09-24-11:11)
Licensed under GNU GPL v2
For bug reports, read
	http://openocd.org/doc/doxygen/bugs.html
DEPRECATED! use 'adapter driver' not 'interface'
Info : auto-selecting first available session transport "hla_swd". To override use 'transport select <transport>'.
Info : The selected transport took over low-level target control. The results might differ compared to plain JTAG/SWD
Info : clock speed 2000 kHz
Info : STLINK V2J37M26 (API v2) VID:PID 0483:374B
Info : Target voltage: 2.901302
Info : stm32f4x.cpu: hardware has 6 breakpoints, 4 watchpoints
Info : starting gdb server for stm32f4x.cpu on 3333
Info : Listening on port 3333 for gdb connections
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
target halted due to debug-request, current mode: Thread
xPSR: 0x01000000 pc: 0x08007e5c msp: 0x20001000
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
Info : Unable to match requested speed 8000 kHz, using 4000 kHz

[DEBUG  ] Found 1 app on the board.
[DEBUG  ]   1. blink
[INFO   ] Installing app bundle. Size: 2048 bytes.
[DEBUG  ] Using program command: "program {{binary}} verify {address:#x};"
[DEBUG  ] Expanded program command: "program {binary} verify 0x8080000;"
[DEBUG  ] Running "openocd -c "interface hla;                                                    hla_layout stlink;                                                    hla_device_desc "ST-LINK/V2-1";                                                    hla_vid_pid 0x0483 0x374b;                                                    set WORKAREASIZE 0x40000;                                                    source [find target/stm32f4x.cfg];  init; reset init; halt; program /var/folders/rc/r22l0fy5611279w4ljf1vbn40000gn/T/tmp279vewt7.bin verify 0x8080000;  exit"".
[INFO   ] Open On-Chip Debugger 0.10.0+dev-01408-g762ddcb74-dirty (2020-09-24-11:11)
Licensed under GNU GPL v2
For bug reports, read
	http://openocd.org/doc/doxygen/bugs.html
DEPRECATED! use 'adapter driver' not 'interface'
Info : auto-selecting first available session transport "hla_swd". To override use 'transport select <transport>'.
Info : The selected transport took over low-level target control. The results might differ compared to plain JTAG/SWD
Info : clock speed 2000 kHz
Info : STLINK V2J37M26 (API v2) VID:PID 0483:374B
Info : Target voltage: 2.901302
Info : stm32f4x.cpu: hardware has 6 breakpoints, 4 watchpoints
Info : starting gdb server for stm32f4x.cpu on 3333
Info : Listening on port 3333 for gdb connections
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
target halted due to debug-request, current mode: Thread
xPSR: 0x01000000 pc: 0x08007e5c msp: 0x20001000
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
target halted due to debug-request, current mode: Thread
xPSR: 0x01000000 pc: 0x08007e5c msp: 0x20001000
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
** Programming Started **
Info : device id = 0x10076413
Info : flash size = 1024 kbytes
** Programming Finished **
** Verify Started **
** Verified OK **

[DEBUG  ] Erasing page at address 0x8080800
[DEBUG  ] Using erase command: "flash fillb 0x8080800 0xff 512;"
[DEBUG  ] Expanded erase command: "flash fillb 0x8080800 0xff 512;"
[DEBUG  ] Running "openocd -c "interface hla;                                                    hla_layout stlink;                                                    hla_device_desc "ST-LINK/V2-1";                                                    hla_vid_pid 0x0483 0x374b;                                                    set WORKAREASIZE 0x40000;                                                    source [find target/stm32f4x.cfg];  init; reset init; halt; flash fillb 0x8080800 0xff 512;  exit"".
[INFO   ] Open On-Chip Debugger 0.10.0+dev-01408-g762ddcb74-dirty (2020-09-24-11:11)
Licensed under GNU GPL v2
For bug reports, read
	http://openocd.org/doc/doxygen/bugs.html
DEPRECATED! use 'adapter driver' not 'interface'
Info : auto-selecting first available session transport "hla_swd". To override use 'transport select <transport>'.
Info : The selected transport took over low-level target control. The results might differ compared to plain JTAG/SWD
Info : clock speed 2000 kHz
Info : STLINK V2J37M26 (API v2) VID:PID 0483:374B
Info : Target voltage: 2.901302
Info : stm32f4x.cpu: hardware has 6 breakpoints, 4 watchpoints
Info : starting gdb server for stm32f4x.cpu on 3333
Info : Listening on port 3333 for gdb connections
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
target halted due to debug-request, current mode: Thread
xPSR: 0x01000000 pc: 0x08007e5c msp: 0x20001000
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
Info : device id = 0x10076413
Info : flash size = 1024 kbytes

[INFO   ] Finished in 4.135 seconds
```

## (1.1) blinkは`0x08080000`にインストールされた

```
$ tockloader read --board stm32f407g-disc1 --openocd 0x8080000 0x100
[INFO   ] Hardcoding command line argument "bundle_apps" to "True" for board stm32f407g-disc1.
[INFO   ] Hardcoding command line argument "openocd" to "True" for board stm32f407g-disc1.
[INFO   ] Using settings from KNOWN_BOARDS["stm32f407g-disc1"]
[STATUS ] Reading flash from the board...
[STATUS ]   Address: 0x8080000
[STATUS ]   Length:  256 bytes
08080000  02 00 2c 00 00 08 00 00  01 00 00 00 d5 75 4c 6e  |..,..........uLn|
08080010  01 00 0c 00 29 00 00 00  00 00 00 00 f4 11 00 00  |....)...........|
08080020  03 00 05 00 62 6c 69 6e  6b 00 00 00 04 04 00 00  |....blink.......|
08080030  00 08 00 00 24 00 00 00  28 04 00 00 24 08 00 00  |....$...(...$...|
08080040  70 00 00 00 94 08 00 00  60 01 00 00 98 04 00 00  |p.......`.......|
08080050  00 08 00 00 44 6a 04 f1  07 04 0c 44 07 25 24 ea  |....Dj.....D.%$.|
08080060  05 04 85 69 c6 69 35 44  0d 44 06 00 0f 00 9d 42  |...i.i5D.D.....B|
08080070  01 dc a5 46 e9 46 00 20  29 00 04 df 0a 20 21 00  |...F.F. ).... !.|
08080080  04 df 0b 20 29 00 04 df  a5 46 e9 46 30 00 39 00  |... )....F.F0.9.|
08080090  00 f0 c0 f8 ff be 70 b5  00 f0 fc f8 00 25 06 46  |......p......%.F|
080800a0  00 24 b4 42 04 db fa 20  00 f0 9e f8 01 35 f7 e7  |.$.B... .....5..|
080800b0  45 fa 04 f3 db 07 20 46  03 d5 00 f0 f1 f8 01 34  |E..... F.......4|
080800c0  ef e7 00 f0 f3 f8 fa e7  01 22 1a 70 70 47 00 23  |.........".ppG.#|
080800d0  1a 46 02 21 18 46 00 f0  55 b9 ff ff f8 b5 1c 46  |.F.!.F..U......F|
080800e0  0f 46 16 46 05 46 ff f7  f2 ff 00 22 c4 e9 00 05  |.F.F.F....."....|
080800f0  c4 e9 02 76 c4 e9 04 22  14 4b 59 f8 03 10 0b 68  |...v...".KY....h|
[INFO   ] Finished in 0.239 seconds
```

## (2) buttonsをインストール

```
$ tockloader install --board stm32f407g-disc1 --openocd --debug buttons/build/buttons.tab
[INFO   ] Hardcoding command line argument "bundle_apps" to "True" for board stm32f407g-disc1.
[INFO   ] Hardcoding command line argument "openocd" to "True" for board stm32f407g-disc1.
[INFO   ] Using settings from KNOWN_BOARDS["stm32f407g-disc1"]
[STATUS ] Installing app on the board...
[DEBUG  ] Using read command: "dump_image {{binary}} {address:#x} {length};"
[DEBUG  ] Expanded read command: "dump_image {binary} 0x8080000 200;"
[DEBUG  ] Running "openocd -c "interface hla;                                                    hla_layout stlink;                                                    hla_device_desc "ST-LINK/V2-1";                                                    hla_vid_pid 0x0483 0x374b;                                                    set WORKAREASIZE 0x40000;                                                    source [find target/stm32f4x.cfg];  init; reset init; halt; dump_image /var/folders/rc/r22l0fy5611279w4ljf1vbn40000gn/T/tmp1wasc06b.bin 0x8080000 200;  exit"".
[INFO   ] Open On-Chip Debugger 0.10.0+dev-01408-g762ddcb74-dirty (2020-09-24-11:11)
Licensed under GNU GPL v2
For bug reports, read
	http://openocd.org/doc/doxygen/bugs.html
DEPRECATED! use 'adapter driver' not 'interface'
Info : auto-selecting first available session transport "hla_swd". To override use 'transport select <transport>'.
Info : The selected transport took over low-level target control. The results might differ compared to plain JTAG/SWD
Info : clock speed 2000 kHz
Info : STLINK V2J37M26 (API v2) VID:PID 0483:374B
Info : Target voltage: 2.899882
Info : stm32f4x.cpu: hardware has 6 breakpoints, 4 watchpoints
Info : starting gdb server for stm32f4x.cpu on 3333
Info : Listening on port 3333 for gdb connections
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
target halted due to debug-request, current mode: Thread
xPSR: 0x01000000 pc: 0x08007e5c msp: 0x20001000
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
Info : Unable to match requested speed 8000 kHz, using 4000 kHz

[DEBUG  ] Using read command: "dump_image {{binary}} {address:#x} {length};"
[DEBUG  ] Expanded read command: "dump_image {binary} 0x8080800 200;"
[DEBUG  ] Running "openocd -c "interface hla;                                                    hla_layout stlink;                                                    hla_device_desc "ST-LINK/V2-1";                                                    hla_vid_pid 0x0483 0x374b;                                                    set WORKAREASIZE 0x40000;                                                    source [find target/stm32f4x.cfg];  init; reset init; halt; dump_image /var/folders/rc/r22l0fy5611279w4ljf1vbn40000gn/T/tmpsl6268a1.bin 0x8080800 200;  exit"".
[INFO   ] Open On-Chip Debugger 0.10.0+dev-01408-g762ddcb74-dirty (2020-09-24-11:11)
Licensed under GNU GPL v2
For bug reports, read
	http://openocd.org/doc/doxygen/bugs.html
DEPRECATED! use 'adapter driver' not 'interface'
Info : auto-selecting first available session transport "hla_swd". To override use 'transport select <transport>'.
Info : The selected transport took over low-level target control. The results might differ compared to plain JTAG/SWD
Info : clock speed 2000 kHz
Info : STLINK V2J37M26 (API v2) VID:PID 0483:374B
Info : Target voltage: 2.899882
Info : stm32f4x.cpu: hardware has 6 breakpoints, 4 watchpoints
Info : starting gdb server for stm32f4x.cpu on 3333
Info : Listening on port 3333 for gdb connections
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
target halted due to debug-request, current mode: Thread
xPSR: 0x01000000 pc: 0x08007e5c msp: 0x20001000
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
Info : Unable to match requested speed 8000 kHz, using 4000 kHz

[DEBUG  ] Found 1 app on the board.
[DEBUG  ]   1. blink
[INFO   ] Reading app blink binary from board.
[DEBUG  ] Using read command: "dump_image {{binary}} {address:#x} {length};"
[DEBUG  ] Expanded read command: "dump_image {binary} 0x8080000 2048;"
[DEBUG  ] Running "openocd -c "interface hla;                                                    hla_layout stlink;                                                    hla_device_desc "ST-LINK/V2-1";                                                    hla_vid_pid 0x0483 0x374b;                                                    set WORKAREASIZE 0x40000;                                                    source [find target/stm32f4x.cfg];  init; reset init; halt; dump_image /var/folders/rc/r22l0fy5611279w4ljf1vbn40000gn/T/tmpcmzzm56_.bin 0x8080000 2048;  exit"".
[INFO   ] Open On-Chip Debugger 0.10.0+dev-01408-g762ddcb74-dirty (2020-09-24-11:11)
Licensed under GNU GPL v2
For bug reports, read
	http://openocd.org/doc/doxygen/bugs.html
DEPRECATED! use 'adapter driver' not 'interface'
Info : auto-selecting first available session transport "hla_swd". To override use 'transport select <transport>'.
Info : The selected transport took over low-level target control. The results might differ compared to plain JTAG/SWD
Info : clock speed 2000 kHz
Info : STLINK V2J37M26 (API v2) VID:PID 0483:374B
Info : Target voltage: 2.901302
Info : stm32f4x.cpu: hardware has 6 breakpoints, 4 watchpoints
Info : starting gdb server for stm32f4x.cpu on 3333
Info : Listening on port 3333 for gdb connections
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
target halted due to debug-request, current mode: Thread
xPSR: 0x01000000 pc: 0x08007e5c msp: 0x20001000
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
Info : Unable to match requested speed 8000 kHz, using 4000 kHz

[INFO   ] Installing app bundle. Size: 3072 bytes.
[DEBUG  ] Using program command: "program {{binary}} verify {address:#x};"
[DEBUG  ] Expanded program command: "program {binary} verify 0x8080000;"
[DEBUG  ] Running "openocd -c "interface hla;                                                    hla_layout stlink;                                                    hla_device_desc "ST-LINK/V2-1";                                                    hla_vid_pid 0x0483 0x374b;                                                    set WORKAREASIZE 0x40000;                                                    source [find target/stm32f4x.cfg];  init; reset init; halt; program /var/folders/rc/r22l0fy5611279w4ljf1vbn40000gn/T/tmpqthef3r9.bin verify 0x8080000;  exit"".
[INFO   ] Open On-Chip Debugger 0.10.0+dev-01408-g762ddcb74-dirty (2020-09-24-11:11)
Licensed under GNU GPL v2
For bug reports, read
	http://openocd.org/doc/doxygen/bugs.html
DEPRECATED! use 'adapter driver' not 'interface'
Info : auto-selecting first available session transport "hla_swd". To override use 'transport select <transport>'.
Info : The selected transport took over low-level target control. The results might differ compared to plain JTAG/SWD
Info : clock speed 2000 kHz
Info : STLINK V2J37M26 (API v2) VID:PID 0483:374B
Info : Target voltage: 2.901302
Info : stm32f4x.cpu: hardware has 6 breakpoints, 4 watchpoints
Info : starting gdb server for stm32f4x.cpu on 3333
Info : Listening on port 3333 for gdb connections
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
target halted due to debug-request, current mode: Thread
xPSR: 0x01000000 pc: 0x08007e5c msp: 0x20001000
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
target halted due to debug-request, current mode: Thread
xPSR: 0x01000000 pc: 0x08007e5c msp: 0x20001000
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
** Programming Started **
Info : device id = 0x10076413
Info : flash size = 1024 kbytes
** Programming Finished **
** Verify Started **
** Verified OK **

[DEBUG  ] Erasing page at address 0x8080c00
[DEBUG  ] Using erase command: "flash fillb 0x8080c00 0xff 512;"
[DEBUG  ] Expanded erase command: "flash fillb 0x8080c00 0xff 512;"
[DEBUG  ] Running "openocd -c "interface hla;                                                    hla_layout stlink;                                                    hla_device_desc "ST-LINK/V2-1";                                                    hla_vid_pid 0x0483 0x374b;                                                    set WORKAREASIZE 0x40000;                                                    source [find target/stm32f4x.cfg];  init; reset init; halt; flash fillb 0x8080c00 0xff 512;  exit"".
[INFO   ] Open On-Chip Debugger 0.10.0+dev-01408-g762ddcb74-dirty (2020-09-24-11:11)
Licensed under GNU GPL v2
For bug reports, read
	http://openocd.org/doc/doxygen/bugs.html
DEPRECATED! use 'adapter driver' not 'interface'
Info : auto-selecting first available session transport "hla_swd". To override use 'transport select <transport>'.
Info : The selected transport took over low-level target control. The results might differ compared to plain JTAG/SWD
Info : clock speed 2000 kHz
Info : STLINK V2J37M26 (API v2) VID:PID 0483:374B
Info : Target voltage: 2.901302
Info : stm32f4x.cpu: hardware has 6 breakpoints, 4 watchpoints
Info : starting gdb server for stm32f4x.cpu on 3333
Info : Listening on port 3333 for gdb connections
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
target halted due to debug-request, current mode: Thread
xPSR: 0x01000000 pc: 0x08007e5c msp: 0x20001000
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
Info : device id = 0x10076413
Info : flash size = 1024 kbytes

[INFO   ] Finished in 4.405 seconds
```

## (2.1) blinkとbuttonsがインストールされていることを確認

```
$ tockloader read --board stm32f407g-disc1 --openocd 0x8080000 0x1000
[INFO   ] Hardcoding command line argument "bundle_apps" to "True" for board stm32f407g-disc1.
[INFO   ] Hardcoding command line argument "openocd" to "True" for board stm32f407g-disc1.
[INFO   ] Using settings from KNOWN_BOARDS["stm32f407g-disc1"]
[STATUS ] Reading flash from the board...
[STATUS ]   Address: 0x8080000
[STATUS ]   Length:  4096 bytes
08080000  02 00 2c 00 00 08 00 00  01 00 00 00 d5 75 4c 6e  |..,..........uLn|
08080010  01 00 0c 00 29 00 00 00  00 00 00 00 f4 11 00 00  |....)...........|
08080020  03 00 05 00 62 6c 69 6e  6b 00 00 00 04 04 00 00  |....blink.......|
08080030  00 08 00 00 24 00 00 00  28 04 00 00 24 08 00 00  |....$...(...$...|
08080040  70 00 00 00 94 08 00 00  60 01 00 00 98 04 00 00  |p.......`.......|
08080050  00 08 00 00 44 6a 04 f1  07 04 0c 44 07 25 24 ea  |....Dj.....D.%$.|
08080060  05 04 85 69 c6 69 35 44  0d 44 06 00 0f 00 9d 42  |...i.i5D.D.....B|
08080070  01 dc a5 46 e9 46 00 20  29 00 04 df 0a 20 21 00  |...F.F. ).... !.|
08080080  04 df 0b 20 29 00 04 df  a5 46 e9 46 30 00 39 00  |... )....F.F0.9.|
08080090  00 f0 c0 f8 ff be 70 b5  00 f0 fc f8 00 25 06 46  |......p......%.F|
080800a0  00 24 b4 42 04 db fa 20  00 f0 9e f8 01 35 f7 e7  |.$.B... .....5..|
080800b0  45 fa 04 f3 db 07 20 46  03 d5 00 f0 f1 f8 01 34  |E..... F.......4|
080800c0  ef e7 00 f0 f3 f8 fa e7  01 22 1a 70 70 47 00 23  |.........".ppG.#|
080800d0  1a 46 02 21 18 46 00 f0  55 b9 ff ff f8 b5 1c 46  |.F.!.F..U......F|
080800e0  0f 46 16 46 05 46 ff f7  f2 ff 00 22 c4 e9 00 05  |.F.F.F....."....|
080800f0  c4 e9 02 76 c4 e9 04 22  14 4b 59 f8 03 10 0b 68  |...v...".KY....h|
08080100  5b b9 0c 60 12 4b 00 21  59 f8 03 00 00 f0 3c f9  |[..`.K.!Y.....<.|
08080110  60 68 bd e8 f8 40 00 f0  3d b9 0e 46 58 68 28 1a  |`h...@..=..FXh(.|
08080120  00 28 09 da 34 60 c4 e9  04 32 5c 61 0b 68 9c 42  |.(..4`...2\a.h.B|
08080130  e8 d0 f8 bd 03 46 f1 e7  18 69 03 f1 10 06 1a 46  |.....F...i.....F|
08080140  00 28 f7 d1 1c 61 c4 e9  04 03 ef e7 08 00 00 00  |.(...a..........|
08080150  04 00 00 00 73 b5 13 4b  59 f8 03 50 2c 68 00 26  |....s..KY..P,h.&|
08080160  0c b9 02 b0 70 bd ff f7  b2 ff d4 e9 00 32 01 46  |....p........2.F|
08080170  d0 1a cb 1a 98 42 05 d9  10 46 02 b0 bd e8 70 40  |.....B...F....p@|
08080180  00 f0 08 b9 2b 68 23 b1  18 69 28 60 00 b1 46 61  |....+h#..i(`..Fa|
08080190  1e 61 a0 68 20 b1 e3 68  00 93 00 23 00 f0 8c f8  |.a.h ..h...#....|
080801a0  2c 68 dd e7 08 00 00 00  2d e9 f0 4d 04 46 0e 46  |,h......-..M.F.F|
080801b0  17 46 98 46 00 f0 f4 f8  4f f4 7a 73 b4 fb f3 f5  |.F.F....O.zs....|
080801c0  00 fb 05 fa 03 fb 15 45  b0 fb f3 f0 45 43 ff f7  |.......E....EC..|
080801d0  7e ff 50 44 08 f1 0c 03  3a 46 31 46 28 44 bd e8  |~.PD....:F1F(D..|
080801e0  f0 4d ff f7 7b bf ff ff  10 b5 8a b0 00 23 8d f8  |.M..{........#..|
080801f0  03 30 07 4b 0d f1 03 04  59 f8 03 10 22 46 01 ab  |.0.K....Y..."F..|
08080200  ff f7 d2 ff 20 46 00 f0  b3 f8 0a b0 10 bd 00 bf  |.... F..........|
08080210  00 00 00 00 04 46 40 68  22 68 08 b5 0d 46 08 44  |.....F@h"h...F.D|
08080220  22 44 00 23 a1 68 b3 eb  91 0f 19 d3 d4 e9 03 10  |"D.#.h..........|
08080230  62 69 21 44 28 44 00 f0  b9 f8 d4 e9 06 02 00 21  |bi!D(D.........!|
08080240  28 44 00 f0 bc f8 21 6a  21 44 00 22 0e 1d 0b 68  |(D....!j!D."...h|
08080250  b2 eb 93 0f 12 d3 ff f7  1e ff 00 f0 5d f8 fc e7  |............]...|
08080260  52 f8 23 10 00 29 52 f8  23 10 b6 bf 01 f1 00 41  |R.#..)R.#......A|
08080270  49 19 09 19 40 f8 23 10  01 33 d3 e7 56 f8 22 00  |I...@.#..3..V.".|
08080280  2b 58 00 2b b6 bf 03 f1  00 43 5b 19 1b 19 2b 50  |+X.+.....C[...+P|
08080290  02 32 dc e7 00 23 1a 46  19 46 02 20 00 f0 72 b8  |.2...#.F.F. ..r.|
080802a0  02 46 00 23 01 21 02 20  00 f0 6c b8 02 21 02 46  |.F.#.!. ..l..!.F|
080802b0  00 23 08 46 00 f0 66 b8  f0 b5 07 46 13 48 59 f8  |.#.F..f....F.HY.|
080802c0  00 c0 dc f8 00 e0 0e f1  01 06 74 42 04 f0 0f 04  |..........tB....|
080802d0  06 f0 0f 00 58 bf 60 42  0d 4c 59 f8 04 40 24 68  |....X.`B.LY..@$h|
080802e0  84 42 0f d0 0b 4c 14 25  59 f8 04 60 cc f8 00 00  |.B...L.%Y..`....|
080802f0  05 fb 0e f5 74 19 c4 e9  01 12 e3 60 77 51 05 9b  |....t......`wQ..|
08080300  23 61 f0 bd 4f f0 ff 30  fb e7 00 bf 10 00 00 00  |#a..O..0........|
08080310  0c 00 00 00 14 00 00 00  10 b5 12 4b 12 4a 59 f8  |...........K.JY.|
08080320  03 10 59 f8 02 20 0b 68  12 68 93 42 17 d0 0f 4a  |..Y.. .h.h.B...J|
08080330  59 f8 02 40 14 22 5a 43  01 33 a0 18 a4 58 5a 42  |Y..@."ZC.3...XZB|
08080340  02 f0 0f 02 03 f0 0f 03  58 bf 53 42 0b 60 a4 46  |........X.SB.`.F|
08080350  d0 e9 03 23 bd e8 10 40  d0 e9 01 01 60 47 00 df  |...#...@....`G..|
08080360  10 bd 00 bf 0c 00 00 00  10 00 00 00 14 00 00 00  |................|
08080370  10 b5 04 46 23 78 03 b1  10 bd ff f7 cd ff f9 e7  |...F#x..........|
08080380  01 df 70 47 02 df 70 47  0b 46 00 21 02 46 08 46  |..pG..pG.F.!.F.F|
08080390  ff f7 f6 bf 00 23 02 46  04 21 18 46 ff f7 f2 bf  |.....#.F.!.F....|
080803a0  00 23 1a 46 01 21 18 46  ff f7 ec bf 00 23 10 b5  |.#.F.!.F.....#..|
080803b0  9a 42 00 d1 10 bd cc 5c  c4 54 01 33 f8 e7 03 00  |.B.....\.T.3....|
080803c0  82 18 93 42 00 d1 70 47  19 70 01 33 f9 e7 ff ff  |...B..pG.p.3....|
080803d0  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
080803e0  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
080803f0  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080400  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080410  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080420  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080430  9d 00 00 80 29 01 00 80  94 08 00 00 98 08 00 00  |....)...........|
08080440  9c 08 00 00 a0 08 00 00  00 00 00 00 00 00 00 00  |................|
08080450  00 00 00 00 00 00 00 00  c4 03 00 80 e4 03 00 80  |................|
08080460  a4 03 00 80 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080470  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080480  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080490  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
080804a0  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
080804b0  00 00 00 00 24 08 00 00  24 08 00 00 01 00 00 00  |....$...$.......|
080804c0  00 00 00 00 28 00 00 00  28 08 00 00 02 be 00 00  |....(...(.......|
080804d0  2c 08 00 00 02 c8 00 00  30 08 00 00 02 b7 00 00  |,.......0.......|
（略）
080807e0  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
080807f0  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080800  02 00 2c 00 00 04 00 00  01 00 00 00 cd 0e 20 74  |..,........... t|
08080810  01 00 0c 00 29 00 00 00  00 00 00 00 e8 11 00 00  |....)...........|
08080820  03 00 07 00 62 75 74 74  6f 6e 73 00 34 02 00 00  |....buttons.4...|
08080830  00 08 00 00 1c 00 00 00  50 02 00 00 1c 08 00 00  |........P.......|
08080840  70 00 00 00 8c 08 00 00  5c 01 00 00 c0 02 00 00  |p.......\.......|
08080850  00 08 00 00 44 6a 04 f1  07 04 0c 44 07 25 24 ea  |....Dj.....D.%$.|
08080860  05 04 85 69 c6 69 35 44  0d 44 06 00 0f 00 9d 42  |...i.i5D.D.....B|
08080870  01 dc a5 46 e9 46 00 20  29 00 04 df 0a 20 21 00  |...F.F. ).... !.|
08080880  04 df 0b 20 29 00 04 df  a5 46 e9 46 30 00 39 00  |... )....F.F0.9.|
08080890  00 f0 2e f8 ff be 01 29  01 d1 00 f0 69 b8 70 47  |.......)....i.pG|
080808a0  38 b5 09 4b 00 21 59 f8  03 00 00 f0 0f f8 00 f0  |8..K.!Y.........|
080808b0  13 f8 00 24 05 46 ac 42  01 db 00 20 38 bd 20 46  |...$.F.B... 8. F|
080808c0  00 f0 10 f8 01 34 f6 e7  00 00 00 00 02 46 0b 46  |.....4.......F.F|
080808d0  03 20 00 21 00 f0 7e b8  00 23 1a 46 19 46 03 20  |. .!..~..#.F.F. |
080808e0  00 f0 7a b8 02 46 00 23  01 21 03 20 00 f0 74 b8  |..z..F.#.!. ..t.|
080808f0  04 46 40 68 22 68 08 b5  0d 46 08 44 22 44 00 23  |.F@h"h...F.D"D.#|
08080900  a1 68 b3 eb 91 0f 19 d3  d4 e9 03 10 62 69 21 44  |.h..........bi!D|
08080910  28 44 00 f0 63 f8 d4 e9  06 02 00 21 28 44 00 f0  |(D..c......!(D..|
08080920  66 f8 21 6a 21 44 00 22  0e 1d 0b 68 b2 eb 93 0f  |f.!j!D."...h....|
08080930  12 d3 ff f7 b5 ff 00 f0  21 f8 fc e7 52 f8 23 10  |........!...R.#.|
08080940  00 29 52 f8 23 10 b6 bf  01 f1 00 41 49 19 09 19  |.)R.#......AI...|
08080950  40 f8 23 10 01 33 d3 e7  56 f8 22 00 2b 58 00 2b  |@.#..3..V.".+X.+|
08080960  b6 bf 03 f1 00 43 5b 19  1b 19 2b 50 02 32 dc e7  |.....C[...+P.2..|
08080970  02 46 00 23 03 21 02 20  00 f0 2e b8 10 b5 12 4b  |.F.#.!. .......K|
08080980  12 4a 59 f8 03 10 59 f8  02 20 0b 68 12 68 93 42  |.JY...Y.. .h.h.B|
08080990  17 d0 0f 4a 59 f8 02 40  14 22 5a 43 01 33 a0 18  |...JY..@."ZC.3..|
080809a0  a4 58 5a 42 02 f0 0f 02  03 f0 0f 03 58 bf 53 42  |.XZB........X.SB|
080809b0  0b 60 a4 46 d0 e9 03 23  bd e8 10 40 d0 e9 01 01  |.`.F...#...@....|
080809c0  60 47 00 df 10 bd 00 bf  04 00 00 00 08 00 00 00  |`G..............|
080809d0  0c 00 00 00 01 df 70 47  02 df 70 47 00 23 10 b5  |......pG..pG.#..|
080809e0  9a 42 00 d1 10 bd cc 5c  c4 54 01 33 f8 e7 03 00  |.B.....\.T.3....|
080809f0  82 18 93 42 00 d1 70 47  19 70 01 33 f9 e7 ff ff  |...B..pG.p.3....|
08080a00  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080a10  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080a20  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080a30  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080a40  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080a50  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080a60  6b 00 00 80 8c 08 00 00  90 08 00 00 94 08 00 00  |k...............|
08080a70  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080a80  f4 01 00 80 14 02 00 80  d4 01 00 80 00 00 00 00  |................|
08080a90  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080aa0  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080ab0  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080ac0  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080ad0  00 00 00 00 00 00 00 00  00 00 00 00 1c 08 00 00  |................|
08080ae0  1c 08 00 00 01 00 00 00  00 00 00 00 28 00 00 00  |............(...|
08080af0  20 08 00 00 02 a5 00 00  24 08 00 00 02 ae 00 00  | .......$.......|
08080b00  28 08 00 00 02 9f 00 00  7c 08 00 00 02 03 00 00  |(.......|.......|
08080b10  80 08 00 00 02 03 00 00  00 00 00 00 00 00 00 00  |................|
08080b20  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
（略）
08080be0  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080bf0  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080c00  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
08080c10  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
（略）
[INFO   ] Finished in 0.267 seconds
```

## (3) c_helloをインストール

```
$ tockloader install --board stm32f407g-disc1 --openocd --debug c_hello/build/c_hello.tab
[INFO   ] Hardcoding command line argument "bundle_apps" to "True" for board stm32f407g-disc1.
[INFO   ] Hardcoding command line argument "openocd" to "True" for board stm32f407g-disc1.
[INFO   ] Using settings from KNOWN_BOARDS["stm32f407g-disc1"]
[STATUS ] Installing app on the board...
[DEBUG  ] Using read command: "dump_image {{binary}} {address:#x} {length};"
[DEBUG  ] Expanded read command: "dump_image {binary} 0x8080000 200;"
[DEBUG  ] Running "openocd -c "interface hla;                                                    hla_layout stlink;                                                    hla_device_desc "ST-LINK/V2-1";                                                    hla_vid_pid 0x0483 0x374b;                                                    set WORKAREASIZE 0x40000;                                                    source [find target/stm32f4x.cfg];  init; reset init; halt; dump_image /var/folders/rc/r22l0fy5611279w4ljf1vbn40000gn/T/tmpt5p1t6jt.bin 0x8080000 200;  exit"".
[INFO   ] Open On-Chip Debugger 0.10.0+dev-01408-g762ddcb74-dirty (2020-09-24-11:11)
Licensed under GNU GPL v2
For bug reports, read
	http://openocd.org/doc/doxygen/bugs.html
DEPRECATED! use 'adapter driver' not 'interface'
Info : auto-selecting first available session transport "hla_swd". To override use 'transport select <transport>'.
Info : The selected transport took over low-level target control. The results might differ compared to plain JTAG/SWD
Info : clock speed 2000 kHz
Info : STLINK V2J37M26 (API v2) VID:PID 0483:374B
Info : Target voltage: 2.901302
Info : stm32f4x.cpu: hardware has 6 breakpoints, 4 watchpoints
Info : starting gdb server for stm32f4x.cpu on 3333
Info : Listening on port 3333 for gdb connections
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
target halted due to debug-request, current mode: Thread
xPSR: 0x01000000 pc: 0x08007e5c msp: 0x20001000
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
Info : Unable to match requested speed 8000 kHz, using 4000 kHz

[DEBUG  ] Using read command: "dump_image {{binary}} {address:#x} {length};"
[DEBUG  ] Expanded read command: "dump_image {binary} 0x8080800 200;"
[DEBUG  ] Running "openocd -c "interface hla;                                                    hla_layout stlink;                                                    hla_device_desc "ST-LINK/V2-1";                                                    hla_vid_pid 0x0483 0x374b;                                                    set WORKAREASIZE 0x40000;                                                    source [find target/stm32f4x.cfg];  init; reset init; halt; dump_image /var/folders/rc/r22l0fy5611279w4ljf1vbn40000gn/T/tmp8ux6jwps.bin 0x8080800 200;  exit"".
[INFO   ] Open On-Chip Debugger 0.10.0+dev-01408-g762ddcb74-dirty (2020-09-24-11:11)
Licensed under GNU GPL v2
For bug reports, read
	http://openocd.org/doc/doxygen/bugs.html
DEPRECATED! use 'adapter driver' not 'interface'
Info : auto-selecting first available session transport "hla_swd". To override use 'transport select <transport>'.
Info : The selected transport took over low-level target control. The results might differ compared to plain JTAG/SWD
Info : clock speed 2000 kHz
Info : STLINK V2J37M26 (API v2) VID:PID 0483:374B
Info : Target voltage: 2.901302
Info : stm32f4x.cpu: hardware has 6 breakpoints, 4 watchpoints
Info : starting gdb server for stm32f4x.cpu on 3333
Info : Listening on port 3333 for gdb connections
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
target halted due to debug-request, current mode: Thread
xPSR: 0x01000000 pc: 0x08007e5c msp: 0x20001000
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
Info : Unable to match requested speed 8000 kHz, using 4000 kHz

[DEBUG  ] Using read command: "dump_image {{binary}} {address:#x} {length};"
[DEBUG  ] Expanded read command: "dump_image {binary} 0x8080c00 200;"
[DEBUG  ] Running "openocd -c "interface hla;                                                    hla_layout stlink;                                                    hla_device_desc "ST-LINK/V2-1";                                                    hla_vid_pid 0x0483 0x374b;                                                    set WORKAREASIZE 0x40000;                                                    source [find target/stm32f4x.cfg];  init; reset init; halt; dump_image /var/folders/rc/r22l0fy5611279w4ljf1vbn40000gn/T/tmpcm30gvbh.bin 0x8080c00 200;  exit"".
[INFO   ] Open On-Chip Debugger 0.10.0+dev-01408-g762ddcb74-dirty (2020-09-24-11:11)
Licensed under GNU GPL v2
For bug reports, read
	http://openocd.org/doc/doxygen/bugs.html
DEPRECATED! use 'adapter driver' not 'interface'
Info : auto-selecting first available session transport "hla_swd". To override use 'transport select <transport>'.
Info : The selected transport took over low-level target control. The results might differ compared to plain JTAG/SWD
Info : clock speed 2000 kHz
Info : STLINK V2J37M26 (API v2) VID:PID 0483:374B
Info : Target voltage: 2.901302
Info : stm32f4x.cpu: hardware has 6 breakpoints, 4 watchpoints
Info : starting gdb server for stm32f4x.cpu on 3333
Info : Listening on port 3333 for gdb connections
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
target halted due to debug-request, current mode: Thread
xPSR: 0x01000000 pc: 0x08007e5c msp: 0x20001000
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
Info : Unable to match requested speed 8000 kHz, using 4000 kHz

[DEBUG  ] Found 2 apps on the board.
[DEBUG  ]   1. blink
[DEBUG  ]   2. buttons
[INFO   ] Reading app blink binary from board.
[DEBUG  ] Using read command: "dump_image {{binary}} {address:#x} {length};"
[DEBUG  ] Expanded read command: "dump_image {binary} 0x8080000 2048;"
[DEBUG  ] Running "openocd -c "interface hla;                                                    hla_layout stlink;                                                    hla_device_desc "ST-LINK/V2-1";                                                    hla_vid_pid 0x0483 0x374b;                                                    set WORKAREASIZE 0x40000;                                                    source [find target/stm32f4x.cfg];  init; reset init; halt; dump_image /var/folders/rc/r22l0fy5611279w4ljf1vbn40000gn/T/tmp9voqlfq7.bin 0x8080000 2048;  exit"".
[INFO   ] Open On-Chip Debugger 0.10.0+dev-01408-g762ddcb74-dirty (2020-09-24-11:11)
Licensed under GNU GPL v2
For bug reports, read
	http://openocd.org/doc/doxygen/bugs.html
DEPRECATED! use 'adapter driver' not 'interface'
Info : auto-selecting first available session transport "hla_swd". To override use 'transport select <transport>'.
Info : The selected transport took over low-level target control. The results might differ compared to plain JTAG/SWD
Info : clock speed 2000 kHz
Info : STLINK V2J37M26 (API v2) VID:PID 0483:374B
Info : Target voltage: 2.901302
Info : stm32f4x.cpu: hardware has 6 breakpoints, 4 watchpoints
Info : starting gdb server for stm32f4x.cpu on 3333
Info : Listening on port 3333 for gdb connections
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
target halted due to debug-request, current mode: Thread
xPSR: 0x01000000 pc: 0x08007e5c msp: 0x20001000
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
Info : Unable to match requested speed 8000 kHz, using 4000 kHz

[INFO   ] Reading app buttons binary from board.
[DEBUG  ] Using read command: "dump_image {{binary}} {address:#x} {length};"
[DEBUG  ] Expanded read command: "dump_image {binary} 0x8080800 1024;"
[DEBUG  ] Running "openocd -c "interface hla;                                                    hla_layout stlink;                                                    hla_device_desc "ST-LINK/V2-1";                                                    hla_vid_pid 0x0483 0x374b;                                                    set WORKAREASIZE 0x40000;                                                    source [find target/stm32f4x.cfg];  init; reset init; halt; dump_image /var/folders/rc/r22l0fy5611279w4ljf1vbn40000gn/T/tmpr_rbqrw4.bin 0x8080800 1024;  exit"".
[INFO   ] Open On-Chip Debugger 0.10.0+dev-01408-g762ddcb74-dirty (2020-09-24-11:11)
Licensed under GNU GPL v2
For bug reports, read
	http://openocd.org/doc/doxygen/bugs.html
DEPRECATED! use 'adapter driver' not 'interface'
Info : auto-selecting first available session transport "hla_swd". To override use 'transport select <transport>'.
Info : The selected transport took over low-level target control. The results might differ compared to plain JTAG/SWD
Info : clock speed 2000 kHz
Info : STLINK V2J37M26 (API v2) VID:PID 0483:374B
Info : Target voltage: 2.901302
Info : stm32f4x.cpu: hardware has 6 breakpoints, 4 watchpoints
Info : starting gdb server for stm32f4x.cpu on 3333
Info : Listening on port 3333 for gdb connections
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
target halted due to debug-request, current mode: Thread
xPSR: 0x01000000 pc: 0x08007e5c msp: 0x20001000
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
Info : Unable to match requested speed 8000 kHz, using 4000 kHz

[INFO   ] Installing app bundle. Size: 4096 bytes.
[DEBUG  ] Using program command: "program {{binary}} verify {address:#x};"
[DEBUG  ] Expanded program command: "program {binary} verify 0x8080000;"
[DEBUG  ] Running "openocd -c "interface hla;                                                    hla_layout stlink;                                                    hla_device_desc "ST-LINK/V2-1";                                                    hla_vid_pid 0x0483 0x374b;                                                    set WORKAREASIZE 0x40000;                                                    source [find target/stm32f4x.cfg];  init; reset init; halt; program /var/folders/rc/r22l0fy5611279w4ljf1vbn40000gn/T/tmpqrgnlguk.bin verify 0x8080000;  exit"".
[INFO   ] Open On-Chip Debugger 0.10.0+dev-01408-g762ddcb74-dirty (2020-09-24-11:11)
Licensed under GNU GPL v2
For bug reports, read
	http://openocd.org/doc/doxygen/bugs.html
DEPRECATED! use 'adapter driver' not 'interface'
Info : auto-selecting first available session transport "hla_swd". To override use 'transport select <transport>'.
Info : The selected transport took over low-level target control. The results might differ compared to plain JTAG/SWD
Info : clock speed 2000 kHz
Info : STLINK V2J37M26 (API v2) VID:PID 0483:374B
Info : Target voltage: 2.901302
Info : stm32f4x.cpu: hardware has 6 breakpoints, 4 watchpoints
Info : starting gdb server for stm32f4x.cpu on 3333
Info : Listening on port 3333 for gdb connections
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
target halted due to debug-request, current mode: Thread
xPSR: 0x01000000 pc: 0x08007e5c msp: 0x20001000
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
target halted due to debug-request, current mode: Thread
xPSR: 0x01000000 pc: 0x08007e5c msp: 0x20001000
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
** Programming Started **
Info : device id = 0x10076413
Info : flash size = 1024 kbytes
** Programming Finished **
** Verify Started **
** Verified OK **

[DEBUG  ] Erasing page at address 0x8081000
[DEBUG  ] Using erase command: "flash fillb 0x8081000 0xff 512;"
[DEBUG  ] Expanded erase command: "flash fillb 0x8081000 0xff 512;"
[DEBUG  ] Running "openocd -c "interface hla;                                                    hla_layout stlink;                                                    hla_device_desc "ST-LINK/V2-1";                                                    hla_vid_pid 0x0483 0x374b;                                                    set WORKAREASIZE 0x40000;                                                    source [find target/stm32f4x.cfg];  init; reset init; halt; flash fillb 0x8081000 0xff 512;  exit"".
[INFO   ] Open On-Chip Debugger 0.10.0+dev-01408-g762ddcb74-dirty (2020-09-24-11:11)
Licensed under GNU GPL v2
For bug reports, read
	http://openocd.org/doc/doxygen/bugs.html
DEPRECATED! use 'adapter driver' not 'interface'
Info : auto-selecting first available session transport "hla_swd". To override use 'transport select <transport>'.
Info : The selected transport took over low-level target control. The results might differ compared to plain JTAG/SWD
Info : clock speed 2000 kHz
Info : STLINK V2J37M26 (API v2) VID:PID 0483:374B
Info : Target voltage: 2.901302
Info : stm32f4x.cpu: hardware has 6 breakpoints, 4 watchpoints
Info : starting gdb server for stm32f4x.cpu on 3333
Info : Listening on port 3333 for gdb connections
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
target halted due to debug-request, current mode: Thread
xPSR: 0x01000000 pc: 0x08007e5c msp: 0x20001000
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
Info : device id = 0x10076413
Info : flash size = 1024 kbytes

[INFO   ] Finished in 4.845 seconds
```

## (3.1) blink, buttons, c_helloがインストールされていることを確認

```
$ tockloader read --board stm32f407g-disc1 --openocd 0x8080000 0x1000
[INFO   ] Hardcoding command line argument "bundle_apps" to "True" for board stm32f407g-disc1.
[INFO   ] Hardcoding command line argument "openocd" to "True" for board stm32f407g-disc1.
[INFO   ] Using settings from KNOWN_BOARDS["stm32f407g-disc1"]
[STATUS ] Reading flash from the board...
[STATUS ]   Address: 0x8080000
[STATUS ]   Length:  4096 bytes
08080000  02 00 2c 00 00 08 00 00  01 00 00 00 d5 75 4c 6e  |..,..........uLn|
08080010  01 00 0c 00 29 00 00 00  00 00 00 00 f4 11 00 00  |....)...........|
08080020  03 00 05 00 62 6c 69 6e  6b 00 00 00 04 04 00 00  |....blink.......|
08080030  00 08 00 00 24 00 00 00  28 04 00 00 24 08 00 00  |....$...(...$...|
（略）
080807e0  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
080807f0  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080800  02 00 2c 00 00 04 00 00  01 00 00 00 cd 0e 20 74  |..,........... t|
08080810  01 00 0c 00 29 00 00 00  00 00 00 00 e8 11 00 00  |....)...........|
08080820  03 00 07 00 62 75 74 74  6f 6e 73 00 34 02 00 00  |....buttons.4...|
08080830  00 08 00 00 1c 00 00 00  50 02 00 00 1c 08 00 00  |........P.......|
（略）
08080be0  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080bf0  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
08080c00  02 00 2c 00 00 04 00 00  01 00 00 00 db 26 20 65  |..,..........& e|
08080c10  01 00 0c 00 29 00 00 00  00 00 00 00 fc 11 00 00  |....)...........|
08080c20  03 00 07 00 63 5f 68 65  6c 6c 6f 00 34 02 00 00  |....c_hello.4...|
08080c30  00 08 00 00 20 00 00 00  54 02 00 00 20 08 00 00  |.... ...T... ...|
（略）
[INFO   ] Finished in 0.271 seconds
```

(4) `info`で確認

```
$ tockloader info --board stm32f407g-disc1 --openocd
[INFO   ] Hardcoding command line argument "bundle_apps" to "True" for board stm32f407g-disc1.
[INFO   ] Hardcoding command line argument "openocd" to "True" for board stm32f407g-disc1.
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


┌──────────────────────────────────────────────────┐
│ App 1                                            |
└──────────────────────────────────────────────────┘
  Name:                  buttons
  Enabled:               True
  Sticky:                False
  Total Size in Flash:   1024 bytes
  Address in Flash:      0x8080800
    version               : 2
    header_size           :         44         0x2c
    total_size            :       1024        0x400
    checksum              :              0x74200ecd
    flags                 :          1          0x1
      enabled             : Yes
      sticky              : No
    TLV: Main (1)
      init_fn_offset      :         41         0x29
      protected_size      :          0          0x0
      minimum_ram_size    :       4584       0x11e8
    TLV: Package Name (3)
      package_name        : buttons


┌──────────────────────────────────────────────────┐
│ App 2                                            |
└──────────────────────────────────────────────────┘
  Name:                  c_hello
  Enabled:               True
  Sticky:                False
  Total Size in Flash:   1024 bytes
  Address in Flash:      0x8080c00
    version               : 2
    header_size           :         44         0x2c
    total_size            :       1024        0x400
    checksum              :              0x652026db
    flags                 :          1          0x1
      enabled             : Yes
      sticky              : No
    TLV: Main (1)
      init_fn_offset      :         41         0x29
      protected_size      :          0          0x0
      minimum_ram_size    :       4604       0x11fc
    TLV: Package Name (3)
      package_name        : c_hello


No bootloader.
[INFO   ] Finished in 1.183 seconds
```

(5) buttonsをアンインストール

```
$ tockloader uninstall --board stm32f407g-disc1 --openocd --debug buttons
[INFO   ] Hardcoding command line argument "bundle_apps" to "True" for board stm32f407g-disc1.
[INFO   ] Hardcoding command line argument "openocd" to "True" for board stm32f407g-disc1.
[INFO   ] Using settings from KNOWN_BOARDS["stm32f407g-disc1"]
[STATUS ] Removing app(s) buttons from board...
[DEBUG  ] Using read command: "dump_image {{binary}} {address:#x} {length};"
[DEBUG  ] Expanded read command: "dump_image {binary} 0x8080000 200;"
[DEBUG  ] Running "openocd -c "interface hla;                                                    hla_layout stlink;                                                    hla_device_desc "ST-LINK/V2-1";                                                    hla_vid_pid 0x0483 0x374b;                                                    set WORKAREASIZE 0x40000;                                                    source [find target/stm32f4x.cfg];  init; reset init; halt; dump_image /var/folders/rc/r22l0fy5611279w4ljf1vbn40000gn/T/tmpz_wns_r3.bin 0x8080000 200;  exit"".
[INFO   ] Open On-Chip Debugger 0.10.0+dev-01408-g762ddcb74-dirty (2020-09-24-11:11)
Licensed under GNU GPL v2
For bug reports, read
	http://openocd.org/doc/doxygen/bugs.html
DEPRECATED! use 'adapter driver' not 'interface'
Info : auto-selecting first available session transport "hla_swd". To override use 'transport select <transport>'.
Info : The selected transport took over low-level target control. The results might differ compared to plain JTAG/SWD
Info : clock speed 2000 kHz
Info : STLINK V2J37M26 (API v2) VID:PID 0483:374B
Info : Target voltage: 2.901302
Info : stm32f4x.cpu: hardware has 6 breakpoints, 4 watchpoints
Info : starting gdb server for stm32f4x.cpu on 3333
Info : Listening on port 3333 for gdb connections
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
target halted due to debug-request, current mode: Thread
xPSR: 0x01000000 pc: 0x08007e5c msp: 0x20001000
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
Info : Unable to match requested speed 8000 kHz, using 4000 kHz

[DEBUG  ] Using read command: "dump_image {{binary}} {address:#x} {length};"
[DEBUG  ] Expanded read command: "dump_image {binary} 0x8080800 200;"
[DEBUG  ] Running "openocd -c "interface hla;                                                    hla_layout stlink;                                                    hla_device_desc "ST-LINK/V2-1";                                                    hla_vid_pid 0x0483 0x374b;                                                    set WORKAREASIZE 0x40000;                                                    source [find target/stm32f4x.cfg];  init; reset init; halt; dump_image /var/folders/rc/r22l0fy5611279w4ljf1vbn40000gn/T/tmp1vwo0v7s.bin 0x8080800 200;  exit"".
[INFO   ] Open On-Chip Debugger 0.10.0+dev-01408-g762ddcb74-dirty (2020-09-24-11:11)
Licensed under GNU GPL v2
For bug reports, read
	http://openocd.org/doc/doxygen/bugs.html
DEPRECATED! use 'adapter driver' not 'interface'
Info : auto-selecting first available session transport "hla_swd". To override use 'transport select <transport>'.
Info : The selected transport took over low-level target control. The results might differ compared to plain JTAG/SWD
Info : clock speed 2000 kHz
Info : STLINK V2J37M26 (API v2) VID:PID 0483:374B
Info : Target voltage: 2.901302
Info : stm32f4x.cpu: hardware has 6 breakpoints, 4 watchpoints
Info : starting gdb server for stm32f4x.cpu on 3333
Info : Listening on port 3333 for gdb connections
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
target halted due to debug-request, current mode: Thread
xPSR: 0x01000000 pc: 0x08007e5c msp: 0x20001000
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
Info : Unable to match requested speed 8000 kHz, using 4000 kHz

[DEBUG  ] Using read command: "dump_image {{binary}} {address:#x} {length};"
[DEBUG  ] Expanded read command: "dump_image {binary} 0x8080c00 200;"
[DEBUG  ] Running "openocd -c "interface hla;                                                    hla_layout stlink;                                                    hla_device_desc "ST-LINK/V2-1";                                                    hla_vid_pid 0x0483 0x374b;                                                    set WORKAREASIZE 0x40000;                                                    source [find target/stm32f4x.cfg];  init; reset init; halt; dump_image /var/folders/rc/r22l0fy5611279w4ljf1vbn40000gn/T/tmphdkrs590.bin 0x8080c00 200;  exit"".
[INFO   ] Open On-Chip Debugger 0.10.0+dev-01408-g762ddcb74-dirty (2020-09-24-11:11)
Licensed under GNU GPL v2
For bug reports, read
	http://openocd.org/doc/doxygen/bugs.html
DEPRECATED! use 'adapter driver' not 'interface'
Info : auto-selecting first available session transport "hla_swd". To override use 'transport select <transport>'.
Info : The selected transport took over low-level target control. The results might differ compared to plain JTAG/SWD
Info : clock speed 2000 kHz
Info : STLINK V2J37M26 (API v2) VID:PID 0483:374B
Info : Target voltage: 2.899882
Info : stm32f4x.cpu: hardware has 6 breakpoints, 4 watchpoints
Info : starting gdb server for stm32f4x.cpu on 3333
Info : Listening on port 3333 for gdb connections
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
target halted due to debug-request, current mode: Thread
xPSR: 0x01000000 pc: 0x08007e5c msp: 0x20001000
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
Info : Unable to match requested speed 8000 kHz, using 4000 kHz

[DEBUG  ] Using read command: "dump_image {{binary}} {address:#x} {length};"
[DEBUG  ] Expanded read command: "dump_image {binary} 0x8081000 200;"
[DEBUG  ] Running "openocd -c "interface hla;                                                    hla_layout stlink;                                                    hla_device_desc "ST-LINK/V2-1";                                                    hla_vid_pid 0x0483 0x374b;                                                    set WORKAREASIZE 0x40000;                                                    source [find target/stm32f4x.cfg];  init; reset init; halt; dump_image /var/folders/rc/r22l0fy5611279w4ljf1vbn40000gn/T/tmprgl3kki7.bin 0x8081000 200;  exit"".
[INFO   ] Open On-Chip Debugger 0.10.0+dev-01408-g762ddcb74-dirty (2020-09-24-11:11)
Licensed under GNU GPL v2
For bug reports, read
	http://openocd.org/doc/doxygen/bugs.html
DEPRECATED! use 'adapter driver' not 'interface'
Info : auto-selecting first available session transport "hla_swd". To override use 'transport select <transport>'.
Info : The selected transport took over low-level target control. The results might differ compared to plain JTAG/SWD
Info : clock speed 2000 kHz
Info : STLINK V2J37M26 (API v2) VID:PID 0483:374B
Info : Target voltage: 2.901302
Info : stm32f4x.cpu: hardware has 6 breakpoints, 4 watchpoints
Info : starting gdb server for stm32f4x.cpu on 3333
Info : Listening on port 3333 for gdb connections
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
target halted due to debug-request, current mode: Thread
xPSR: 0x01000000 pc: 0x08007e5c msp: 0x20001000
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
Info : Unable to match requested speed 8000 kHz, using 4000 kHz

[DEBUG  ] Found 3 apps on the board.
[DEBUG  ]   1. blink
[DEBUG  ]   2. buttons
[DEBUG  ]   3. c_hello
[STATUS ] Attempting to uninstall:
[STATUS ]   - buttons
[INFO   ] Reading app blink binary from board.
[DEBUG  ] Using read command: "dump_image {{binary}} {address:#x} {length};"
[DEBUG  ] Expanded read command: "dump_image {binary} 0x8080000 2048;"
[DEBUG  ] Running "openocd -c "interface hla;                                                    hla_layout stlink;                                                    hla_device_desc "ST-LINK/V2-1";                                                    hla_vid_pid 0x0483 0x374b;                                                    set WORKAREASIZE 0x40000;                                                    source [find target/stm32f4x.cfg];  init; reset init; halt; dump_image /var/folders/rc/r22l0fy5611279w4ljf1vbn40000gn/T/tmpw5ee8xif.bin 0x8080000 2048;  exit"".
[INFO   ] Open On-Chip Debugger 0.10.0+dev-01408-g762ddcb74-dirty (2020-09-24-11:11)
Licensed under GNU GPL v2
For bug reports, read
	http://openocd.org/doc/doxygen/bugs.html
DEPRECATED! use 'adapter driver' not 'interface'
Info : auto-selecting first available session transport "hla_swd". To override use 'transport select <transport>'.
Info : The selected transport took over low-level target control. The results might differ compared to plain JTAG/SWD
Info : clock speed 2000 kHz
Info : STLINK V2J37M26 (API v2) VID:PID 0483:374B
Info : Target voltage: 2.901302
Info : stm32f4x.cpu: hardware has 6 breakpoints, 4 watchpoints
Info : starting gdb server for stm32f4x.cpu on 3333
Info : Listening on port 3333 for gdb connections
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
target halted due to debug-request, current mode: Thread
xPSR: 0x01000000 pc: 0x08007e5c msp: 0x20001000
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
Info : Unable to match requested speed 8000 kHz, using 4000 kHz

[INFO   ] Reading app c_hello binary from board.
[DEBUG  ] Using read command: "dump_image {{binary}} {address:#x} {length};"
[DEBUG  ] Expanded read command: "dump_image {binary} 0x8080c00 1024;"
[DEBUG  ] Running "openocd -c "interface hla;                                                    hla_layout stlink;                                                    hla_device_desc "ST-LINK/V2-1";                                                    hla_vid_pid 0x0483 0x374b;                                                    set WORKAREASIZE 0x40000;                                                    source [find target/stm32f4x.cfg];  init; reset init; halt; dump_image /var/folders/rc/r22l0fy5611279w4ljf1vbn40000gn/T/tmpju2j_zl0.bin 0x8080c00 1024;  exit"".
[INFO   ] Open On-Chip Debugger 0.10.0+dev-01408-g762ddcb74-dirty (2020-09-24-11:11)
Licensed under GNU GPL v2
For bug reports, read
	http://openocd.org/doc/doxygen/bugs.html
DEPRECATED! use 'adapter driver' not 'interface'
Info : auto-selecting first available session transport "hla_swd". To override use 'transport select <transport>'.
Info : The selected transport took over low-level target control. The results might differ compared to plain JTAG/SWD
Info : clock speed 2000 kHz
Info : STLINK V2J37M26 (API v2) VID:PID 0483:374B
Info : Target voltage: 2.901302
Info : stm32f4x.cpu: hardware has 6 breakpoints, 4 watchpoints
Info : starting gdb server for stm32f4x.cpu on 3333
Info : Listening on port 3333 for gdb connections
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
target halted due to debug-request, current mode: Thread
xPSR: 0x01000000 pc: 0x08007e5c msp: 0x20001000
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
Info : Unable to match requested speed 8000 kHz, using 4000 kHz

[INFO   ] Installing app bundle. Size: 3072 bytes.
[DEBUG  ] Using program command: "program {{binary}} verify {address:#x};"
[DEBUG  ] Expanded program command: "program {binary} verify 0x8080000;"
[DEBUG  ] Running "openocd -c "interface hla;                                                    hla_layout stlink;                                                    hla_device_desc "ST-LINK/V2-1";                                                    hla_vid_pid 0x0483 0x374b;                                                    set WORKAREASIZE 0x40000;                                                    source [find target/stm32f4x.cfg];  init; reset init; halt; program /var/folders/rc/r22l0fy5611279w4ljf1vbn40000gn/T/tmpxg1qvjed.bin verify 0x8080000;  exit"".
[INFO   ] Open On-Chip Debugger 0.10.0+dev-01408-g762ddcb74-dirty (2020-09-24-11:11)
Licensed under GNU GPL v2
For bug reports, read
	http://openocd.org/doc/doxygen/bugs.html
DEPRECATED! use 'adapter driver' not 'interface'
Info : auto-selecting first available session transport "hla_swd". To override use 'transport select <transport>'.
Info : The selected transport took over low-level target control. The results might differ compared to plain JTAG/SWD
Info : clock speed 2000 kHz
Info : STLINK V2J37M26 (API v2) VID:PID 0483:374B
Info : Target voltage: 2.901302
Info : stm32f4x.cpu: hardware has 6 breakpoints, 4 watchpoints
Info : starting gdb server for stm32f4x.cpu on 3333
Info : Listening on port 3333 for gdb connections
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
target halted due to debug-request, current mode: Thread
xPSR: 0x01000000 pc: 0x08007e5c msp: 0x20001000
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
target halted due to debug-request, current mode: Thread
xPSR: 0x01000000 pc: 0x08007e5c msp: 0x20001000
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
** Programming Started **
Info : device id = 0x10076413
Info : flash size = 1024 kbytes
** Programming Finished **
** Verify Started **
** Verified OK **

[DEBUG  ] Erasing page at address 0x8080c00
[DEBUG  ] Using erase command: "flash fillb 0x8080c00 0xff 512;"
[DEBUG  ] Expanded erase command: "flash fillb 0x8080c00 0xff 512;"
[DEBUG  ] Running "openocd -c "interface hla;                                                    hla_layout stlink;                                                    hla_device_desc "ST-LINK/V2-1";                                                    hla_vid_pid 0x0483 0x374b;                                                    set WORKAREASIZE 0x40000;                                                    source [find target/stm32f4x.cfg];  init; reset init; halt; flash fillb 0x8080c00 0xff 512;  exit"".
[INFO   ] Open On-Chip Debugger 0.10.0+dev-01408-g762ddcb74-dirty (2020-09-24-11:11)
Licensed under GNU GPL v2
For bug reports, read
	http://openocd.org/doc/doxygen/bugs.html
DEPRECATED! use 'adapter driver' not 'interface'
Info : auto-selecting first available session transport "hla_swd". To override use 'transport select <transport>'.
Info : The selected transport took over low-level target control. The results might differ compared to plain JTAG/SWD
Info : clock speed 2000 kHz
Info : STLINK V2J37M26 (API v2) VID:PID 0483:374B
Info : Target voltage: 2.901302
Info : stm32f4x.cpu: hardware has 6 breakpoints, 4 watchpoints
Info : starting gdb server for stm32f4x.cpu on 3333
Info : Listening on port 3333 for gdb connections
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
target halted due to debug-request, current mode: Thread
xPSR: 0x01000000 pc: 0x08007e5c msp: 0x20001000
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
Info : device id = 0x10076413
Info : flash size = 1024 kbytes

[STATUS ] Uninstall complete.
[DEBUG  ] Using read command: "dump_image {{binary}} {address:#x} {length};"
[DEBUG  ] Expanded read command: "dump_image {binary} 0x8080000 200;"
[DEBUG  ] Running "openocd -c "interface hla;                                                    hla_layout stlink;                                                    hla_device_desc "ST-LINK/V2-1";                                                    hla_vid_pid 0x0483 0x374b;                                                    set WORKAREASIZE 0x40000;                                                    source [find target/stm32f4x.cfg];  init; reset init; halt; dump_image /var/folders/rc/r22l0fy5611279w4ljf1vbn40000gn/T/tmp9ykvuiwo.bin 0x8080000 200;  exit"".
[INFO   ] Open On-Chip Debugger 0.10.0+dev-01408-g762ddcb74-dirty (2020-09-24-11:11)
Licensed under GNU GPL v2
For bug reports, read
	http://openocd.org/doc/doxygen/bugs.html
DEPRECATED! use 'adapter driver' not 'interface'
Info : auto-selecting first available session transport "hla_swd". To override use 'transport select <transport>'.
Info : The selected transport took over low-level target control. The results might differ compared to plain JTAG/SWD
Info : clock speed 2000 kHz
Info : STLINK V2J37M26 (API v2) VID:PID 0483:374B
Info : Target voltage: 2.901302
Info : stm32f4x.cpu: hardware has 6 breakpoints, 4 watchpoints
Info : starting gdb server for stm32f4x.cpu on 3333
Info : Listening on port 3333 for gdb connections
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
target halted due to debug-request, current mode: Thread
xPSR: 0x01000000 pc: 0x08007e5c msp: 0x20001000
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
Info : Unable to match requested speed 8000 kHz, using 4000 kHz

[DEBUG  ] Using read command: "dump_image {{binary}} {address:#x} {length};"
[DEBUG  ] Expanded read command: "dump_image {binary} 0x8080800 200;"
[DEBUG  ] Running "openocd -c "interface hla;                                                    hla_layout stlink;                                                    hla_device_desc "ST-LINK/V2-1";                                                    hla_vid_pid 0x0483 0x374b;                                                    set WORKAREASIZE 0x40000;                                                    source [find target/stm32f4x.cfg];  init; reset init; halt; dump_image /var/folders/rc/r22l0fy5611279w4ljf1vbn40000gn/T/tmpnbxbopzc.bin 0x8080800 200;  exit"".
[INFO   ] Open On-Chip Debugger 0.10.0+dev-01408-g762ddcb74-dirty (2020-09-24-11:11)
Licensed under GNU GPL v2
For bug reports, read
	http://openocd.org/doc/doxygen/bugs.html
DEPRECATED! use 'adapter driver' not 'interface'
Info : auto-selecting first available session transport "hla_swd". To override use 'transport select <transport>'.
Info : The selected transport took over low-level target control. The results might differ compared to plain JTAG/SWD
Info : clock speed 2000 kHz
Info : STLINK V2J37M26 (API v2) VID:PID 0483:374B
Info : Target voltage: 2.901302
Info : stm32f4x.cpu: hardware has 6 breakpoints, 4 watchpoints
Info : starting gdb server for stm32f4x.cpu on 3333
Info : Listening on port 3333 for gdb connections
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
target halted due to debug-request, current mode: Thread
xPSR: 0x01000000 pc: 0x08007e5c msp: 0x20001000
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
Info : Unable to match requested speed 8000 kHz, using 4000 kHz

[DEBUG  ] Using read command: "dump_image {{binary}} {address:#x} {length};"
[DEBUG  ] Expanded read command: "dump_image {binary} 0x8080c00 200;"
[DEBUG  ] Running "openocd -c "interface hla;                                                    hla_layout stlink;                                                    hla_device_desc "ST-LINK/V2-1";                                                    hla_vid_pid 0x0483 0x374b;                                                    set WORKAREASIZE 0x40000;                                                    source [find target/stm32f4x.cfg];  init; reset init; halt; dump_image /var/folders/rc/r22l0fy5611279w4ljf1vbn40000gn/T/tmpqje5_khm.bin 0x8080c00 200;  exit"".
[INFO   ] Open On-Chip Debugger 0.10.0+dev-01408-g762ddcb74-dirty (2020-09-24-11:11)
Licensed under GNU GPL v2
For bug reports, read
	http://openocd.org/doc/doxygen/bugs.html
DEPRECATED! use 'adapter driver' not 'interface'
Info : auto-selecting first available session transport "hla_swd". To override use 'transport select <transport>'.
Info : The selected transport took over low-level target control. The results might differ compared to plain JTAG/SWD
Info : clock speed 2000 kHz
Info : STLINK V2J37M26 (API v2) VID:PID 0483:374B
Info : Target voltage: 2.901302
Info : stm32f4x.cpu: hardware has 6 breakpoints, 4 watchpoints
Info : starting gdb server for stm32f4x.cpu on 3333
Info : Listening on port 3333 for gdb connections
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
target halted due to debug-request, current mode: Thread
xPSR: 0x01000000 pc: 0x08007e5c msp: 0x20001000
Info : Unable to match requested speed 8000 kHz, using 4000 kHz
Info : Unable to match requested speed 8000 kHz, using 4000 kHz

[DEBUG  ] Found 2 apps on the board.
[DEBUG  ]   1. blink
[DEBUG  ]   2. c_hello
Traceback (most recent call last):
  File "/Users/dspace/Library/Python/3.8/bin/tockloader", line 8, in <module>
    sys.exit(main())
  File "/Users/dspace/Library/Python/3.8/lib/python/site-packages/tockloader/main.py", line 653, in main
    args.func(args)
  File "/Users/dspace/Library/Python/3.8/lib/python/site-packages/tockloader/main.py", line 156, in command_uninstall
    tock_loader.uninstall_app(args.name)
  File "/Users/dspace/Library/Python/3.8/lib/python/site-packages/tockloader/tockloader.py", line 293, in uninstall_app
    logging.info('After uninstall, remaining apps on board: ', end='')
  File "/usr/local/Cellar/python@3.8/3.8.5/Frameworks/Python.framework/Versions/3.8/lib/python3.8/logging/__init__.py", line 2070, in info
    root.info(msg, *args, **kwargs)
  File "/usr/local/Cellar/python@3.8/3.8.5/Frameworks/Python.framework/Versions/3.8/lib/python3.8/logging/__init__.py", line 1434, in info
    self._log(INFO, msg, args, **kwargs)
TypeError: _log() got an unexpected keyword argument 'end'
```

## (6) `info`でbuttonsがアンインストールされていることを確認

```
$ tockloader info --board stm32f407g-disc1 --openocd
[INFO   ] Hardcoding command line argument "bundle_apps" to "True" for board stm32f407g-disc1.
[INFO   ] Hardcoding command line argument "openocd" to "True" for board stm32f407g-disc1.
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


┌──────────────────────────────────────────────────┐
│ App 1                                            |
└──────────────────────────────────────────────────┘
  Name:                  c_hello
  Enabled:               True
  Sticky:                False
  Total Size in Flash:   1024 bytes
  Address in Flash:      0x8080800
    version               : 2
    header_size           :         44         0x2c
    total_size            :       1024        0x400
    checksum              :              0x652026db
    flags                 :          1          0x1
      enabled             : Yes
      sticky              : No
    TLV: Main (1)
      init_fn_offset      :         41         0x29
      protected_size      :          0          0x0
      minimum_ram_size    :       4604       0x11fc
    TLV: Package Name (3)
      package_name        : c_hello


No bootloader.
[INFO   ] Finished in 0.964 seconds
```