all: q1a q1b

clean:
	rm -f q[123][abcd]

q1a: q1.h q1.c q1a.c
	gcc q1.c q1a.c -o q1a

q1b: q1.h q1.c q1a.c
	gcc q1.c q1b.c -o q1b
