/* It prints:
Result: 23.67*/
#include <stdio.h>
int main() {
    enum eResult {
        Success,
        Failure,
        Uncertainty
    };

    struct sResult {
        enum eResult r;
        union {
            double value;
            struct {
                unsigned short error_code;
                char module;
            } s;
        } u;
    } outcome;
    outcome.r = Success;
    outcome.u.value = 23.67;
    switch (outcome.r) {
        case Success:
            printf("Result: %g", outcome.u.value);
            break;
        case Failure:
            printf("Error n. %d in module %c",
                outcome.u.s.error_code,
                outcome.u.s.module);
            break;
        case Uncertainty:
            break;
    }
    return 0;
}
