#include <stdio.h>

#define ll long long

ll M = 1000000007;

ll pc[1501][1501]; // prefix columns
ll pr[1501][1501]; // prefix rows
ll pd[1501][1501]; // prefix diagonals

ll dp[1501][1501];

ll mod(ll x) {
    return ((x % M) + M) % M;
}

ll mod_add(ll x, ll y) {
    return mod(mod(x) + mod(y));
}

ll mod_mul(ll x, ll y) {
    return mod(mod(x) * mod(y));
}

int main() {
    ll n, m;
    scanf("%lld", &n);
    scanf("%lld", &m);

    for (int i = 0; i < n; ++i) {
        for (int j = 0; j < m; ++j) {
            pc[i][j] = 1;
            pr[i][j] = 1;
            pd[i][j] = 1;
        }
    }

    dp[0][0] = 1;
    dp[0][1] = 1;
    dp[1][0] = 1;

    for (int i = 2; i < m; ++i) {
        pc[0][i] = mod_mul(pc[0][i - 1], 2);
        dp[0][i] = pc[0][i];
        pr[0][i] = pc[0][i];
        pd[0][i] = pc[0][i];
    }

    for (int i = 2; i < n; ++i) {
        pr[i][0] = mod_mul(pr[i - 1][0], 2);
        dp[i][0] = pr[i][0];
        pc[i][0] = pr[i][0];
        pd[i][0] = pr[i][0];
    }

    for (int i = 1; i < n; ++i) {
        for (int j = 1; j < m; ++j) {
            ll top_pref = pr[i - 1][j];
            ll left_pref = pc[i][j - 1];
            ll diag_pref = pd[i - 1][j - 1];

            dp[i][j] = mod_add(diag_pref, mod_add(left_pref, top_pref));

            ll pc_to_add = mod_add(dp[i][j], left_pref);
            pc[i][j] = pc_to_add;

            ll pr_to_add = mod_add(dp[i][j], top_pref);
            pr[i][j] = pr_to_add;

            ll pd_to_add = mod_add(dp[i][j], diag_pref);
            pd[i][j] = pd_to_add;
        }
    }

//    for (int i = 0; i < n; ++i) {
//        for (int j = 0; j < m; ++j) {
//            printf("%lld ", dp[i][j]);
//        }
//        printf("\n");
//    }

    printf("%lld", dp[n - 1][m - 1]);
    return 0;
}