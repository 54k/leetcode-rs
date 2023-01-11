#include <stdio.h>

int MOD = 1000003;

int main(void) {
    int n;
    scanf("%d", &n);
    int fib1 = 1, fib2 = 1, fib3 = 1;
    for (int i = 2; i <= n; ++i) {
        fib3 = fib1 % MOD + fib2 % MOD;
        fib1 = fib2;
        fib2 = fib3;
    }
    printf("%d", fib3 % MOD);
    return 0;
}
