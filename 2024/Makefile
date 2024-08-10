CC=gcc
CXX=g++
CFLAGS=-Wall -Wextra

all: q1a q3a

clean:
	rm -f q1a q3a

# terminates with buffer overflow if I enable optimizer :P
q1a: q1a.c
	$(CC) $(CFLAGS) q1a.c -o q1a -lm

q3a: q3.cc q3a.cc q3.h
	$(CXX) $(CFLAGS) -O q3.cc q3a.cc -o q3a
