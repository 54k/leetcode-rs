#include <stdio.h>
#include <stdlib.h>

#define N 501

int adj[N][N];

int n, m, pi;

void dfs(int u) {
    if (adj[u][0] == 1) {
        printf("NO");
        exit(0);
    }

    adj[u][0] = 1; // seen

    for (int i = 1; i <= n; ++i) {
        if (adj[u][i] && adj[i][0] != 2) {
            dfs(i);
        }
    }
    adj[0][pi--] = u; // path
    adj[u][0] = 2;
}

int main() {
    scanf("%d %d", &n, &m);
    for (int i = 1; i <= m; ++i) {
        int u, v;
        scanf("%d %d", &u, &v);
        adj[u][v] = 1;
    }

    pi = n;
    for (int i = 1; i <= n; ++i) {
        if (!adj[i][0]) dfs(i);
    }

    printf("YES\n");
    while (pi < n) {
        printf("%d ", adj[0][++pi]);
    }
    return 0;
}