package main

import (
	"math/big"

	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/common/math"
	"github.com/ethereum/go-ethereum/core/types"
)

func obtainAssetsStatic() *input {
	key, err := stringToKey("0xb71c71a67e1177ad4e901695e1b4b9ee17ae16c6668d313eac2f96dbcda3f291")
	if err != nil {
	    // handle error, e.g. panic or log
	    panic(err)
	}

	
	return &input{
		Alloc: map[common.Address]types.Account{
			common.HexToAddress("0x8a0a19589531694250d570040a0c4b74576919b8"): {
				Nonce:   0,
				Balance: big.NewInt(0x0de0b6b3a7640000),
				Code:    common.FromHex("0x600060006000600060007310000000000000000000000000000000000000015af1600155600060006000600060007310000000000000000000000000000000000000025af16002553d600060003e600051600355"),
				Storage: map[common.Hash]common.Hash{
					common.HexToHash("0x01"): common.HexToHash("0x0100"),
					common.HexToHash("0x02"): common.HexToHash("0x0100"),
					common.HexToHash("0x03"): common.HexToHash("0x0100"),
				},
			},
			common.HexToAddress("0x000000000000000000000000000000000000aaaa"): {
				Nonce:   0,
				Balance: big.NewInt(0x4563918244f40000),
				Code:    common.FromHex("0x58808080600173703c4b2bd70c169f5717101caee543299fc946c75af100"),
				Storage: map[common.Hash]common.Hash{},
			},
			common.HexToAddress("0x000000000000000000000000000000000000bbbb"): {
				Nonce:   0,
				Balance: big.NewInt(0x29a2241af62c0000),
				Code:    common.FromHex("0x6042805500"),
				Storage: map[common.Hash]common.Hash{},
			},
			common.HexToAddress("0x71562b71999873DB5b286dF957af199Ec94617F7"): {
				Nonce:   0,
				Balance: big.NewInt(0x6124fee993bc0000),
				Code:    common.FromHex("0x"),
				Storage: map[common.Hash]common.Hash{},
			},
		},
		Env: &stEnv{
		    Coinbase: common.HexToAddress("0x2adc25665018aa1fe0e6bc666dac8fc2697ff9ba"),
		    GasLimit: 71794957647893862,
		    Number: 1,
		    Timestamp: 1000,
		    Random: big.NewInt(0),
		    Difficulty: big.NewInt(0),
		    BlockHashes: map[math.HexOrDecimal64]common.Hash{},
		    Ommers: []ommer{},
		    BaseFee: big.NewInt(7),
		    ParentUncleHash: common.HexToHash("0x0000000000000000000000000000000000000000000000000000000000000000"),
		    Withdrawals: []*types.Withdrawal{},
		    ParentBeaconBlockRoot: func() *common.Hash {
		        h := common.HexToHash("0x0000000000000000000000000000000000000000000000000000000000000000")
		        return &h
		    }(),
		},
		Txs: []*txWithKey{
			{
				key: key,
				tx: &types.Transaction{
					
				},
				protected: true,
			},
		},
	}
}