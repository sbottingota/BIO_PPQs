#include <stdio.h>
#include <stdlib.h>

#include "q1.h"


int main() {
    int n = 0;

    printf("Enter number: ");
    scanf("%d", &n);

    Node *zeckendorf_list = find_zeckendorf(n);

    while (zeckendorf_list != NULL) {
        printf("%d ", zeckendorf_list->data);

        Node *temp = zeckendorf_list;
        zeckendorf_list = zeckendorf_list->next;
        free(temp);
    }

    printf("\n");
}
