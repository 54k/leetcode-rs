#include <stdio.h>

#define ll long long

int main() {
    int n;
    scanf("%d", &n);
    if (n == 0) {
        printf("1");
        return 0;
    }
    ll dp[n + 1];
    dp[0] = 1;
    dp[1] = 2;
    for (int i = 2; i <= n; ++i) {
        dp[i] = dp[i - 1] + dp[i - 2];
    }
    printf("%lld", dp[n]);
    return 0;
}
