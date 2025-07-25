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
		// prestate Prestate
		txIt     txIterator
		inputData = obtainAssets(alloc_path, evn_path, tx_path)
		chainConfig = obtainChainConfig()
	)
	
	
	fmt.Println("Loading transactions")
	txIt, err := loadTransactions(tx_path, inputData, chainConfig)
	if err != nil {
		panic("Transactions failed to load")
	}
	fmt.Println(txIt)
	
	// if txIt, err = loadTransactions(inputData, chainConfig); err != nil {
	// 	return err
	// }
	// if err := applyLondonChecks(&prestate.Env, chainConfig); err != nil {
	// 	return err
	// }
	// if err := applyShanghaiChecks(&prestate.Env, chainConfig); err != nil {
	// 	return err
	// }
	// if err := applyMergeChecks(&prestate.Env, chainConfig); err != nil {
	// 	return err
	// }
	// if err := applyCancunChecks(&prestate.Env, chainConfig); err != nil {
	// 	return err
	// }
	
	
}



