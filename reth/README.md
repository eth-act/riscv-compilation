# **RETH RISC-V Stateless Block Execution**

This project aims to compile a RUST binary targeting the RISC-V architecture, providing a stateless block execution environment. This binary would be used to inspect the syscalls that would be invoked by the RISC-V binary.

## **Compiling the Binary**
```bash
cargo build --release --target riscv64gc-unknown-linux-gnu
```

Move bin from target to reth/

You might have some linker issue when compling with Mac, Linux was used for this compilation. --
`exec-block` bin.

## Running block Execution
```bash
./exec-block
```

## Running block Execution [with strace]
```bash
strace -o reth_trace.log ./exec-block
```

## **Emulating a RISC-V Environment**

Before proceeding, you’ll need to run this setup on a Linux distribution with a RISC-V64 CPU. Since such hardware is hard to come by, you’ll emulate a RISC-V64 system using QEMU and run Ubuntu on it.

Documentation:
👉 [Ubuntu RISC-V Boards Documentation](https://canonical-ubuntu-boards.readthedocs-hosted.com/en/latest/how-to/qemu-riscv/)