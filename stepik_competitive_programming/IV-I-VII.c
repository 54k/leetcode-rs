#include <stdio.h>

int row[501][501];

int main() {
    int n, m, u, v;
    scanf("%d %d", &n, &m);

    for (int i = 1; i <= m; ++i) {
        scanf("%d %d", &u, &v);
        row[u][v] = 1;
        row[v][u] = 1;
    }

    for (int i = 1; i <= n; ++i) {
        for (int j = 1; j <= n; ++j) {
            printf("%d", row[i][j]);
        }
        printf("\n");
    }
    return 0;
}