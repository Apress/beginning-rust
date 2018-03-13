/* It prints:
12
and then it has undefined behavior*/
#include <stdio.h>
int main() {
    int* ref_to_n;
    {
        int n = 12;
        ref_to_n = &n;
        printf("%d ", *ref_to_n);
    }
    printf("%d", *ref_to_n);

    return 0;
}
