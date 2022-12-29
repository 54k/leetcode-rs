#include <stdio.h>

int dp[301][10001];

int main() {
    int S, N, wi;
    scanf("%d %d", &S, &N);

    dp[0][0] = 1;

    for (int i = 1; i <= N; ++i) {
        scanf("%d", &wi);
        for (int j = 0; j <= S; ++j) {
            if (wi <= j) {
                dp[i][j] = dp[i - 1][j] | dp[i - 1][j - wi];
            } else {
                dp[i][j] = dp[i - 1][j];
            }
        }
    }

    for (int i = S; i >= 0; --i) {
        if (dp[N][i]) {
            printf("%d", i);
            return 0;
        }
    }

    return 0;
}