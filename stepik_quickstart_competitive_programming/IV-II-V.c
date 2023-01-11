#include <stdio.h>

int G[501][501];
int T[250000];

int n, m, q, k;

void dfs(int u) {
    G[0][u] = 1;
    for (int i = 1; i <= n; ++i) {
        if (!G[0][i] && G[u][i] && T[G[u][i]]) {
            k++;
            dfs(i);
        }
    }
    for (int i = 1; i <= n; ++i) {
        if (!G[0][i] && G[u][i]) {
            k = -7;
            dfs(i);
        }
    }
}

int main() {
    scanf("%d %d", &n, &m);
    for (int i = 1; i <= m; ++i) {
        int u, v;
        scanf("%d %d", &u, &v);
        G[u][v] = i;
        G[v][u] = i;
    }

    scanf("%d", &q);
    for (int i = 1; i <= q; ++i) {
        int qi;
        scanf("%d", &qi);
        T[qi] = 1;
    }

    for (int i = 1; i <= n; ++i) {
        for (int j = 1; j <= n; ++j) {
            G[0][j] = 0;
        }
        k = 0;
        dfs(i);
        if (q == k) {
            printf("YES");
            return 0;
        }
    }
    printf("NO");
    return 0;
}