SRC_FILES=$(shell git ls-files libthyme ui src) Cargo.toml

all: build

target/%/thyme: $(SRC_FILES)
	@cargo build

.PHONY:

build: target/debug/thyme

release: target/release/thyme

clean:
	@$(MAKE) -C libthyme clean
	@$(MAKE) -C ui clean
	@cargo clean

run: target/debug/thyme
	./target/debug/thyme

test:
	@$(MAKE) -C libthyme test
	@$(MAKE) -C ui test
	@cargo test
