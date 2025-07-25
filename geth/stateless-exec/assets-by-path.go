package main

import (
	"fmt"
	"os"
)


func obtainAssets() *input {
	
	alloc_path := ""
	evn_path := ""
	tx_path := ""
	
	// opening files
	alloc_file, err := os.Open(alloc_path)
	if err != nil {
		panic(fmt.Sprintf("Could not open %s: %v", alloc_path, err))
	}
	defer alloc_file.Close()
	
	evn_file, err := os.Open(evn_path)
	if err != nil {
		panic(fmt.Sprintf("Could not open %s: %v", evn_path, err))
	}
	defer evn_file.Close()
	
	tx_file, err := os.Open(tx_path)
	if err != nil {
		panic(fmt.Sprintf("Could not open %s: %v", tx_path, err))
	}
	defer tx_file.Close()
	
	var inputOut input
	return &inputOut
}