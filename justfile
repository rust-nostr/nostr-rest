default: build

# Build (release)
build:
	cargo build --release

bench:
	ab -n 100000 -c 16 -k -r -t 60 http://127.0.0.1:7773/ping

# Check format and crates
check: check-fmt check-crate

# Format the code and execute some checks
precommit: fmt
    cargo check
    cargo test
    cargo clippy

# Format the entire Rust code
fmt:
	@bash contrib/scripts/check-fmt.sh

# Check if the Rust code is formatted
check-fmt:
	@bash contrib/scripts/check-fmt.sh check

# Check crate
check-crate:
	@bash contrib/scripts/check-crate.sh

# Remove artifacts that cargo has generated
clean:
	cargo clean

# Count the lines of codes of this project
loc:
	@echo "--- Counting lines of .rs files (LOC):" && find crates/ bindings/ -type f -name "*.rs" -not -path "*/target/*" -exec cat {} \; | wc -l
