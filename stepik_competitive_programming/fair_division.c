/// https://www.eolymp.com/en/problems/2449 Fair Division
#include <stdio.h>
#include <stdlib.h>

#define min(a, b) (a < b ? a : b)

struct Node {
    int val;
    int idx;
    int con;
} q[101];

int cmpval(const void *a, const void *b) {
    struct Node *an = ((struct Node *) a);
    struct Node *bn = ((struct Node *) b);
    if (an->val != bn->val) {
        return an->val - bn->val;
    }
    return an->idx < bn->idx;
}

int cmpidx(const void *a, const void *b) {
    return ((struct Node *) a)->idx > ((struct Node *) b)->idx;
}

void solve(int cost, int n) {
    qsort(q, n, sizeof(struct Node), cmpval);

    for (int i = 0; i < n; ++i) {
        q[i].con = min(q[i].val, cost / (n - i));
        cost -= q[i].con;
    }

    qsort(q, n, sizeof(struct Node), cmpidx);
}

int main() {
    int t;
    scanf("%d", &t);
    printf("\n");

    while (t--) {
        int cost;
        int n;
        scanf("%d", &cost);
        scanf("%d", &n);

        int sum = 0;
        for (int i = 0; i < n; ++i) {
            scanf("%d", &q[i].val);
            q[i].idx = i;
            sum += q[i].val;
            q[i].con = 0;
        }

        if (sum < cost) {
            printf("IMPOSSIBLE\n");
            continue;
        }

        solve(cost, n);

        for (int i = 0; i < n; ++i) {
            if (i < n - 1) {
                printf("%d ", q[i].con);
            } else {
                printf("%d\n", q[i].con);
            }
        }
    }

    return 0;
}
