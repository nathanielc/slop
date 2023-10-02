# Makefile provides an API for CI related tasks
# Using the makefile is not required however CI
# uses the specific targets within the file.
# Therefore may be useful in ensuring a change
# is ready to pass CI checks.

CARGO = RUSTFLAGS="-D warnings" cargo

.PHONY: all
all: build check-fmt check-clippy test

.PHONY: build
build:
	# Build with default features
	${CARGO} build --locked --release

.PHONY: test
test:
	# Test with default features
	${CARGO} test --locked --release

.PHONY: check-fmt
check-fmt:
	${CARGO} fmt --all -- --check

.PHONY: check-clippy
check-clippy:
	# Check with default features
	${CARGO} clippy --workspace --locked --release

.PHONY: slop-app
slop-app:
	$(MAKE) -C app dist

.PHONY: clean
clean:
	$(MAKE) -C app clean
	${CARGO} clean
