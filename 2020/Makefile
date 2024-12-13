RUSTC=clippy-driver --rustc 
RUSTFLAGS=-C opt-level=3

.PHONY: clean all

all: q1a q2a q3a

q1a: q1.rs q1a.rs
	$(RUSTC) q1a.rs $(RUSTFLAGS)

q2a: q2a.rs q2.rs
	$(RUSTC) q2a.rs $(RUSTFLAGS)

q3a: q3.rs q3a.rs
	$(RUSTC) q3a.rs $(RUSTFLAGS)

clean:
	rm -f q1a q2a q3a

