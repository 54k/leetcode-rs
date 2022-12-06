#include <stdio.h>

#define ull unsigned long long
#define ULONG_LONG_MAX 18446744073709551615ULL

// При n%3 = 0: n/3
// При n%3 = 1: (2^64 - 1)/3 * 2 + n/3 + 1
// При n%3 = 2: (2^64 - 1)/3 + n/3 + 1
ull solve(ull n) {
    switch (n % 3) {
        case 0:
            return n / 3;
        case 1:
            return ULONG_LONG_MAX / 3 * 2 + n / 3 + 1;
        case 2:
            return ULONG_LONG_MAX / 3 + n / 3 + 1;
        default:
            return 0;
    }
}

int main() {
    ull n;
    scanf("%llu", &n);
    printf("%llu\n", solve(n));
    return 0;
}