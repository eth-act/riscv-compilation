# GETH RISCV stateless tx execution

In this project, the goal is the compile a go bin targetting the RISC-V architecture, and execute transactions statelessly.
Another goal of this project is to inspect the syscalls employed when executing a tx statelessly.



## Compiling this binary.
The Goal here is to compile the `evm` binary for RISC-V architecture.

#### Clone geth get repo
```bash 
git clone https://github.com/ethereum/go-ethereum.git
cd go-ethereum
```

#### Build evm binary
```bash 
cd cmd/evm
GOOS=linux GOARCH=riscv64 go build -o evm
```

#### Move evm binary to riscv-compilation/geth
```bash 
mv evm ../../../riscv-compilation/geth/
```

if you don't want to compile the bin yourself, you can run this script:
```bash 
./geth_tx_asset_and_evm_bin_script
```

#### Pull tx asset (tx, witness...)
if you compiled the bin yourself, you can run this script:
```bash 
./geth_tx_asset_script
```

else: (you have already pulled the assets using this script)
```bash 
./geth_tx_asset_and_evm_bin_script
```

