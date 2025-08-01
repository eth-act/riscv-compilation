# **GETH RISC-V Stateless Transaction Execution**

This project aims to compile a Go binary targeting the RISC-V architecture and execute Ethereum transactions statelessly.
An additional goal is to inspect the syscalls invoked during stateless transaction execution. The binary employs codes from the go ethereum `t8n` cli-tool.


## Compilation of binary
_note: You migth encounter some error parforming this compilation on a Mac or Windows PC._
```bash 
GOOS=tamago GOARCH=riscv64 /home/guest/Documents/duc-works/tamago-go-latest/bin/go build -tags sifive_u,semihosting,tinygo.wasm -trimpath -ldflags "-T 0x80010000 -R 0x1000" -o evm
```

if you find this `/home/guest/Documents/duc-works/tamago-go-latest/bin/go` confusing, this comes from the installation of tamago,
it is an extension of the Go compiler, enabling the compilation of bare-metal binaries inclusing RISC-V.

See: [Tamago](https://github.com/usbarmory/tamago), for a detailed installation guide.

## **Emulating a RISC-V Environment**
Compiling this binary retruns to you a bare-matal riscv bin, which you might not be able to run on your machine. To emulate a RISC-V environment, you can use QEMU.

Before proceeding, you’ll need to run this setup on a Linux distribution with a RISC-V64 CPU. Since such hardware is hard to come by, you’ll emulate a RISC-V64 system using QEMU and run Ubuntu on it.

Documentation:
👉 [Ubuntu RISC-V Boards Documentation](https://canonical-ubuntu-boards.readthedocs-hosted.com/en/latest/how-to/qemu-riscv/)


## Running Binary

```bash
GODEBUG=asyncpreemptoff=1 GOGC=off GOMAXPROCS=1 strace -o geth_strace.log ./geth_evm_riscv64_linux t8n   --input.alloc=./assets/alloc.json   --input.txs=./assets/tx.json   --input.env=./assets/env.json   --state.fork=Prague
```

## Running Binary with Strace

```bash
GODEBUG=asyncpreemptoff=1 GOGC=off GOMAXPROCS=1 strace -o geth_strace.log ./geth_evm_riscv64_linux t8n   --input.alloc=./assets/alloc.json   --input.txs=./assets/tx.json   --input.env=./assets/env.json   --state.fork=Prague
```
