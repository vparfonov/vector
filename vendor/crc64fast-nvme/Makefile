PROJECT_NAME := crc64fast_nvme

# Detect operating system
UNAME_S := $(shell uname -s)

# Allow override of installation directory
DESTDIR ?=

# Determine OS-specific variables
ifeq ($(UNAME_S),Linux)
    LIB_EXTENSION := so
    PREFIX ?= /usr/local
    INSTALL_LIB_DIR := $(PREFIX)/lib
    INSTALL_INCLUDE_DIR := $(PREFIX)/include
    POST_INSTALL := ldconfig
else ifeq ($(UNAME_S),Darwin)
    LIB_EXTENSION := dylib
    # on macOS, there's not really a default location, so require DESTDIR
    ifeq ($(DESTDIR),)
        $(error On macOS, DESTDIR must be set for installation)
    endif
    INSTALL_LIB_DIR := /lib
    INSTALL_INCLUDE_DIR := /include
    POST_INSTALL := true
else
    # Windows
    LIB_EXTENSION := dll
    ifeq ($(DESTDIR),)
        $(error On Windows, DESTDIR must be set for installation)
    endif
    # Use relative paths when DESTDIR is set to avoid path joining issues
    PREFIX ?= Program Files\\$(PROJECT_NAME)
    INSTALL_LIB_DIR := $(PREFIX)\\bin
    INSTALL_INCLUDE_DIR := $(PREFIX)\\include
    POST_INSTALL := true
endif

# Library name with extension
LIB_NAME := lib$(PROJECT_NAME).$(LIB_EXTENSION)

# Default target
.PHONY: all
all: build

# Build the library using Cargo
.PHONY: build
build:
	cargo build --release

# Test the library using Cargo
.PHONY: test
test:
	cargo test

# Install the library and headers
.PHONY: install
install: build
	@install -d $(DESTDIR)$(INSTALL_LIB_DIR)
	@install -d $(DESTDIR)$(INSTALL_INCLUDE_DIR)

	install -m 755 target/release/$(LIB_NAME) $(DESTDIR)$(INSTALL_LIB_DIR)/

	install -m 644 $(PROJECT_NAME).h $(DESTDIR)$(INSTALL_INCLUDE_DIR)/

	@if [ -z "$(DESTDIR)" ] && [ "$(POST_INSTALL)" != "true" ]; then \
		$(POST_INSTALL); \
	fi

# Uninstall the library and headers
.PHONY: uninstall
uninstall:
	rm -f $(DESTDIR)$(INSTALL_LIB_DIR)/$(LIB_NAME)
	rm -f $(DESTDIR)$(INSTALL_INCLUDE_DIR)/$(PROJECT_NAME).h

	@if [ -z "$(DESTDIR)" ] && [ "$(UNAME_S)" = "Linux" ]; then \
		ldconfig; \
	fi

# Clean build artifacts
.PHONY: clean
clean:
	cargo clean

# Print installation paths (useful for debugging)
.PHONY: print-paths
print-paths:
	@echo "Installation paths:"
	@echo "Library dir: $(DESTDIR)$(INSTALL_LIB_DIR)"
	@echo "Include dir: $(DESTDIR)$(INSTALL_INCLUDE_DIR)"