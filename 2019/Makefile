# RUSTC=rustc
RUSTC=clippy-driver --rustc
RUSTFLAGS=-C opt-level=3

.PHONY: all clear

all: q1a q3a

q1a: q1a.rs q1.rs
	$(RUSTC) $(RUSTFLAGS) $@.rs

q3a: q3a.rs q3.rs
	$(RUSTC) $(RUSTFLAGS) $@.rs


