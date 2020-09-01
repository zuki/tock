System Calls
========================================

**TRD:** <br/>
**Working Group:** Kernel<br/>
**Type:** Documentary<br/>
**Status:** Draft <br/>
**Author:** Philip Levis, Amit Levy, Leon Schuermann <br/>
**Draft-Created:** August 31, 2020<br/>
**Draft-Modified:** Sep 1, 2020<br/>
**Draft-Version:** 1<br/>
**Draft-Discuss:** tock-dev@googlegroups.com</br>

Abstract
-------------------------------

This document describes the system call application binary interface (ABI)
between user space applications and the Tock kernel.

1 Introduction
===============================

The Tock operating system can run multiple independent userspace applications.
Because these applications are untrusted,  the kernel uses hardware memory 
protection to isolate them from it. This allows applications written in C
(or even assembly) to safely run on Tock.

Each application image is a separate process: it has its own address space
and thread stack. Applications invoke operations on and receive callbacks
from the Tock kernel through the system call programming interface.

This document describes Tock's system call programming interface (API)
and application binary interface (ABI). It describes the system calls
that Tock implements, their semantics, and how a userspace process
invokes them.

2 Design Considerations
===============================

Three design considerations guide the design of Tock's system call API and
ABI.

  1. Tock is currently supported on ARM CortexM and RISCV and may support
  others in the future. Its ABI must support both architectures and be
  flexible enough to support future ones.
  2. Tock userspace applications can be written in any language. The system
  call API must support their calling semantics in a safe way. Rust is
  especially important.
  3. Both the API and ABI must be efficient and support common call
  patterns in an efficient way.

2.1 Architectural Support and ABIs
--------------------------------

The primary question for the ABI is how many and which registers transfer
data between the kernel and userspace. Passing more registers has the benefit
of the kernel and userspace being able to transfer more information
without relying on pointers to memory structures. It has the cost of requiring
every system call to transfer and manipulate more registers.

2.2 Programming Language APIs
---------------------------------

Userspace support for Rust is an important requirement for Tock. A key
invariant in Rust is that a given memory object can either have multiple
references or a single mutable reference. If userspace
passes a writeable (mutable) buffer into the kernel, it must relinquish
any references to that buffer. As a result, the only way for userspace
to regain a reference to the buffer is for the kernel to pass it back.

2.3 Efficiency
---------------------------------

Programming language calling conventions are
another consideration because they affect efficiency. For
example the C calling convention in ARM says that the first four arguments
to a function are stored in r0-r3. Additional arguments are stored on
the stack. Therefore, if the system call ABI says that arguments are stored
in different registers than r0-r3, a C function call that invokes a system
call will need to move the C arguments into those registers.


3. System Call ABI
=================================

This section describes the ABI for Tock on 32-bit platforms. The ABI for
64-bit platforms is currently undefined but may be specified in a future TRD.

When userspace invokes a system call, it passes 4 registers to the kernel
as arguments. It also pass an 8-bit value of which type of system call (see
Section 4) is being invoked (the Syscall Class ID). When the system call returns, it returns 4 registers
as return values. When the kernel invokes a callback on userspace, it passes
4 registers to userspace as arguments and has no return value. 

|                        | CortexM | RISC-V |
|------------------------|---------|--------|
| Syscall Arguments      | r0-r3   | a1-a4  |
| Syscall Return Values  | r0-r3   | a1-a4  |
| Syscall Class ID       | svc     | a0     |
| Callback Arguments     | r0-r3   | a0-a3  |
| Callback Return Values | None    | None   |

All system calls have the same return value format. A system call can return
one of nine values, which are shown here. r0-r3 refer to the return value
registers: for CortexM they are r0-r3 and for RISC-V they are a1-a4.

|                          | r0 | r1                 | r2                 | r3             |
|--------------------------|----|--------------------|--------------------|----------------|
| Failure                  | 0  | Error code         | -                  | -              |
| Failure with 1 value     | 1  | Error code         | Return Value 0     |                |
| Failure with 2 values    | 2  | Error code         | Return Value 0     | Return Value 1 |
| Success                  | 32 |                    |                    |                |
| Success with u32         | 33 | Return Value 0     |                    |                |
| Success with 2 u32       | 34 | Return Value 0     | Return Value 1     |                |
| Success with 3 u32       | 35 | Return Value 0     | Return Value 1     | Return Value 2 |
| Success with u64         | 36 | Return Value 0 LSB | Return Value 0 MSB |                |
| Success with u64 and u32 | 37 | Return Value 0 LSB | Return Value 0 MSB | Return Value 1 |

