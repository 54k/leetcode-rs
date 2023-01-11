#include<stdio.h>

#define ll long long

#define N (1024*1024)

ll dp[N];

int left, top[N], diag[N], path[N];

int main() {
    int n, m;
    scanf("%d %d", &n, &m);

    ll cell;
    for (int i = 0; i < n; ++i) {
        for (int j = 0; j < m; ++j) {
            scanf("%lld", &cell);

            int k = i * m + j;
            int x = n + (j - i);

            if (i == 0 && j == 0) {
                dp[0] = cell;
                continue;
            } else if (i == 0) {
                dp[k] = dp[left] + cell;
                top[j] = k;
                diag[x] = k;

                path[k] = left;
            } else if (j == 0) {
                dp[k] = dp[top[j]] + cell;
                left = k;
                diag[x] = k;

                path[k] = top[j];
            } else {
                int lt = dp[top[j]] < dp[left] ? top[j] : left;
                int ltd = dp[lt] < dp[diag[x]] ? lt : diag[x];

                dp[k] = dp[ltd] + cell;
                path[k] = ltd;
            }

            top[j] = dp[top[j]] < dp[k] ? top[j] : k;
            left = dp[left] < dp[k] ? left : k;
            diag[x] = dp[diag[x]] < dp[k] ? diag[x] : k;
        }
    }

    int k = 1, parent = n * m - 1;
    for (top[0] = parent; parent; k++) top[k] = parent = path[parent];
    printf("%lld %d\n", dp[n * m - 1], k);

    for (k--; k >= 0; parent = top[--k])
        printf("%d %d\n", parent / m + 1, parent % m + 1);

    return 0;
}