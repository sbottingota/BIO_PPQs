#include <stdio.h>
#include <stdlib.h>

#include "q1.h"


// find the largest fibonacci number that is less than or equal to n
int find_largest_le_fib_n(int n) {
    if (n == 0 || n == 1) {
       return n;
    }

    int f1 = 0, f2 = 1, f3 = 1;

    while (f3 <= n) {
        f1 = f2;
        f2 = f3;
        f3 = f1 + f2;
    }

    return f2;
}

// find the zeckendorf representation of a number
// returns an int list
Node* find_zeckendorf(int n) {
    Node *last;
    Node *first;
    Node *penultimate = NULL;

    int largest_le_fib;

    while (n > 0) {
        largest_le_fib = find_largest_le_fib_n(n);

        last = malloc(sizeof(Node));        
        last->data = largest_le_fib;
        last->next = NULL;

        if (first == NULL) {
            first = last;
        }
        
        if (penultimate != NULL) {
            penultimate->next = last;
        }

        penultimate = last;

        n -= largest_le_fib;
    }

    return first;
}

