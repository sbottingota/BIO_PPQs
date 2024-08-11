CC=gcc
CXX=g++
CFLAGS=-Wall -Wextra

all: q1a q1b q2a q2b q3a q3b q3c

clean:
	rm -f q1a q1b q2a q2b q3a q3b q3c

# terminates with buffer overflow if I enable optimizer :P
q1a: q1.c q1a.c q1.h
	$(CC) $(CFLAGS) q1.c q1a.c -o q1a -lm

# same
q1b: q1.c q1b.c q1.h
	$(CC) $(CFLAGS) q1.c q1b.c -o q1b -lm

q2a: q2.cc q2a.cc q2.h
	$(CXX) $(CFLAGS) -O q2.cc q2a.cc -o q2a

q2b: q2.cc q2b.cc q2.h
	$(CXX) $(CFLAGS) -O q2.cc q2b.cc -o q2b

q3a: q3.cc q3a.cc q3.h
	$(CXX) $(CFLAGS) -O q3.cc q3a.cc -o q3a

q3b: q3.cc q3b.cc q3.h
	$(CXX) $(CFLAGS) -O q3.cc q3b.cc -o q3b

q3c: q3.cc q3c.cc q3.h
	$(CXX) $(CFLAGS) -O q3.cc q3c.cc -o q3c
