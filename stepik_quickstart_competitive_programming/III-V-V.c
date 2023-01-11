#include <stdio.h>

#define max(x, y) (x > y ? x : y)

long INF = -10000000007;

int w[301];
int s[301];
long dp[301][10001];

int main() {
    int N, S;
    scanf("%d %d", &S, &N);
    for (int i = 1; i <= N; ++i) {
        scanf("%d", &w[i]);
    }
    for (int i = 1; i <= N; ++i) {
        scanf("%d", &s[i]);
    }
    for (int i = 0; i <= S; ++i) {
        dp[0][i] = INF;
    }
    dp[0][0] = 0;

    for (int i = 1; i <= N; ++i) {
        for (int j = 0; j <= S; ++j) {
            if (w[i] <= j) {
                dp[i][j] = max(dp[i - 1][j], dp[i - 1][j - w[i]] + s[i]);
            } else {
                dp[i][j] = dp[i - 1][j];
            }
        }
    }

    long ans = 0;
    for (int i = 0; i <= S; ++i) {
        ans = max(ans, dp[N][i]);
    }
    printf("%ld", ans);
    return 0;
}