# Report: Analysis of Linux Syscalls for Stateless Execution on RISC-V

This report details the Linux system calls (`syscalls`) utilized by two different stateless Ethereum execution binaries, compiled for the `riscv64gc-unknown-linux-gnu` target. The initial analysis focuses on a Rust-based `reth-stateless` tool for block execution. This is now extended with a comparative analysis of the Go-based Geth `t8n` tool for single transaction execution. The objective remains to identify the minimal subset of the Linux ABI required to run such programs, paving the way for zkVMs to support languages like Go and C# that depend on a Linux-like syscall environment.


## Part 1: Stateless Block Execution with `reth-stateless` (Rust)

### Methodology

The experiment involved executing a Rust program that performs stateless Ethereum block validation. The program was compiled for a RISC-V 64-bit target with Linux ABI support. To capture the syscalls, the binary was executed within a QEMU-emulated RISC-V virtual machine running a Linux distribution, and the `strace` utility was used to log all interactions between the program and the kernel.

The analysis correctly distinguishes between syscalls made during the initial program loading phase (by the dynamic linker) and those made by the application's core logic. We focus on the latter, which begins with the program's first output: `write(1, "Starting stateless block validat"..., 36)`.

### Syscall Frequency Analysis

The following table presents the system calls used by the `reth-stateless` application's main logic, their frequency, and their purpose.

| Syscall | Frequency | Category | Description |
| :--- | :--- | :--- | :--- |
| `brk` | 45 | Memory Management | Used to change the location of the program break, which defines the end of the process's data segment. It's a primary mechanism for heap memory allocation. |
| `munmap` | 3 | Memory Management | Unmaps memory regions. Used here to release the memory holding the JSON file and the signal stack upon completion. |
| `write` | 2 | File I/O | Writes data to a file descriptor. Used here to print the "Starting..." and "Block validation completed..." messages to standard output. |
| `read` | 2 | File I/O | Reads data from a file descriptor into a buffer. Used to read the contents of the `block_and_witness.json` file. |
| `openat` | 1 | File I/O | Opens a file relative to a directory file descriptor. Used to open `block_and_witness.json`. |
| `statx` | 1 | File I/O | Gets file status. Used to get metadata about the input JSON file, likely to determine its size for memory allocation. |
| `lseek` | 1 | File I/O | Repositions the read/write file offset. Used here to check the current position in the file. |
| `mmap` | 1 | Memory Management | Maps files or devices into memory. Used to allocate a large buffer for the JSON file content. |
| `mremap` | 1 | Memory Management | Expands or shrinks an existing memory mapping. Used during the JSON parsing and data structuring phase. |
| `sigaltstack` | 1 | Process Management | Sets up an alternate signal stack. Used here to tear it down before exiting. |
| `exit_group` | 1 | Process Management | Terminates all threads in the process. This is the final call made to exit the program. |

***

## Part 2: Stateless Transaction Execution with GETH `t8n` (Go)

The analysis was extended to review the syscalls from the Go-based Geth `t8n` tool, which statelessly executes a single transaction. It's important to note this is not an apples-to-apples comparison due to the different scopes (block vs. transaction) and underlying codebases. However, it provides critical insights into the syscall requirements of a Go application in a similar context.

### Syscall Analysis of Geth `t8n`

The `strace` log for the Geth `t8n` tool reveals a significantly more complex syscall profile, characteristic of the Go runtime.

