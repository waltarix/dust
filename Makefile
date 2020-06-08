OSTYPE := $(shell uname -s)
ifeq ($(OSTYPE),Linux)
	TARGET := x86_64-unknown-linux-musl
else ifeq ($(OSTYPE),Darwin)
	TARGET := x86_64-apple-darwin
else
$(error "Unsupported OSTYPE: $(OSTYPE)")
endif

VERSION := $(word 3,$(shell grep -m1 "^version" Cargo.toml))
RELEASE := dust-$(VERSION)-$(shell echo $(OSTYPE) | tr "[:upper:]" "[:lower:]")

all: release

dust:
	cargo build --locked --release --target=$(TARGET)

bin:
	mkdir -p $@

bin/dust: dust bin
	cp -f target/$(TARGET)/release/dust $@

release: bin/dust
	tar -C bin -Jcvf $(RELEASE).tar.xz dust

.PHONY: all release