There are a wide variety of failure and success values because different
system calls need to pass different amounts of data. A command that requests
a 64-bit timestamp, for example, needs its success to return a `u64`, but its
failure can return nothing. In contrast, a system call that passes a pointer
into the kernel may have a simple success return value but requires a failure with
1 value so the pointer can be passed back.

Every system call MUST return only one failure and only one success type. Different
system calls may use different failure and success types, but any specific system
call returns exactly one of each. If an operation might have different success return
types or failure return types, then it should be split into multiple system calls.

This requirement of a single failure type and a single success type is to simplify
userspace implementations and preclude them from having to handle many different cases.
The presence of many difference cases suggests that the operation should be split up --
there is non-determinism in its execution or its meaning is overloaded. It also fits
well with Rust's `Result` type.

4. System Call API
=================================

Tock has 6 classes or types of system calls. When a system call is invoked, the
class is encoded as the Syscall Class ID. Some system call classes are implemented
by the core kernel and so always have the same operations. Others are implemented
by peripheral syscall drivers and so the set of valid operations depends on what peripherals
the platform has and which have drivers installed in the kernel.

The 6 classes are:

| Syscall Class   | Syscall ID |
|-----------------|------------|
| Yield           |     0      |
| Subscribe       |     1      |
| Command         |     2      |
| Allow           |     3      |
| Read-Only Allow |     4      | 
| Memory          |     5      |

All of the system call classes except Yield are non-blocking. When a userspace
process calls a Subscribe, Command, Allow, Read-Only Allow, or Memory syscall,
the kernel will not put it on a wait queue. Instead, it will return immediately
upon completion. The kernel scheduler may decide to suspect the process due to
a timeslice expiration or the kernel thread being runnable, but the system calls
themselves do not block. If an operation is long-running (e.g., I/O), its completion
is signaled by a callback (see the Subscribe call in 4.2).

Peripheral driver-specific system calls (Subscribe, Command, Allow, Read-Only Allow)
all include two arguments, a driver identifier and a syscall identifier. The driver identifier
specifies which peripheral system call driver to invoke. The syscall identifier (which
is different than the Syscall ID in the table above)
specifies which instance of that system call on that driver to invoke. For example, the
Console driver has driver identifier `0x1` and a Command to the console driver with
syscall identifier `0x2` starts receiving console data into a buffer.

4.1 Yield (Class ID: 0)
--------------------------------

The Yield system call class implements the only blocking system call in Tock.
Yield is how a userspace process can relinquish the processor to other processes,
or wait for one of its long-running calls to complete.

When a process calls a Yield system call, if there are any pending callbacks from
the kernel, the kernel schedules one of them to execute on the userspace stack.
If there are multiple pending callbacks, each one requires a separate Yield call
to invoke. The kernel invokes callbacks only in response to Yield system calls.
This form of very limited preemption allows userspace to manage concurrent access
to its variables.

There are two Yield system calls:
  - `yield-wait`
  - `yield-no-wait`

The first call, `yield-wait`, blocks until a callback executes. This is the
only blocking system call in Tock. It is commonly used to provide a blocking
I/O interface to userspace. A userspace library starts a long-running operation
that has a callback, then calls `yield-wait` to wait for a callback. When the
`yield-wait` returns, the process checks if the resuming callback was the one
it was expecting, and if not calls `yield-wait` again. The `yield-wait` system
call always returns `Success`.

The second call, 'yield-no-wait', executes a single callback if any is pending.
If no callbacks are pending it returns immediately. The `yield-no-wait` system
call returns `Success` if a callback executed and `Failure` if no callback
executed.

4.2 Subscribe (Class ID: 1)
--------------------------------

The Subscribe system call class is how a userspace process registers callbacks
with the kernel. Subscribe system calls are implemented by peripheral syscall
drivers, so the set of valid Subscribe calls depends on the platform and what
drivers were compiled into the kernel.