| Syscall | Frequency | Category | Description |
| :--- | :--- | :--- | :--- |
| `rt_sigaction`| 66 | Signal Handling | Manages signal handlers. The Go runtime extensively sets up handlers for various signals. |
| `mmap` | 20 | Memory Management | Maps files or devices into memory. Go's memory manager heavily relies on `mmap` for memory allocation, in contrast to `brk`. |
| `nanosleep` | 13 | Process Management | Pauses the execution of a thread for a specified time. Likely used by the Go scheduler. |
| `rt_sigprocmask`| 8 | Signal Handling | Changes the signal mask of the calling thread. |
| `clone` | 4 | Process Management | Creates a new thread. Essential for Go's goroutine model. |
| `futex` | 3 | Process Management | Provides a mechanism for fast userspace locking and waiting. Critical for goroutine synchronization. |
| `sigaltstack` | 4 | Signal Handling | Manages alternate signal stacks, one for each thread. |
| `gettid` | 4 | Process Management | Returns the thread ID of the calling thread. |
| `fcntl` | 3 | File I/O | Manipulates file descriptors. |
| `openat` | 2 | File I/O | Opens files. Here, it also checks for system information like huge page size. |
| `madvise` | 2 | Memory Management | Advises the kernel about memory usage patterns. |
| `read` | 1 | File I/O | Reads from a file descriptor. |
| `close` | 1 | File I/O | Closes a file descriptor. |
| `sched_getaffinity`| 1 | Process Management | Gets a thread's CPU affinity mask. |
| `getpid` | 1 | Process Management | Returns the process ID. |
| `tgkill` | 1 | Process Management | Sends a signal to a specific thread. |
| `rt_sigreturn` | 1 | Signal Handling | Returns from a signal handler. |


## Part 3: Deeper Analysis of a Single-Core Go Execution

A more detailed trace was captured by running the Geth `t8n` binary while restricting it to a single CPU core. This analysis provides a clearer picture of the Go runtime's non-negotiable setup and execution requirements, revealing several more crucial syscalls.

### Key Observations and Newly Identified Syscalls

The most significant finding is that even when limited to one core, **the Go runtime is not truly single-threaded**. The `strace` log clearly shows calls to `clone`, indicating the runtime still creates threads for its internal mechanisms, such as the garbage collector and scheduler. This confirms that robust support for threading (`clone`, `futex`) is a hard requirement for Go.

Furthermore, this detailed trace uncovered several new syscalls used during the program's initialization phase that are essential for compatibility.

| Syscall | Category | Rationale for zkVM Implementation |
| :--- | :--- | :--- |
| `prlimit64` | Process Management | Used to get and set process resource limits. The Go runtime uses it to increase the limit on open file descriptors (`RLIMIT_NOFILE`). A zkVM can support this by simply acknowledging the request and returning success. |
| `epoll_*` | File I/O | Includes `epoll_create1`, `epoll_ctl`, and `epoll_pwait`. This is a high-performance I/O event notification facility. The Go scheduler's **`netpoller` relies on `epoll`** to wait for I/O readiness efficiently. Even without network I/O, it's used for file operations. Basic emulation of the `epoll` interface is necessary. |
| `ioctl` | Device I/O | Used for device-specific control operations. The log shows it being used to query terminal settings (`TCGETS`). Since a zkVM is a non-interactive environment, these calls can be safely **stubbed to return an appropriate success or error code** (e.g., `ENOTTY` - Not a typewriter). |
| `uname` | System Information | Retrieves system and kernel information. The Go runtime uses this for introspection. A zkVM can easily handle this by returning a fixed, valid `utsname` struct. |


#### What happens when Go Garbage collection is turn on?
As you would expect the exection speed would increase dreamatically, a 4x speed bump was noticed. When GC was turned off, this introduction of a a syscall `sched_yield` was introduced. This syscall allows a thread to voluntarily cede its execution time, telling the kernel to run another available thread. Its appearance here further underscores the cooperative nature of the Go scheduler. 



### The `SIGURG` Signal Pattern

The detailed trace is dominated by `SIGURG` signals. This is not an error. The Go runtime uses this otherwise obscure signal as a mechanism to **preempt long-running goroutines** and return control to its internal scheduler. A zkVM's signal handling mechanism must be robust enough to correctly deliver these signals and handle the subsequent `rt_sigreturn` calls to ensure the Go scheduler functions as expected. It is important to note that when `GODEBUG=asyncpreemptoff=1` flag is activated, the `rt_sigreturn` syscall is dropped.


## Implications for zkVMs and Comparative Analysis

The Geth `t8n` analysis provides a more comprehensive picture of what is required to support a complex, multi-threaded application with its own runtime, like those written in Go.

1.  **Memory Management is More Complex**: The Rust binary primarily uses `brk` for heap allocation, which is a simpler syscall to implement. In stark contrast, Go's runtime heavily favors `mmap` for managing memory, which is a more complex but powerful memory management tool. A zkVM would need a robust `mmap` implementation to support Go applications.

