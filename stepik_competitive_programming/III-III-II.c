#include <stdio.h>

#define min(x, y) (x < y ? x : y)
#define abs(x) (x < 0 ? -(x) : x)

int main() {
    int n, m;
    scanf("%d", &m);
    scanf("%d", &n);
    int arr[n][m];
    int dp[n][m];

    for (int i = 0; i < n; ++i) {
        for (int j = 0; j < m; ++j) {
            dp[i][j] = 0;
            scanf("%d", &arr[i][j]);
        }
    }

    for (int i = 1; i < n; ++i) {
        dp[i][0] = dp[i - 1][0] + abs(arr[i][0] - arr[i - 1][0]);
    }

    for (int i = 1; i < m; ++i) {
        dp[0][i] = dp[0][i - 1] + abs(arr[0][i] - arr[0][i - 1]);
    }

    for (int i = 1; i < n; ++i) {
        for (int j = 1; j < m; ++j) {
            int h1 = dp[i - 1][j] + abs(arr[i][j] - arr[i - 1][j]);
            int h2 = dp[i][j - 1] + abs(arr[i][j] - arr[i][j - 1]);
            dp[i][j] = min(h1, h2);
        }
    }

    printf("%d", dp[n - 1][m - 1]);
    return 0;
}
