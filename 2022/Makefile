CFLAGS=-Wall -Wextra -pg -g

all: q1a q1b q3a

clean:
	rm -f q[123][abcd]

q1a: q1.h q1.c q1a.c
	gcc q1.c q1a.c -o q1a $(CFLAGS)

q1b: q1.h q1.c q1a.c
	gcc q1.c q1b.c -o q1b $(CFLAGS)

q3a: q3.h q3.cc q3a.cc
	g++ q3.cc q3a.cc -o q3a $(CFLAGS)
