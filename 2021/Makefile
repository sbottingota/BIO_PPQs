.PHONY: all clean

CXX=g++
CPPFLAGS=-Wall -Wextra -pg -g

# RUSTC=rustc
RUSTC=clippy-driver --rustc
RUSTFLAGS=-C opt-level=3

all: q1a q1b q2a q3a q3b

q1a: q1.h q1.cc q1a.cc
q1b: q1.h q1.cc q1b.cc
q2a: q2.rs q2a.rs
q3a: q3.h q3.cc q3a.cc
q3b: q3.h q3.cc q3b.cc

q1a q1b q3a q3b:
	$(CXX) $(CPPFLAGS) $@.cc $$(echo $@ | cut -c 1-2).cc -o $@

q2a:
	$(RUSTC) $(RUSTFLAGS) $@.rs


clean:
	rm -f q1a q1b q2a q3a q3b
