#include <stdio.h>

#define min(x, y) (x.sum <= y.sum ? x : y)
#define ll long long

ll arr[1001][1001];

struct DP {
    ll sum;
    int steps;
} dp[1001][1001];

struct C {
    int x;
    int y;
} ans[1000500];

int main() {
    int n, m;
    scanf("%d", &n);
    scanf("%d", &m);

    for (int i = 0; i <= n; ++i) {
        for (int j = 0; j <= m; ++j) {
            if (i * j == 0) {
                dp[i][j].sum = 1000000007;
                dp[i][j].steps = 0;
                arr[i][j] = dp[i][j].sum;
            } else {
                scanf("%lld", &arr[i][j]);
                dp[i][j].sum = arr[i][j];
                dp[i][j].steps = 1;
            }
        }
    }

    for (int i = 1; i <= n; ++i) {
        for (int j = 1; j <= m; ++j) {
            if (i * j == 1) continue;
            struct DP dp_min = min(min(dp[i - 1][j], dp[i][j - 1]), dp[i - 1][j - 1]);
            dp[i][j].sum += dp_min.sum;
            dp[i][j].steps += dp_min.steps;
        }
    }

    printf("%lld %d\n", dp[n][m].sum, dp[n][m].steps);
    int steps = dp[n][m].steps;
    int j = steps;
    int x = n;
    int y = m;

    while (x > 0 && y > 0) {
        ans[j - 1].x = x;
        ans[j - 1].y = y;
        j--;

        if (arr[x][y] + dp[x - 1][y].sum == dp[x][y].sum) {
            x--;
        } else if (arr[x][y] + dp[x][y - 1].sum == dp[x][y].sum) {
            y--;
        } else {
            x--;
            y--;
        }
    }

    for (int i = 0; i < steps; ++i) {
        if (i < steps - 1) {
            printf("%d %d\n", ans[i].x, ans[i].y);
        } else {
            printf("%d %d", ans[i].x, ans[i].y);
        }
    }

    return 0;
}