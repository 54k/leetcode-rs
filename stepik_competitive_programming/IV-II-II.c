#include <stdio.h>

#define N 20001

int adj[N][N];
int seen[N];
int cmp[N];

int n, m, s;

void dfs(int v, int c) {
    seen[v] = 1;
    cmp[v] = c;
    for (int i = 1; i <= adj[v][0]; ++i) {
        int u = adj[v][i];
        if (!seen[u]) {
            dfs(u, c);
        }
    }
}

int main() {
    int u, v;
    scanf("%d %d", &n, &m);
    for (int i = 1; i <= m; ++i) {
        scanf("%d %d", &u, &v);
        adj[u][++adj[u][0]] = v;
        adj[v][++adj[v][0]] = u;
    }

    int c = 1;
    for (int i = 1; i <= n; ++i) {
        if (!seen[i]) {
            dfs(i, c++);
        }
    }

    printf("%d\n", c - 1);
    for (int i = 1; i <= n; ++i) {
        printf("%d ", cmp[i]);
    }

    return 0;
}