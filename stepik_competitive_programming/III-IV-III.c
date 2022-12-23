#include <stdio.h>

#define ll long long

#define min(x, y) (x.path_length > y.path_length ? y : x)

static struct DP {
    ll path_length;
    int idx;
    int from_idx;
} dp[1000500];

static struct DP INF = {
        .path_length = 1000007,
        .idx = 0,
        .from_idx = 0
};

int main() {
    int n;
    scanf("%d", &n);

    for (int i = 0; i < 2; ++i) {
        dp[i].path_length = 0;
        dp[i].from_idx = 0;
        dp[i].idx = i;
    }

    for (int i = 2; i <= n; ++i) {
        struct DP a = INF, b = INF, c;

        if (i % 3 == 0) {
            a = dp[i / 3];
        }
        if (i % 2 == 0) {
            b = dp[i / 2];
        }
        c = dp[i - 1];

        struct DP d = min(min(a, b), c);

        dp[i].path_length = 1 + d.path_length;
        dp[i].from_idx = d.idx;
        dp[i].idx = i;
    }

    ll ans_len = dp[n].path_length;

    printf("%lld\n", ans_len);

    ll ans[ans_len];

    int j = 0;
    struct DP *p = &dp[n];
    while (p->idx != 0) {
        ans[j] = p->idx;
        p = &dp[p->from_idx];
        j++;
    }
    while (j-- > 0) {
        printf("%lld ", ans[j]);
    }

    return 0;
}