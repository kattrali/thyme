SRC_FILES=$(shell git ls-files libthyme ui src) Cargo.toml
# Target installation directory
DESTDIR := /usr/local/bin
# Command to install file
INSTALLCMD := install -C

DEV_FILE=target/debug/thyme
PROD_FILE=target/release/thyme

all: build

$(DEV_FILE): $(SRC_FILES)
	@cargo build

$(PROD_FILE): $(SRC_FILES)
	@cargo build --release

.PHONY:

build: $(DEV_FILE)

release: $(PROD_FILE)

install: $(PROD_FILE)
	@$(INSTALLCMD) $(PROD_FILE) $(DESTDIR)/thyme

uninstall:
	@rm $(DESTDIR)/thyme

clean:
	@$(MAKE) -C libthyme clean
	@$(MAKE) -C ui clean
	@cargo clean

run: $(DEV_FILE)
	./$(DEV_FILE)

test:
	@$(MAKE) -C libthyme test
	@$(MAKE) -C ui test
	@cargo test
