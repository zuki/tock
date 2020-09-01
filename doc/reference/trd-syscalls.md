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

N Authors' Address
=================================

email - amit@amitlevy.com
email - pal@cs.stanford.edu
