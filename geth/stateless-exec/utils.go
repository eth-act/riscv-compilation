package main

import (
	"bytes"
	"crypto/ecdsa"
	"fmt"

	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/core/vm"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/ethereum/go-ethereum/params"
	"github.com/ethereum/go-ethereum/rlp"
)

const (
	ErrorEVM = 2
	ErrorConfig = 3
	ErrorMissingBlockhash = 4
	ErrorJson = 10
	ErrorIO   = 11
	stdinSelector = "stdin"
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

type rlpTxIterator struct {
	in *rlp.Stream
}

func newRlpTxIterator(rlpData []byte) txIterator {
	in := rlp.NewStream(bytes.NewBuffer(rlpData), 1024*1024)
	in.List()
	return &rlpTxIterator{in}
}
func (it *rlpTxIterator) Next() bool {
	return it.in.MoreDataInList()
}

func (it *rlpTxIterator) Tx() (*types.Transaction, error) {
	var a types.Transaction
	if err := it.in.Decode(&a); err != nil {
		return nil, err
	}
	return &a, nil
}


func stringToKey(s string) (*ecdsa.PrivateKey, error) {
	key := common.HexToHash(s)
	k := key.Hex()[2:]
	return crypto.HexToECDSA(k)
}
