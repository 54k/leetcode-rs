#include <stdio.h>

#define N 501

int adj[N][N];
int seen[N];

int n, m;

void dfs(int v) {
    seen[v] = 1;
    for (int i = 1; i <= adj[v][0]; ++i) {
        int u = adj[v][i];
        if (!seen[u]) {
            dfs(u);
        }
    }
    m--;
}

int main() {
    int u, v;
    scanf("%d %d", &n, &m);
    for (int i = 1; i <= m; ++i) {
        scanf("%d %d", &u, &v);
        adj[u][++adj[u][0]] = v;
        adj[v][++adj[v][0]] = u;
    }
    dfs(1);
    printf("%d\n", m + 1);
    return 0;
}

//#include <stdio.h>
//
//int n, m;
//
//int main() {
//    int u, v;
//    scanf("%d %d", &n, &m);
//    printf("%d\n", m - (n - 1));
//    return 0;
//}