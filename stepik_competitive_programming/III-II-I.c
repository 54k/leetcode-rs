#include <stdio.h>

#define ll long long

int main() {
    ll n;
    scanf("%d", &n);
    ll arr[n + 1];
    for (int i = 1; i <= n; ++i) {
        scanf("%lld", &arr[i]);
    }

    ll dp[n + 1];
    dp[0] = 0;
    dp[1] = arr[1];
    for (int i = 2; i <= n; ++i) {
        dp[i] = (dp[i - 1] > dp[i - 2] ? dp[i - 1] : dp[i - 2]) + arr[i];
    }

    printf("%lld", dp[n]);
    return 0;
}
