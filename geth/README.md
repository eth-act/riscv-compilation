# **GETH RISC-V Stateless Transaction Execution**

This project aims to compile a Go binary targeting the RISC-V architecture and execute Ethereum transactions statelessly.
An additional goal is to inspect the syscalls invoked during stateless transaction execution.


## **Compiling the Binary**

The objective here is to compile the `evm` binary for the RISC-V architecture.

### **Clone the Geth Repository**

```bash
git clone https://github.com/ethereum/go-ethereum.git
cd go-ethereum
```

### **Build the EVM Binary**

```bash
cd cmd/evm
GOOS=linux GOARCH=riscv64 go build -o evm
```

### **Move the EVM Binary to the Compilation Directory**

```bash
mv evm ../../../riscv-compilation/geth/
```

If you prefer not to compile the binary yourself, you can run the following script instead:

```bash
./geth_tx_asset_and_evm_bin_script
```



## **Download Transaction Assets (TX, Witness, etc.)**

If you compiled the binary yourself, run:

```bash
./geth_tx_asset_script
```

Otherwise, if you’ve already used the full setup script:

```bash
./geth_tx_asset_and_evm_bin_script
```



## **Emulating a RISC-V Environment**

Before proceeding, you’ll need to run this setup on a Linux distribution with a RISC-V64 CPU. Since such hardware is hard to come by, you’ll emulate a RISC-V64 system using QEMU and run Ubuntu on it.

Documentation:
👉 [Ubuntu RISC-V Boards Documentation](https://canonical-ubuntu-boards.readthedocs-hosted.com/en/latest/how-to/qemu-riscv/)

After setting up the emulated environment, transfer the compiled binary or use the provided script. We recommend using the `./geth_tx_asset_and_evm_bin_script.sh` for simplicity.
However, if you prefer to use your own compiled binary, update the script to point to its correct location (possibly in your forked version of the repository).



## **Run the Transaction Statelessly**

### **If You Compiled the Binary Yourself**

```bash
GODEBUG=asyncpreemptoff=1 GOGC=off GOMAXPROCS=1 ./geth_evm_riscv64_linux t8n   --input.alloc=./assets/alloc.json   --input.txs=./assets/tx.json   --input.env=./assets/env.json   --state.fork=Prague
```

### **If You're Using the Precompiled Binary**

```bash
GODEBUG=asyncpreemptoff=1 GOGC=off GOMAXPROCS=1 ./geth_evm_riscv64_linux t8n   --input.alloc=./assets/alloc.json   --input.txs=./assets/tx.json   --input.env=./assets/env.json   --state.fork=Prague
```



## **Run the Transaction With `strace` (Syscall Tracing)**

You can also trace syscalls by running the binary with `strace`.

### **If You Compiled the Binary Yourself**

```bash
GODEBUG=asyncpreemptoff=1 GOGC=off GOMAXPROCS=1 strace -o geth_strace.log ./evm t8n   --input.alloc=./assets/alloc.json   --input.txs=./assets/tx.json   --input.env=./assets/env.json   --state.fork=Prague
```

### **If You're Using the Precompiled Binary**

```bash
GODEBUG=asyncpreemptoff=1 GOGC=off GOMAXPROCS=1 strace -o geth_strace.log ./geth_evm_riscv64_linux t8n   --input.alloc=./assets/alloc.json   --input.txs=./assets/tx.json   --input.env=./assets/env.json   --state.fork=Prague
```
