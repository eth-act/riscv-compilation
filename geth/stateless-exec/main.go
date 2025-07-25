package main

import (
	"fmt"

	// _ "github.com/usbarmory/tamago/board/qemu/sifive_u"
)





func main() {
	fmt.Println("Starting stateless block execution")
	
	
	alloc_path := "/Users/gregg/Documents/work/ethereum/riscv-compilation/geth/assets/alloc.json"
	evn_path := "/Users/gregg/Documents/work/ethereum/riscv-compilation/geth/assets/env.json"
	tx_path := "/Users/gregg/Documents/work/ethereum/riscv-compilation/geth/assets/tx.json"
	
	var (
		prestate Prestate
		txIt     txIterator
		inputData = obtainAssets(alloc_path, evn_path, tx_path)
		chainConfig = obtainChainConfig()
	)
	
	
	prestate.Pre = inputData.Alloc
	prestate.Env = *inputData.Env
	
	
	fmt.Println("Loading transactions")
	txIt, txit_err := loadTransactions(tx_path, inputData, chainConfig)
	if txit_err != nil {
		panic("Transactions failed to load")
	}
	
	fmt.Println("Applying london checks")

	
	if err := applyLondonChecks(&prestate.Env, chainConfig); err != nil {
		panic("An error occurred while applying London checks")
	}
	if err := applyShanghaiChecks(&prestate.Env, chainConfig); err != nil {
		panic("An error occurred while applying shanghai checks")
	}
	if err := applyMergeChecks(&prestate.Env, chainConfig); err != nil {
		panic("An error occurred while applying merge checks")
	}
	// if err := applyCancunChecks(&prestate.Env, chainConfig); err != nil {
	// 	return err
	// }
	
	
}



