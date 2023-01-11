#include <stdio.h>

int sieve[1000001];

long factors(int n) {
    long ans = 0;
    for (int i = 2; i <= n; ++i) {
        if (sieve[i]) continue;
        for (int j = 2 * i; j <= n; j += i) {
            if (sieve[j]) continue;
            sieve[j] = 1;
            ans += i;
        }
    }
    return ans;
}

int main() {
    int n;
    scanf("%d", &n);
    printf("%ld\n", factors(n));
    return 0;
}