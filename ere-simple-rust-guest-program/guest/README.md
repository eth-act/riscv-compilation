# Guest Program - RISC-V Executable for zkVMs

This directory contains a minimal Rust program compiled to RISC-V that can execute inside zero-knowledge virtual machines (zkVMs) like SP1 and RISC0.

## Overview

The guest program is a `no_std` Rust application that demonstrates:
- Atomic operations in a constrained environment
- Heap allocation without the standard library
- Platform-specific runtime implementations for different zkVMs
- Custom panic handling and memory management

## Architecture

### Entry Point and Initialization

The program uses inline assembly to bootstrap execution:

```assembly
.section .text._start;
.globl _start;
_start:
    .option push;
    .option norelax;
    la gp, __global_pointer$;    # Initialize global pointer
    .option pop;
    la sp, STACK_TOP              # Load stack pointer
    lw sp, 0(sp)
    call __start;                 # Jump to Rust code
```

This sets up:
1. **Global Pointer (GP)**: Used by the linker for optimized addressing
2. **Stack Pointer (SP)**: Points to `0x00200400`
3. **Program Counter**: Transfers control to the `__start` function

### Program Logic

The main function (`src/main.rs`) performs two verification tests:

#### Test 1: Atomic Variables
```rust
let a: AtomicU16 = AtomicU16::new(5);
let b: AtomicU16 = AtomicU16::new(7);
if a.load(Ordering::SeqCst) + b.load(Ordering::SeqCst) != 12 {
    panic!("Something went wrong!");
}
```

#### Test 2: Vector Allocation
```rust
let mut v: Vec<AtomicU16> = Vec::new();
v.push(AtomicU16::new(5));
v.push(AtomicU16::new(7));
if v[0].load(Ordering::SeqCst) + v[1].load(Ordering::SeqCst) != 12 {
    panic!("Something went wrong!");
}
```

These tests verify:
- Atomic operations work correctly
- Memory allocation functions properly
- The vector implementation operates in a no-std environment

## Platform-Specific Runtimes

### SP1 Runtime (`src/sp1/mod.rs`)

**Environment Call (ecall)**:
```rust
unsafe {
    core::arch::asm!(
        "ecall",
        in("t0") 0x00_00_00_00,  // Halt syscall
        in("a0") 0                // Exit code
    );
}
```

**Features**:
- Simple exit mechanism via ecall
- Compatible with SP1's syscall interface
- Minimal overhead

### RISC0 Runtime (`src/risc0/mod.rs`)

**Environment Call (ecall)**:
```rust
const EMPTY_OUTPUT: [u32; 8] = [0; 8];
unsafe {
    core::arch::asm!(
        "ecall",
        in("t0") 0,                // Halt syscall
        in("a0") 0,                // Exit code
        in("a1") &EMPTY_OUTPUT,    // Output buffer
    );
}
```

**Features**:
- Includes output buffer for returning data
- RISC0-specific syscall convention
- Supports data marshaling between host and guest

## Memory Management

### Heap Allocator

Both runtimes implement a simple bump allocator:

```rust
pub unsafe extern "C" fn sys_alloc_aligned(bytes: usize, align: usize) -> *mut u8 {
    static mut HEAP_POS: usize = 0;
    
    // Initialize heap at _end symbol
    if HEAP_POS == 0 {
        HEAP_POS = &_end as *const u8 as usize;
    }
    
    // Align the pointer
    let offset = HEAP_POS & (align - 1);
    if offset != 0 {
        HEAP_POS += align - offset;
    }
    
    let ptr = HEAP_POS as *mut u8;
    HEAP_POS += bytes;
    
    // Check memory limit
    if HEAP_POS > MAX_MEMORY {
        panic!("Memory limit exceeded");
    }
    
    ptr
}
```

**Characteristics**:
- **Type**: Bump allocator (never deallocates)
- **Start**: `_end` symbol (end of .data section)
- **Limit**: `0x78000000` (2GB)
- **Growth**: Upward from the end of static data
- **Alignment**: Supports arbitrary alignment requirements

### Memory Layout

```
0x00200800  ┌─────────────────┐
            │   Image Base    │
0x00200400  ├─────────────────┤
            │   Stack Top     │
            ├─────────────────┤
0x00201000  │  Text Section   │
            │    (.text)      │
            ├─────────────────┤
            │  Data Section   │
            │    (.data)      │
            ├─────────────────┤
   _end  -> │  Heap Start     │
            │       ↓         │
            │   (grows up)    │
            │       ↓         │
            ├─────────────────┤
0x78000000  │  Heap Limit     │
            └─────────────────┘
```

