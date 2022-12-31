#include <stdio.h>

#define max(x, y) (x > y ? x : y)

int w[301];
int s[301];
long long dp[301][100001];

void print_ans(int i, int j, int d) {
    if (i == 0 && j == 0) {
        printf("%d\n", d);
        return;
    }
    if (dp[i - 1][j] == dp[i][j]) {
        print_ans(i - 1, j, d);
    } else {
        print_ans(i - 1, j - w[i], d + 1);
        printf("%d ", i);
    }
}

int main() {
    int N, S;
    scanf("%d %d", &S, &N);
    for (int i = 1; i <= N; ++i) {
        scanf("%d", &w[i]);
    }
    for (int i = 1; i <= N; ++i) {
        scanf("%d", &s[i]);
    }

    for (int i = 0; i <= N; ++i) {
        dp[i][0] = 0;
    }
    for (int i = 0; i <= S; ++i) {
        dp[0][i] = 0;
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

    long long ans = 0;
    int maxS = 0;
    for (int i = 0; i <= S; ++i) {
        if (dp[N][i] > ans) {
            ans = dp[N][i];
            maxS = i;
        }
    }

    printf("%lld ", ans);
    print_ans(N, maxS, 0);
    return 0;
}
