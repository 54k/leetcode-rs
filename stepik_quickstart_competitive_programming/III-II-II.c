#include <stdio.h>

#define ll long long
#define max(x, y) (x > y ? x : y)

int main() {
    int n;
    scanf("%d", &n);
    ll arr[n];
    ll dp[n + 1];
    dp[0] = 0;

    for (int i = 0; i < n; ++i) {
        scanf("%lld", &arr[i]);
        dp[i + 1] = 0;
    }

    for (int i = 1; i <= n; ++i) {
        if (i <= 1) {
            dp[i] = arr[i - 1];
        } else if (i < 3) {
            dp[i] = dp[i - 1] + arr[i - 1];
        } else if (i <= 4) {
            dp[i] = max(dp[i - 1], dp[i - 3]) + arr[i - 1];
        } else {
            dp[i] = max(max(dp[i - 1], dp[i - 3]), dp[i - 5]) + arr[i - 1];
        }
    }

    printf("%lld", dp[n]);
    return 0;
}
