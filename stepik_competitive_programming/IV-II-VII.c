#include <stdio.h>
#include <stdlib.h>

#define N 501

int adj[N][N];

int n, m, pi;

void dfs(int u) {
    if (adj[u][0] == 1) {
        while (adj[0][n] != u) n--;
        printf("YES\n%d\n", n - pi);
        while (n > pi) {
            printf("%d ", adj[0][n--]);
        }
        exit(0);
    }

    adj[u][0] = 1;
    adj[0][pi--] = u;

    for (int i = 1; i <= n; ++i) {
        if (adj[u][i] && adj[i][0] != 2) {
            dfs(i);
        }
    }

    adj[u][0] = 2;
}

int main() {
    scanf("%d %d", &n, &m);
    for (int i = 1; i <= m; ++i) {
        int u, v;
        scanf("%d %d", &u, &v);
        adj[u][v] = 1;
    }
    for (int i = 1; i <= n; ++i) {
        pi = n;
        if (adj[i][0] != 2) dfs(i);
    }
    printf("NO");
    return 0;
}