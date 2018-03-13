/* It prints:
1 2 :2 */
#include <stdio.h>
int main() {
    int limit = 4;
    for (int i = 1; i < limit; i++) {
        limit -= 1;
        printf("%d ", i);
    }
    printf(":%d ", limit);
    return 0;
}
