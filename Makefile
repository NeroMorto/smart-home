## help: Show this help
help: Makefile
	@echo "Choose a command run:"
	@sed -n 's/^##//p' $< | column -t -s ':' |  sed -e 's/^/ /'



## test: Run tests:
test:
	cargo test --all

## lint: Run lints (check, clippy and fmt --check)
lint:
	cargo check --all && cargo clippy --all-targets && cargo fmt --all --check

## format: Format source code
format:
	cargo fmt --all

## run: Run app example
run:
	cargo run example_app --release
