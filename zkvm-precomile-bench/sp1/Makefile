# Makefile for executing all guest programs in SP1
# Usage:
#   make all      - Run all guest programs
#   make <program> - Run a specific guest program
#   make clean    - Clean up any generated files (optional)

# Set shell to bash
SHELL := /bin/bash

# Base directory for all guest programs
BASE_DIR := $(shell pwd)/bin

# Define the guest programs
PROGRAMS := arks-pairing arks-pairing-patched bigint-paring bigint-paring-patched bn-paring bn-paring-patched

# Default target
.PHONY: all $(PROGRAMS) clean

all: $(PROGRAMS)
	@echo "All guest programs executed successfully!"

# Rules for each guest program
arks-pairing:
	@echo "Executing arks-pairing..."
	cd $(BASE_DIR)/arks-pairing/script && RUST_LOG=info cargo run --release -- --execute

arks-pairing-patched:
	@echo "Executing arks-pairing-patched..."
	cd $(BASE_DIR)/arks-pairing-patched/script && RUST_LOG=info cargo run --release -- --execute

bigint-paring:
	@echo "Executing bigint-paring..."
	cd $(BASE_DIR)/bigint-paring/script && RUST_LOG=info cargo run --release -- --execute

bigint-paring-patched:
	@echo "Executing bigint-paring-patched..."
	cd $(BASE_DIR)/bigint-paring-patched/script && RUST_LOG=info cargo run --release -- --execute

bn-paring:
	@echo "Executing bn-paring..."
	cd $(BASE_DIR)/bn-paring/script && RUST_LOG=info cargo run --release -- --execute

bn-paring-patched:
	@echo "Executing bn-paring-patched..."
	cd $(BASE_DIR)/bn-paring-patched/script && RUST_LOG=info cargo run --release -- --execute

# Clean target (optional)
clean:
	@echo "Cleaning up..."
	# Add any cleanup commands if needed
