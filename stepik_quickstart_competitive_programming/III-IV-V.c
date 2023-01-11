#include <stdio.h>

#define ll long long

#define min(x, y) (x <= y ? x : y)

#define minp(x, y) (x->min < y->min ? x : y)

struct P {
    ll val;
    ll min;
    int x;
    int y;
};

struct DP {
    ll val;
    int from_x;
    int from_y;
};

struct xy {
    int x;
    int y;
};

static ll arr[1000001];
static struct P pc[1000001];
static struct P pr[1000001];
static struct DP dp[1000001];
static struct xy ans[1000001];

int to_idx(int x, int m, int y) {
    return x * m + y;
}

int main() {
    int n, m;
    scanf("%d", &n);
    scanf("%d", &m);

    for (int i = 0; i < n; ++i) {
        for (int j = 0; j < m; ++j) {
            int idx = to_idx(i, m, j);
            scanf("%lld", &arr[idx]);

            pr[idx].val = arr[idx];
            pr[idx].min = arr[idx];
            pr[idx].x = i;
            pr[idx].y = j;

            pc[idx].val = arr[idx];
            pc[idx].min = arr[idx];
            pc[idx].x = i;
            pc[idx].y = j;
        }
    }

    pr[0].min = arr[0];
    pc[0].min = arr[0];
    dp[0].val = arr[0];
    dp[0].from_x = 0;
    dp[0].from_y = 0;

    for (int i = 1; i < n; ++i) {
        int idx = to_idx(i, m, 0);
        int top_idx = to_idx(i - 1, m, 0);
        pr[idx].val += pr[top_idx].min;
        pr[idx].min = min(pr[top_idx].min, pr[idx].val);

        if (pr[top_idx].min <= pr[idx].val) {
            pr[idx].x = pr[top_idx].x;
            pr[idx].y = pr[top_idx].y;
        }
        pc[idx].val = pr[idx].val;
        pc[idx].min = pr[idx].val;

        dp[idx].val = pr[idx].val;
        dp[idx].from_x = pr[top_idx].x;
        dp[idx].from_y = pr[top_idx].y;
    }

    for (int i = 1; i < m; ++i) {
        pc[i].val += pc[i - 1].min;
        pc[i].min = min(pc[i - 1].min, pc[i].val);

        if (pc[i - 1].min <= pc[i].val) {
            pc[i].x = pc[i - 1].x;
            pc[i].y = pc[i - 1].y;
        }
        pr[i].val = pc[i].val;
        pr[i].min = pc[i].val;

        dp[i].val = pc[i].val;
        dp[i].from_x = pc[i - 1].x;
        dp[i].from_y = pc[i - 1].y;

    }

    for (int i = 1; i < n; ++i) {
        for (int j = 1; j < m; ++j) {
            int top_idx = to_idx(i - 1, m, j);
            struct P *top = &pr[top_idx];
            int left_idx = to_idx(i, m, j - 1);
            struct P *left = &pc[left_idx];
            struct P *min_pref = minp(top, left);

            int idx = to_idx(i, m, j);

            dp[idx].val = arr[idx] + min_pref->min;

            dp[idx].from_x = min_pref->x;
            dp[idx].from_y = min_pref->y;

            pr[idx].val = dp[idx].val;
            pr[idx].min = min(pr[top_idx].min, pr[idx].val);

            if (pr[top_idx].min <= pr[idx].val) {
                pr[idx].x = pr[top_idx].x;
                pr[idx].y = pr[top_idx].y;
            }

            pc[idx].val = dp[idx].val;
            pc[idx].min = min(pc[left_idx].min, pc[idx].val);

            if (pc[left_idx].min <= pc[idx].val) {
                pc[idx].x = pc[left_idx].x;
                pc[idx].y = pc[left_idx].y;
            }
        }
    }

//    for (int i = 0; i < n; ++i) {
//        for (int j = 0; j < m; ++j) {
//            if (dp[to_idx(i, m, j)].val >= 0) printf("+%lld ", dp[to_idx(i, m, j)].val);
//            if (dp[to_idx(i, m, j)].val < 0) printf("%lld ", dp[to_idx(i, m, j)].val);
//        }
//        printf("\n");
//    }

    int k = 0;
    int fx = n - 1, fy = m - 1;

    while (fx >= 0 || fy >= 0) {
        ans[k].x = fx;
        ans[k].y = fy;

        struct DP d = dp[to_idx(fx, m, fy)];
        ++k;

        if (d.from_x == fx && d.from_y == fy) {
            break;
        }

        fx = d.from_x;
        fy = d.from_y;
    }

    printf("%lld %d\n", dp[to_idx(n - 1, m, m - 1)].val, k);

    while (--k >= 0) {
        struct xy d = ans[k];
        printf("%d %d\n", d.x + 1, d.y + 1);
    }

    return 0;
}