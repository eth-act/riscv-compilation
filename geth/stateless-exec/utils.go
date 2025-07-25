package main

import (
	"fmt"

	"github.com/ethereum/go-ethereum/core/vm"
	"github.com/ethereum/go-ethereum/params"
)

const (
	ErrorJson = 10
)

// obtainChainConfig returns the fork configuration for block execution
func obtainChainConfig() *params.ChainConfig {
	// Using Prague fork configuration (similar to the Rust implementation)
	return params.MainnetChainConfig
}


// obtainVmConfig returns the VM configuration for block execution
func obtainVmConfig() *vm.Config {
	// Using default VM configuration
	return &vm.Config{}
}

func NewError(errorCode int, err error) *NumberedError {
	return &NumberedError{errorCode, err}
}

func (n *NumberedError) Error() string {
	return fmt.Sprintf("ERROR(%d): %v", n.errorCode, n.err.Error())
}