#include <stdio.h>

#define ll long long

int main() {
    int n;
    scanf("%d", &n);
    ll dp[n + 1];
    dp[0] = 1;
    dp[1] = 3;
    for (int i = 2; i <= n; ++i) {
        dp[i] = dp[i - 1] * 3 - dp[i - 2];
    }
    printf("%lld", dp[n]);
    return 0;
}
