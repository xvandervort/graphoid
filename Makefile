# Graphoid Makefile
#
# Usage:
#   make build                    Build release binary
#   make install                  Install to ~/.local (default)
#   make install PREFIX=/usr/local  Install system-wide
#   make uninstall                Remove installation
#   make test                     Run tests
#   make clean                    Clean build artifacts
#

PREFIX ?= $(HOME)/.local
BINDIR = $(PREFIX)/bin
DATADIR = $(PREFIX)/share/graphoid

CARGO = cargo

.PHONY: build install uninstall test spec clean help

build:
	$(CARGO) build --release

install: build
	@echo "Installing Graphoid to $(PREFIX)"
	install -d $(BINDIR)
	install -d $(DATADIR)
	install -m 755 target/release/gr $(BINDIR)/gr
	rm -rf $(DATADIR)/stdlib
	cp -r stdlib $(DATADIR)/stdlib
	@echo ""
	@echo "Installed:"
	@echo "  Binary:  $(BINDIR)/gr"
	@echo "  Stdlib:  $(DATADIR)/stdlib/"
	@echo ""
	@if echo "$$PATH" | grep -qv "$(BINDIR)"; then \
		echo "NOTE: $(BINDIR) may not be in your PATH."; \
		echo "Add to your shell profile:"; \
		echo "  export PATH=\"$(BINDIR):\$$PATH\""; \
		echo ""; \
	fi
	@echo "Test with: gr version"

uninstall:
	@echo "Removing Graphoid from $(PREFIX)"
	rm -f $(BINDIR)/gr
	rm -rf $(DATADIR)
	@echo "Done."

test:
	$(CARGO) test --lib

spec: build
	./target/release/gr spec tests/gspec/

clean:
	$(CARGO) clean

help:
	@echo "Graphoid Makefile"
	@echo ""
	@echo "Targets:"
	@echo "  build      Build release binary"
	@echo "  install    Install to PREFIX (default: ~/.local)"
	@echo "  uninstall  Remove installation"
	@echo "  test       Run Rust unit tests"
	@echo "  spec       Run Graphoid spec tests"
	@echo "  clean      Clean build artifacts"
	@echo ""
	@echo "Variables:"
	@echo "  PREFIX     Installation prefix (default: ~/.local)"
	@echo ""
	@echo "Examples:"
	@echo "  make install                     # User install"
	@echo "  sudo make install PREFIX=/usr/local  # System install"
