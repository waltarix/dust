ifeq ($(RUST_TARGET),)
	TARGET :=
	RELEASE_SUFFIX :=
else
	TARGET := $(RUST_TARGET)
	RELEASE_SUFFIX := -$(TARGET)
	export CARGO_BUILD_TARGET = $(RUST_TARGET)
endif

PROJECT_NAME := dust

VERSION := $(subst $\",,$(word 3,$(shell grep -m1 "^version" Cargo.toml)))
RELEASE := $(PROJECT_NAME)-$(VERSION)$(RELEASE_SUFFIX)

DIST_DIR := dist
RELEASE_DIR := $(DIST_DIR)/$(RELEASE)
COMPLETIONS_DIR := $(RELEASE_DIR)/etc/completions

BINARY := target/$(TARGET)/release/$(PROJECT_NAME)

RELEASE_BINARY := $(RELEASE_DIR)/$(PROJECT_NAME)
COMPLETION_FILES := $(notdir $(wildcard completions/*))
COMPLETIONS := $(addprefix $(COMPLETIONS_DIR)/,$(COMPLETION_FILES))

ARTIFACT := $(RELEASE).tar.xz

.PHONY: all
all: $(ARTIFACT)

$(BINARY):
	cargo build --locked --release

$(DIST_DIR) $(RELEASE_DIR) $(COMPLETIONS_DIR):
	mkdir -p $@

$(RELEASE_BINARY): $(BINARY) | $(RELEASE_DIR)
	cp -f $< $@

$(COMPLETIONS): | $(COMPLETIONS_DIR)
	cp -f completions/$(notdir $@) $@

$(ARTIFACT): $(RELEASE_BINARY) $(COMPLETIONS)
	tar -C $(DIST_DIR) -Jcvf $@ $(RELEASE)

.PHONY: clean
clean:
	$(RM) -r $(ARTIFACT) $(DIST_DIR)
