#include <stdio.h>

int M = 1000000033;

int pc[1501][1501]; // prefix columns
int pr[1501][1501]; // prefix rows

int dp[1501][1501];

int mod(int x) {
    return ((x % M) + M) % M;
}

int mod_add(int x, int y) {
    return mod(mod(x) + mod(y));
}

int mod_mul(int x, int y) {
    return mod(mod(x) * mod(y));
}

int main() {
    int n, m;
    scanf("%d", &n);
    scanf("%d", &m);

    for (int i = 0; i < n; ++i) {
        for (int j = 0; j < m; ++j) {
            pc[i][j] = 1;
            pr[i][j] = 1;
        }
    }

    dp[0][0] = 1;
    dp[0][1] = 1;
    dp[1][0] = 1;

    for (int i = 2; i < m; ++i) {
        pc[0][i] = mod_mul(pc[0][i - 1], 2);
        dp[0][i] = pc[0][i];
        pr[0][i] = pc[0][i];
    }

    for (int i = 2; i < n; ++i) {
        pr[i][0] = mod_mul(pr[i - 1][0], 2);
        dp[i][0] = pr[i][0];
        pc[i][0] = pr[i][0];
    }

    for (int i = 1; i < n; ++i) {
        for (int j = 1; j < m; ++j) {
            int left_pref = pc[i][j - 1];
            int top_pref = pr[i - 1][j];
            dp[i][j] = mod_add(left_pref, top_pref);

            int pc_to_add = mod_add(dp[i][j], left_pref);
            pc[i][j] = pc_to_add;

            int pr_to_add = mod_add(dp[i][j], top_pref);
            pr[i][j] = pr_to_add;

        }
    }


//    for (int i = 0; i < n; ++i) {
//        for (int j = 0; j < m; ++j) {
//            printf("%d ", dp[i][j]);
//        }
//        printf("\n");
//    }

    printf("%d", dp[n - 1][m - 1]);
    return 0;
}