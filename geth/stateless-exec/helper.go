package main

import (
	"fmt"
	"math/big"

	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/consensus/ethash"
	"github.com/ethereum/go-ethereum/consensus/misc/eip1559"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/params"
)


func applyLondonChecks(env *stEnv, chainConfig *params.ChainConfig) error {
	if !chainConfig.IsLondon(big.NewInt(int64(env.Number))) {
		return nil
	}
	// Sanity check, to not `panic` in state_transition
	if env.BaseFee != nil {
		// Already set, base fee has precedent over parent base fee.
		return nil
	}
	if env.ParentBaseFee == nil || env.Number == 0 {
		return NewError(ErrorConfig, fmt.Errorf("EIP-1559 config but missing 'parentBaseFee' in env section"))
	}
	env.BaseFee = eip1559.CalcBaseFee(chainConfig, &types.Header{
		Number:   new(big.Int).SetUint64(env.Number - 1),
		BaseFee:  env.ParentBaseFee,
		GasUsed:  env.ParentGasUsed,
		GasLimit: env.ParentGasLimit,
	})
	return nil
}

func applyShanghaiChecks(env *stEnv, chainConfig *params.ChainConfig) error {
	if !chainConfig.IsShanghai(big.NewInt(int64(env.Number)), env.Timestamp) {
		return nil
	}
	if env.Withdrawals == nil {
		return NewError(ErrorConfig, fmt.Errorf("Shanghai config but missing 'withdrawals' in env section"))
	}
	return nil
}

// calcDifficulty is based on ethash.CalcDifficulty. This method is used in case
// the caller does not provide an explicit difficulty, but instead provides only
// parent timestamp + difficulty.
// Note: this method only works for ethash engine.
func calcDifficulty(config *params.ChainConfig, number, currentTime, parentTime uint64,
	parentDifficulty *big.Int, parentUncleHash common.Hash) *big.Int {
	uncleHash := parentUncleHash
	if uncleHash == (common.Hash{}) {
		uncleHash = types.EmptyUncleHash
	}
	parent := &types.Header{
		ParentHash: common.Hash{},
		UncleHash:  uncleHash,
		Difficulty: parentDifficulty,
		Number:     new(big.Int).SetUint64(number - 1),
		Time:       parentTime,
	}
	return ethash.CalcDifficulty(config, currentTime, parent)
}

func applyMergeChecks(env *stEnv, chainConfig *params.ChainConfig) error {
	isMerged := chainConfig.TerminalTotalDifficulty != nil && chainConfig.TerminalTotalDifficulty.BitLen() == 0
	if !isMerged {
		// pre-merge: If difficulty was not provided by caller, we need to calculate it.
		if env.Difficulty != nil {
			// already set
			return nil
		}
		switch {
		case env.ParentDifficulty == nil:
			return NewError(ErrorConfig, fmt.Errorf("currentDifficulty was not provided, and cannot be calculated due to missing parentDifficulty"))
		case env.Number == 0:
			return NewError(ErrorConfig, fmt.Errorf("currentDifficulty needs to be provided for block number 0"))
		case env.Timestamp <= env.ParentTimestamp:
			return NewError(ErrorConfig, fmt.Errorf("currentDifficulty cannot be calculated -- currentTime (%d) needs to be after parent time (%d)",
				env.Timestamp, env.ParentTimestamp))
		}
		env.Difficulty = calcDifficulty(chainConfig, env.Number, env.Timestamp,
			env.ParentTimestamp, env.ParentDifficulty, env.ParentUncleHash)
		return nil
	}
	// post-merge:
	// - random must be supplied
	// - difficulty must be zero
	switch {
	case env.Random == nil:
		return NewError(ErrorConfig, fmt.Errorf("post-merge requires currentRandom to be defined in env"))
	case env.Difficulty != nil && env.Difficulty.BitLen() != 0:
		return NewError(ErrorConfig, fmt.Errorf("post-merge difficulty must be zero (or omitted) in env"))
	}
	env.Difficulty = nil
	return nil
}