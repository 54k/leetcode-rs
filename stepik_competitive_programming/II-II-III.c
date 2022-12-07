#include <stdio.h>

long long mod = 1000003;

long long norm(long long d) {
    return ((d % mod) + mod) % mod;
}

long long mul(long long a, long long b) {
    return norm(norm(a) * norm(b));
}

int main() {
    long long n;
    scanf("%lld", &n);
    long long res = 1;
    if (n >= mod) {
        printf("0");
        return 0;
    }
    for (long long i = 2; i <= n; ++i) {
        res = mul(i, res);
    }
    printf("%lld", res);
    return 0;
}