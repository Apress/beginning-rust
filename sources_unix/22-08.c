// It has undefined behavior. Possibly it prints a number.
#include <stdio.h>
#include <stdlib.h>
int main() {
    int* v = malloc(1 * sizeof (int));
    v[0] = 12;
    const int* ref_to_first = &v[0];
    v = realloc(v, 2 * sizeof (int));
    v[1] = 13;
    printf("%d", *ref_to_first);
    free(v);
}