## Panic Handler

The panic handler uses the `fence` instruction to halt execution:

```rust
#[panic_handler]
fn panic_impl(_panic_info: &core::panic::PanicInfo) -> ! {
    unsafe { 
        core::arch::asm!("fence", options(noreturn)) 
    };
}
```

The `fence` instruction:
- Ensures memory ordering
- Acts as a synchronization barrier
- Causes the zkVM to halt execution
- Is visible in the zkVM's execution trace

## Building

### Prerequisites

Install the RISC-V target:
```bash
rustup target add riscv32im-unknown-none-elf
```

### Build Commands

**For SP1**:
```bash
cargo build --release --features sp1-guest
```

**For RISC0**:
```bash
cargo build --release --features risc0-guest
```

**Output Location**:
```
target/riscv32im-unknown-none-elf/release/general-rust-guest-program
```

### Build Configuration

The `.cargo/config.toml` file configures:

```toml
[build]
target = "riscv32im-unknown-none-elf"
rustflags = [
    "-C", "passes=lower-atomic",           # Lower atomics to lock-free ops
    "-C", "link-arg=-Ttext=0x00201000",    # Text section address
    "-C", "link-arg=--image-base=0x00200800", # Image base address
    "-C", "panic=abort",                    # No unwinding
    "--cfg", "getrandom_backend=\"custom\"" # Custom RNG backend
]
```

## Dependencies

### portable-atomic

```toml
portable-atomic = { 
    version = "1", 
    default-features = false, 
    features = ["unsafe-assume-single-core"] 
}
```

Provides atomic operations without OS support:
- Lock-free implementations
- Single-core optimization (no actual atomicity needed in zkVM)
- No-std compatible

### portable-atomic-util

```toml
portable-atomic-util = { 
    version = "0.2.4", 
    default-features = false, 
    features = ["alloc"] 
}
```

Additional utilities:
- Atomic types with allocation support
- Works with the custom allocator
- Provides `Vec<AtomicU16>` support

## Feature Flags

- **sp1-guest**: Includes SP1-specific runtime code
- **risc0-guest**: Includes RISC0-specific runtime code

Only one should be enabled at a time.

## Debugging

### Common Issues

**1. Linker Errors**

If you see undefined symbols:
```bash
cargo clean
rustup update
cargo build --release --features sp1-guest
```

**2. Memory Limit Exceeded**

Reduce allocation sizes or increase `MAX_MEMORY` in the runtime:
```rust
pub const MAX_MEMORY: usize = 0x78000000;  // Increase if needed
```

**3. Panic During Execution**

Check the zkVM logs for the panic message. Common causes:
- Atomic operations not working correctly
- Memory allocation failures
- Incorrect feature flag

### Inspecting the Binary

View sections:
```bash
riscv64-unknown-elf-objdump -h target/riscv32im-unknown-none-elf/release/general-rust-guest-program
```

Disassemble:
```bash
riscv64-unknown-elf-objdump -d target/riscv32im-unknown-none-elf/release/general-rust-guest-program
```

Check symbols:
```bash
riscv64-unknown-elf-nm target/riscv32im-unknown-none-elf/release/general-rust-guest-program
```

## Extending the Program

### Adding New Functionality

1. Keep the `no_std` requirement
2. Avoid operations that require OS support
3. Use heap allocation sparingly
4. Test with both zkVM backends

### Example: Adding Computation

```rust
fn compute_fibonacci(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    let mut a = 0;
    let mut b = 1;
    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    b
}

fn main() {
    let result = compute_fibonacci(10);
    if result != 55 {
        panic!("Fibonacci calculation failed!");
    }
}
```

### Adding Inputs/Outputs

For input/output, modify the ecall section:

```rust
// Read from host
let input: u32 = zkvm_read_u32();

// Write to host
zkvm_write_u32(result);
```

Implement these functions in the platform-specific modules.

## Performance Considerations

- **Atomic Operations**: Minimal overhead in zkVM (single-threaded)
- **Heap Allocation**: Fast bump allocation, no deallocation overhead
- **Vector Operations**: Efficient in-place growth
- **Panic**: Immediate halt, no cleanup needed

## References

- [RISC-V ISA Specification](https://riscv.org/technical/specifications/)
- [Rust Embedded Book](https://rust-embedded.github.io/book/)
- [portable-atomic Documentation](https://docs.rs/portable-atomic/)
- [SP1 Documentation](https://docs.succinct.xyz/)
- [RISC0 Documentation](https://dev.risczero.com/)