package main

import (
	"crypto/ecdsa"
	"encoding/json"
	"math/big"

	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/common/math"
	"github.com/ethereum/go-ethereum/core/stateless"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/crypto"
)

// StatelessInput represents the input data for stateless block execution
type StatelessInput struct {
	Block   *types.Block      `json:"block"`
	Witness *stateless.Witness `json:"witness"`
}

type ommer struct {
	Delta   uint64         `json:"delta"`
	Address common.Address `json:"address"`
}

type stEnv struct {
	Coinbase              common.Address                      `json:"currentCoinbase"   gencodec:"required"`
	Difficulty            *big.Int                            `json:"currentDifficulty"`
	Random                *big.Int                            `json:"currentRandom"`
	ParentDifficulty      *big.Int                            `json:"parentDifficulty"`
	ParentBaseFee         *big.Int                            `json:"parentBaseFee,omitempty"`
	ParentGasUsed         uint64                              `json:"parentGasUsed,omitempty"`
	ParentGasLimit        uint64                              `json:"parentGasLimit,omitempty"`
	GasLimit              uint64                              `json:"currentGasLimit"   gencodec:"required"`
	Number                uint64                              `json:"currentNumber"     gencodec:"required"`
	Timestamp             uint64                              `json:"currentTimestamp"  gencodec:"required"`
	ParentTimestamp       uint64                              `json:"parentTimestamp,omitempty"`
	BlockHashes           map[math.HexOrDecimal64]common.Hash `json:"blockHashes,omitempty"`
	Ommers                []ommer                             `json:"ommers,omitempty"`
	Withdrawals           []*types.Withdrawal                 `json:"withdrawals,omitempty"`
	BaseFee               *big.Int                            `json:"currentBaseFee,omitempty"`
	ParentUncleHash       common.Hash                         `json:"parentUncleHash"`
	ExcessBlobGas         *uint64                             `json:"currentExcessBlobGas,omitempty"`
	ParentExcessBlobGas   *uint64                             `json:"parentExcessBlobGas,omitempty"`
	ParentBlobGasUsed     *uint64                             `json:"parentBlobGasUsed,omitempty"`
	ParentBeaconBlockRoot *common.Hash                        `json:"parentBeaconBlockRoot"`
}

// txWithKey is a helper-struct, to allow us to use the types.Transaction along with
// a `secretKey`-field, for input
type txWithKey struct {
	key       *ecdsa.PrivateKey
	tx        *types.Transaction
	protected bool
}

func (t *txWithKey) UnmarshalJSON(input []byte) error {
	// Read the metadata, if present
	type txMetadata struct {
		Key       *common.Hash `json:"secretKey"`
		Protected *bool        `json:"protected"`
	}
	var data txMetadata
	if err := json.Unmarshal(input, &data); err != nil {
		return err
	}
	if data.Key != nil {
		k := data.Key.Hex()[2:]
		if ecdsaKey, err := crypto.HexToECDSA(k); err != nil {
			return err
		} else {
			t.key = ecdsaKey
		}
	}
	if data.Protected != nil {
		t.protected = *data.Protected
	} else {
		t.protected = true
	}
	// Now, read the transaction itself
	var tx types.Transaction
	if err := json.Unmarshal(input, &tx); err != nil {
		return err
	}
	t.tx = &tx
	return nil
}

type input struct {
	Alloc types.GenesisAlloc `json:"alloc,omitempty"`
	Env   *stEnv             `json:"env,omitempty"`
	Txs   []*txWithKey       `json:"txs,omitempty"`
	TxRlp string             `json:"txsRlp,omitempty"`
}

type Prestate struct {
	Env stEnv              `json:"env"`
	Pre types.GenesisAlloc `json:"pre"`
}

type txIterator interface {
	// Next returns true until EOF
	Next() bool
	// Tx returns the next transaction, OR an error.
	Tx() (*types.Transaction, error)
}

type NumberedError struct {
	errorCode int
	err       error
}


type sliceTxIterator struct {
	idx int
	txs []*types.Transaction
}