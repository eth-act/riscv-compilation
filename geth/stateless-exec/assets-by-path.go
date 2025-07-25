package main

import (
	"encoding/json"
	"fmt"
	"io"
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
	
	
	// reading the file contents
	alloc_data, err := io.ReadAll(alloc_file)
	if err != nil {
		panic(fmt.Sprintf("Could not read %s: %v", alloc_path, err))
	}
	evn_data, err := io.ReadAll(evn_file)
	if err != nil {
		panic(fmt.Sprintf("Could not read %s: %v", evn_path, err))
	}
	tx_data, err := io.ReadAll(tx_file)
	if err != nil {
		panic(fmt.Sprintf("Could not read %s: %v", tx_path, err))
	}
	
	// parsing the Json content
	var inputOut input
	if err := json.Unmarshal(alloc_data, &inputOut.Alloc); err != nil {
		panic(fmt.Sprintf("Could not parse %s: %v", alloc_path, err))
	}
	if err := json.Unmarshal(evn_data, &inputOut.Env); err != nil {
		panic(fmt.Sprintf("Could not parse %s: %v", evn_path, err))
	}
	if err := json.Unmarshal(tx_data, &inputOut.Txs); err != nil {
		panic(fmt.Sprintf("Could not parse %s: %v", tx_path, err))
	}
	
	return &inputOut
}