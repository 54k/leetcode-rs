#include <stdio.h>

#define ll long long
#define min(x, y) (x->sum <= y->sum ? x : y)

static struct DP {
    ll sum;
    ll path;
    int x;
    int y;
    int from_x;
    int from_y;
    int calculated;
} dp[1001][1001];

static struct DP INF = {
        .sum = 1000000000000 + 1,
        .path = 0,
        .x = -1,
        .y = -1,
        .from_x = -1,
        .from_y = -1,
        .calculated = 1,
};

struct DP *dfs(int x, int y, int n, int m) {
    if (x < 0 || x >= n || y < 0 || y >= m) {
        return &INF;
    }

    if (x == n - 1 && y == m - 1) {
        return &dp[x][y];
    }

    if (dp[x][y].calculated == 1) {
        return &dp[x][y];
    }

    // x + 1, y + 2
    // x - 1, y + 2
    // x + 2, y + 1
    // x + 2, y - 1
    struct DP *a = dfs(x + 1, y + 2, n, m);
    struct DP *b = dfs(x - 1, y + 2, n, m);
    struct DP *c = dfs(x + 2, y + 1, n, m);
    struct DP *d = dfs(x + 2, y - 1, n, m);

    struct DP *f = min(min(a, b), min(c, d));

    if (f->sum >= INF.sum) {
        dp[x][y].sum = INF.sum;
        dp[x][y].path = INF.sum;
    } else {
        dp[x][y].sum += f->sum;
        dp[x][y].path += f->path;
        dp[x][y].from_x = f->x;
        dp[x][y].from_y = f->y;
    }
    dp[x][y].calculated = 1;
    return &dp[x][y];
}

int main() {
    int n, m;
    scanf("%d", &n);
    scanf("%d", &m);

    for (int i = 0; i < n; ++i) {
        for (int j = 0; j < m; ++j) {
            scanf("%lld", &dp[i][j].sum);
            dp[i][j].path = 1;
            dp[i][j].x = i;
            dp[i][j].y = j;
            dp[i][j].from_x = i;
            dp[i][j].from_y = j;
            dp[i][j].calculated = 0;
        }
    }

    struct DP *p = dfs(0, 0, n, m);

    if (p->sum == INF.sum) {
        printf("NO");
        return 0;
    } else {
        printf("YES\n");
        printf("%lld %lld\n", p->sum, p->path);
    }

    while (p->from_x != p->x || p->from_y != p->y) {
        printf("%d %d\n", p->x + 1, p->y + 1);
        p = &dp[p->from_x][p->from_y];
    }
    printf("%d %d\n", p->x + 1, p->y + 1);

//    for (int i = 0; i < n; ++i) {
//        for (int j = 0; j < m; ++j) {
//            printf("%lld ", dp[i][j].sum);
//        }
//        printf("\n");
//    }

    return 0;
}
