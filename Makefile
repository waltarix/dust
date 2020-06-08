ifeq ($(RUST_TARGET),)
	TARGET := ""
	RELEASE_SUFFIX := ""
else
	TARGET := $(RUST_TARGET)
	RELEASE_SUFFIX := "-$(TARGET)"
	export CARGO_BUILD_TARGET = $(RUST_TARGET)
endif

VERSION := $(word 3,$(shell grep -m1 "^version" Cargo.toml))
RELEASE := dust-$(VERSION)$(RELEASE_SUFFIX)

all: release

dust:
	cargo build --locked --release

bin:
	mkdir -p $@

bin/dust: dust bin
	cp -f target/$(TARGET)/release/dust $@

release: bin/dust
	tar -C bin -Jcvf $(RELEASE).tar.xz dust

.PHONY: all release
