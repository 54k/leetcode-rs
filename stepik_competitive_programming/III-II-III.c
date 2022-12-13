#include <stdio.h>

#define ll long long
#define min(x, y) (x > y ? y : x)

int INF = 1000007;

int main() {
    int n;
    scanf("%d", &n);

    int dp[n + 1];
    dp[1] = 0;

    for (int i = 2; i <= n; ++i) {
        int a = INF, b = INF, c;
        if (i % 3 == 0) {
            a = dp[i / 3];
        }
        if (i % 2 == 0) {
            b = dp[i / 2];
        }
        c = dp[i - 1];
        dp[i] = 1 + min(min(a, b), c);
    }

    printf("%d", dp[n]);

    return 0;
}