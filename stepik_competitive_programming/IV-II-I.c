#include <stdio.h>

#define N 101

int adj[N][N];
int seen[N];

int n, m, s;

int pi;
int path[N * 1000];

void dfs(int v) {
    seen[v] = 1;
    for (int i = 1; i <= adj[v][0]; ++i) {
        int u = adj[v][i];
        if (!seen[u]) {
            path[pi++] = v;
            dfs(u);
        }
    }
    path[pi++] = v;
}

int main() {
    int u, v;
    scanf("%d %d", &n, &m);
    for (int i = 1; i <= m; ++i) {
        scanf("%d %d", &u, &v);
        adj[u][++adj[u][0]] = v;
        adj[v][++adj[v][0]] = u;
    }
    scanf("%d", &s);
    dfs(s);
    printf("%d\n", pi);
    for (int i = 0; i < pi; ++i) {
        printf("%d ", path[i]);
    }
    return 0;
}