The register arguments for Subscribe system calls are as follows. The registers
r0-r3 correspond to r0-r3 on CortexM and a1-a4 on RISC-V.

| Argument               | Register |
|------------------------|----------|
| Driver identifer       | r0       |
| Syscall identifier     | r1       |
| Callback pointer       | r2       |
| Application data       | r3       |

The `callback pointer` is the address of the first instruction of
the callback function. The `application data` argument is a parameter
that an application passes in and the kernel passes back in callbacks
unmodified. 

4.3 Command (Class ID: 2)
---------------------------------

The Command system call class is how a userspace process calls a function
in the kernel, either to return an immediate result or start a long-running
operation. Command system calls are implemented by peripheral syscall drivers,
so the set of valid Command calls depends on the platform and what drivers were
compiled into the kernel.

The register arguments for Command system calls are as follows. The registers
r0-r3 correspond to r0-r3 on CortexM and a1-a4 on RISC-V.

| Argument               | Register |
|------------------------|----------|
| Driver identifer       | r0       |
| Command identifier     | r1       |
| Argument 0             | r2       |
| Argument 1             | r3       |

Argument 0 and argument 1 are unsigned 32-bit integers. Command calls should
never pass pointers: those are passed with Allow calls, as they can adjust
memory protection to allow the kernel to access them.

The return types of Command are instance-specific. Each specific Command
instance (combination of major and minor identifier) specifies its failure
type and success type.

4.4 Allow (Class ID: 3)
---------------------------------

The Allow system call class is how a userspace process shares a read-write buffer
with the kernel. When userspace shares a buffer, it can no longer access
it. Calling an Allow system call also returns a buffer (address and length).
On the first call to an
Allow system call, the kernel returns a zero-length buffer. Subsequent calls
to Allow return the previous buffer passed. Therefore, to regain access to the
buffer, the process must call the same Allow system call again. 

The register arguments for Allow system calls are as follows. The registers
r0-r3 correspond to r0-r3 on CortexM and a1-a4 on RISC-V.

| Argument               | Register |
|------------------------|----------|
| Driver identifer       | r0       |
| Buffer identifier      | r1       |
| Address                | r2       |
| Size                   | r3       |

The return values for Allow system calls are `Failure with 2 u32` and `Success with 2 u32`.
In both cases, `Argument 0` contains an address and `Argument 1` contains a length.
In the case of failure, the address and length are those that were passed in the call.
In the case of success, the address and length are those that were passed in the previous
call. On the first successful invocation of a particular Allow system call, the kernel returns
address 0 and size 0.

The buffer identifier specifies which buffer this is. A driver may support multiple
allowed buffers. For example, the console driver has a read buffer (buffer identifier 2)
and a write buffer (buffer identifier 1).

The Tock kernel MUST check that the passed buffer is contained within the application's
address space. Specifically, every byte of the passed buffer must be readable and writeable
by the process. Zero-length buffers may therefore have abitrary addresses.

Because a process relinquishes access to a buffer when it makes an Allow call with it,
the buffer passed on the subsequent Allow call cannot overlap with the first passed buffer.
This is because the application does not have access to that memory. If an application needs
to extend a buffer, it must first call Allow to reclaim the buffer, then call Allow again
to re-allow it with a different size.


4.5 Read-Only Allow (Class ID: 4)
---------------------------------

The Read-Only Allow class is identical to the Allow class with one exception: the buffer
it passes to the kernel is read-only: the kernel cannot write to it. It semantics and
calling conventions are otherwise identical to Allow.

The Read-Only Allow class exists so that userspace can pass references to constant data
to the kernel. Often, constant data is stored in flash rather than RAM. Constant data
stored in flash cannot be passed with Allow because it is not within the writeable address
space of the process, and so the call will fail. If there is no Read-Only allow, userspace
processes must copy constant data into RAM buffers to pass with Allow, which wastes RAM and
introduces more complex memory management.

4.6 Memop (Class ID: 5)
---------------------------------

To be described.






N Authors' Address
=================================

email - amit@amitlevy.com
email - pal@cs.stanford.edu
