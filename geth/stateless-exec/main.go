package main

import (
	"fmt"

	// _ "github.com/usbarmory/tamago/board/qemu/sifive_u"
)





func main() {
	fmt.Println("Starting stateless block execution")
	
	var (
		// prestate Prestate
		txIt     txIterator
		inputData = obtainAssets()
		chainConfig = obtainChainConfig()
	)
	
	
	txIt, err := loadTransactions(inputData, chainConfig)
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



