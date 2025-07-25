package main

import (
	"fmt"
	"io"

	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/params"
)


func signUnsignedTransactions(txs []*txWithKey, signer types.Signer) (types.Transactions, error) {
	var signedTxs []*types.Transaction
	for i, tx := range txs {
		var (
			v, r, s = tx.tx.RawSignatureValues()
			signed  *types.Transaction
			err     error
		)
		if tx.key == nil || v.BitLen()+r.BitLen()+s.BitLen() != 0 {
			// Already signed
			signedTxs = append(signedTxs, tx.tx)
			continue
		}
		// This transaction needs to be signed
		if tx.protected {
			signed, err = types.SignTx(tx.tx, signer, tx.key)
		} else {
			signed, err = types.SignTx(tx.tx, types.HomesteadSigner{}, tx.key)
		}
		if err != nil {
			return nil, NewError(ErrorJson, fmt.Errorf("tx %d: failed to sign tx: %v", i, err))
		}
		signedTxs = append(signedTxs, signed)
	}
	return signedTxs, nil
}

func newSliceTxIterator(transactions types.Transactions) txIterator {
	return &sliceTxIterator{0, transactions}
}

func (ait *sliceTxIterator) Next() bool {
	return ait.idx < len(ait.txs)
}

func (ait *sliceTxIterator) Tx() (*types.Transaction, error) {
	if ait.idx < len(ait.txs) {
		ait.idx++
		return ait.txs[ait.idx-1], nil
	}
	return nil, io.EOF
}

func loadTransactions(inputData *input, chainConfig *params.ChainConfig) (txIterator, error) {
	// We may have to sign the transactions.
	signer := types.LatestSignerForChainID(chainConfig.ChainID)
	txs, err := signUnsignedTransactions(inputData.Txs, signer)
	return newSliceTxIterator(txs), err
}