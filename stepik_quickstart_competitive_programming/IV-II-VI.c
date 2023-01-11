#include <stdio.h>

#define N 501

int n, m;
int adj[N][N];

int visited[N];

void dfs(int v) {
    visited[v] = 1;
    for (int i = 1; i <= n; ++i) {
        int u = adj[v][i] > 0 ? i : 0;
        if (!u || v == u) continue;
        if (visited[u] == 0) {
            adj[v][u] = 0;
            adj[u][v] = 0;
            dfs(u);
        }
    }
}

int main() {
    scanf("%d %d", &n, &m);
    for (int i = 1; i < m; ++i) {
        int v, u;
        scanf("%d %d", &u, &v);
        adj[v][u] = 1;
        adj[u][v] = 1;
    }

    for (int i = 1; i <= n; ++i) {
        if (!visited[i]) dfs(i);
    }

    for (int i = 1; i <= n; ++i) {
        for (int j = 1; j <= n; ++j) {
            if (adj[i][j]) {
                printf("YES");
                return 0;
            }
        }
    }
    printf("NO");
    return 0;
}
