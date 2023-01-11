#include <stdio.h>

#define N 501

int seen[N];

int n, m;

int matrix[N][N];

void dfs(int v) {
    seen[v] = 1;
    for (int i = 1; i <= n; ++i) {
        if (matrix[v][i] > 0 && !seen[i]) {
            printf("%d ", matrix[v][i]);
            printf("%d %d\n", v, i);
            dfs(i);
        }
    }
}

int main() {
    int u, v;
    scanf("%d %d", &n, &m);
    for (int i = 1; i <= m; ++i) {
        scanf("%d %d", &v, &u);
        matrix[v][u] = i;
        matrix[u][v] = i;
    }
    printf("%d\n", n - 1);
    dfs(1);
    return 0;
}