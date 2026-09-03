SHELL := /bin/bash

CARGO_LLVM_COV_VERSION := 0.9.0
RUST_CODE_ANALYSIS_VERSION := 0.0.25
COVERAGE_ROOT := target/coverage
QUALITY_ROOT := target/quality

.PHONY: \
	install-analysis-tools \
	format-check \
	clippy \
	clippy-strict \
	complexity \
	quality \
	quality-strict \
	coverage-unit \
	coverage-bdd \
	coverage-all

install-analysis-tools:
	rustup component add llvm-tools-preview clippy rustfmt
	cargo install cargo-llvm-cov --version $(CARGO_LLVM_COV_VERSION) --locked
	cargo install rust-code-analysis-cli --version $(RUST_CODE_ANALYSIS_VERSION) --locked

format-check:
	cargo fmt --all -- --check

clippy:
	cargo clippy --all-targets --all-features --locked

clippy-strict:
	cargo clippy --all-targets --all-features --locked -- -D warnings

complexity:
	mkdir -p $(QUALITY_ROOT)/rust-code-analysis
	rust-code-analysis-cli \
		-m \
		-p src \
		-I '*.rs' \
		-O json \
		--pr \
		-o $(QUALITY_ROOT)/rust-code-analysis

quality: format-check clippy complexity

quality-strict: format-check clippy-strict complexity

coverage-unit:
	cargo llvm-cov clean --workspace
	cargo llvm-cov --locked --lib --no-report
	mkdir -p $(COVERAGE_ROOT)/unit
	cargo llvm-cov report
	cargo llvm-cov report --json --summary-only --output-path $(COVERAGE_ROOT)/unit/summary.json
	cargo llvm-cov report --lcov --output-path $(COVERAGE_ROOT)/unit/lcov.info
	cargo llvm-cov report --html --output-dir $(COVERAGE_ROOT)/unit

coverage-bdd:
	cargo llvm-cov clean --workspace
	cargo llvm-cov --locked --test cucumber --no-report
	mkdir -p $(COVERAGE_ROOT)/bdd
	cargo llvm-cov report
	cargo llvm-cov report --json --summary-only --output-path $(COVERAGE_ROOT)/bdd/summary.json
	cargo llvm-cov report --lcov --output-path $(COVERAGE_ROOT)/bdd/lcov.info
	cargo llvm-cov report --html --output-dir $(COVERAGE_ROOT)/bdd

coverage-all:
	cargo llvm-cov clean --workspace
	cargo llvm-cov --locked --no-report
	mkdir -p $(COVERAGE_ROOT)/all
	cargo llvm-cov report
	cargo llvm-cov report --json --summary-only --output-path $(COVERAGE_ROOT)/all/summary.json
	cargo llvm-cov report --lcov --output-path $(COVERAGE_ROOT)/all/lcov.info
	cargo llvm-cov report --html --output-dir $(COVERAGE_ROOT)/all
