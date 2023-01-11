#include <stdio.h>

int M = 1000000123;

int dp[1001][1001];

int mod(int x) {
    return ((x % M) + M) % M;
}

int mod_add(int x, int y) {
    return mod(mod(x) + mod(y));
}

int dfs(int x, int y, int n, int m) {
    if (x < 0 || x >= n || y < 0 || y >= m) {
        return 0;
    }

    if (x == n - 1 && y == m - 1) {
        return 1;
    }

    if (dp[x][y] > -1) {
        return dp[x][y];
    }

    // x + 1, y + 2
    // x - 1, y + 2
    // x + 2, y + 1
    // x + 2, y - 1
    int a = dfs(x + 1, y + 2, n, m);
    int b = dfs(x - 1, y + 2, n, m);
    int c = dfs(x + 2, y + 1, n, m);
    int d = dfs(x + 2, y - 1, n, m);

    return dp[x][y] = mod_add(mod_add(a, b), mod_add(c, d));
}

int main() {
    int n, m;
    scanf("%d", &n);
    scanf("%d", &m);

    for (int i = 0; i < n; ++i) {
        for (int j = 0; j < m; ++j) {
            dp[i][j] = -1;
        }
    }

    printf("%d", dfs(0, 0, n, m));
    return 0;
}