2.  **Threading is a Major Hurdle**: The `reth-stateless` binary is single-threaded, making its process management syscalls minimal. Geth, with its goroutine-based concurrency, extensively uses `clone` to create threads and `futex` for synchronization. Supporting these is a significant step up in complexity for a zkVM, as it requires the hypervisor to manage multiple execution contexts and their synchronization.

3.  **Extensive Signal Handling**: The Go runtime's heavy use of `rt_sigaction` and other signal-related syscalls presents another challenge. While some of this might be boilerplate that can be stubbed out, a zkVM would need to emulate this behavior to a degree that satisfies the Go runtime's expectations.

4.  **File I/O is a Shared Requirement**: Both applications require a basic set of file I/O operations. The principle of pre-loading necessary files into the zkVM's memory and handling these syscalls by operating on that in-memory data remains a viable strategy for both scenarios. (This can be skip though as there is not usecase for a file system in zkVM today)

### The `ECALL` Instruction and an Updated Path Forward

The RISC-V `ECALL` instruction remains the key mechanism for a program to request services from the hypervisor. The initial proposal of a syscall dispatcher that routes `ECALL`s based on the syscall number in a register is still valid, but the scope of that dispatcher is now clearer.

1.  **Tiered Support**: A zkVM could offer tiered support for the Linux ABI. A basic tier could support the minimal set of syscalls identified in the `reth-stateless` analysis, enabling support for Rust and potentially C/C++ applications. A more advanced tier would need to implement the more complex syscalls related to threading and advanced memory management to support Go and other managed runtimes.

2.  **The Syscall Dispatcher Revisited**: The dispatcher would need to differentiate between:
    * **zkVM-specific precompiles** (like `keccak`).
    * **Basic Linux syscalls** (`brk`, `openat`, `read`, `write`, `exit_group`, etc.).
    * **Advanced Linux syscalls** (`clone`, `futex`, `mmap`, `rt_sigaction`, etc.).

By implementing a dispatch logic that can handle this expanded set of syscalls, a zkVM can create a compatible environment for a wider range of `riscv64-linux` binaries. This broader support is crucial for enabling major Ethereum clients like Geth (Go) and Nethermind (C#) to run within a zkVM without requiring forks of their compilers, thereby significantly expanding the potential of the zkVM ecosystem.

<br/>
<br/>
<br/>

Building upon the analysis of `reth-stateless` and Geth `t8n`, a broader review of the `riscv64-linux` syscall list reveals several other candidates that are critical for supporting complex, general-purpose applications. While the initial analysis identified core requirements for memory, threading, and I/O, supporting runtimes like Go's will necessitate emulating a richer subset of the Linux ABI.

The following syscalls are proposed as high-priority additions for a more capable, "advanced tier :)" zkVM.

| Syscall | Category | Rationale for zkVM Implementation |
| :--- | :--- | :--- |
| `clock_gettime` | Time & Clocks | Provides access to system-wide clocks. It is heavily used by schedulers (like Go's), for setting timeouts, logging timestamps, and performance profiling. A zkVM would need to provide a **deterministic clock source**, potentially tied to the execution step or cycle count, to satisfy applications that rely on it. |
| `getrandom` | System & Randomness | Provides a source of **cryptographically secure random numbers**. This is a fundamental requirement for countless modern applications and libraries, from generating unique identifiers to cryptographic key generation. A zkVM must handle this by sourcing entropy from the host environment as a non-deterministic input. |
| `clone3` | Process Management | A more modern and extensible version of the `clone` syscall. As `glibc` and language runtimes evolve, they will increasingly favor `clone3` for its flexibility in thread creation. Supporting it is crucial for **forward compatibility**. |
| `readv` / `writev` | File I/O | These "vector" I/O operations read from or write to multiple memory buffers in a single syscall (`readv` reads into a "vector" of buffers). They are an efficiency optimization used by many high-performance libraries to reduce syscall overhead. Supporting them would be a natural extension of the basic `read`/`write` capabilities. |

_riscv64 linux syscalls source" https://jborza.com/post/2021-05-11-riscv-linux-syscalls/_



The journey to support unmodified, general-purpose binaries in a zkVM is an incremental one. The Geth `t8n` case study broadenly highlights there is alot more to do.
