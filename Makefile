# Target installation directory
DESTDIR := /usr/local
# Command to create a directory
INSTALLDIRCMD := install -d
# Command to install file
INSTALLCMD := install -C

SRC_FILES=$(shell git ls-files libthyme ui src) Cargo.toml
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
	@$(INSTALLDIRCMD) $(DESTDIR)/bin
	@$(INSTALLCMD) $(PROD_FILE) $(DESTDIR)/bin/thyme

uninstall:
	@rm $(DESTDIR)/bin/thyme

clean:
	@$(MAKE) -C libthyme clean
	@$(MAKE) -C ui clean
	@cargo clean

run:
	@cargo run

test:
	@$(MAKE) -C libthyme test
	@$(MAKE) -C ui test
	@cargo test
