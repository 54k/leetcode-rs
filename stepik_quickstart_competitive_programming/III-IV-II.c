#include <stdio.h>

#define ll long long

#define max(x, y) (x.sum >= y.sum ? x : y)

static struct DP {
    ll sum;
    ll path_length;
    int idx;
    int from_idx;
} dp[100500];

int main() {
    int n;
    scanf("%d", &n);

    for (int i = 1; i <= n; ++i) {
        struct DP *d = &dp[i];
        scanf("%lld", &d->sum);
        d->path_length = 1;
        d->idx = i;
        d->from_idx = 0;
    }

    for (int i = 1; i <= n; ++i) {
        if (i <= 1) {
            continue;
        }

        if (i < 3) {
            struct DP *d = &dp[i];
            d->sum += dp[i - 1].sum;
            d->path_length += dp[i - 1].path_length;
            d->from_idx = dp[i - 1].idx;
        } else if (i <= 4) {
            struct DP m = max(dp[i - 1], dp[i - 3]);
            struct DP *d = &dp[i];
            d->sum += m.sum;
            d->path_length += m.path_length;
            d->from_idx = m.idx;
        } else {
            struct DP m = max(max(dp[i - 1], dp[i - 3]), dp[i - 5]);
            struct DP *d = &dp[i];
            d->sum += m.sum;
            d->path_length += m.path_length;
            d->from_idx = m.idx;
        }
    }

    ll ans_len = dp[n].path_length;

    printf("%lld %lld\n", dp[n].sum, ans_len);

    int ans[ans_len];

    int j = 0;
    struct DP *p = &dp[n];
    while (p->idx != 0) {
        ans[j] = p->idx;
        p = &dp[p->from_idx];
        j++;
    }
    while (--j >= 0) {
        printf("%d ", ans[j]);
    }

    return 0;
}