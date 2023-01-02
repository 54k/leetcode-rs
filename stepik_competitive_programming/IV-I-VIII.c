#include <stdio.h>

int row[501][501];

int main() {
    int n, c, e;
    scanf("%d", &n);

    for (int i = 1; i <= n; ++i) {
        scanf("%d", &c);
        for (int j = 1; j <= c; ++j) {
            scanf("%d", &e);
            row[i][e] = 1;
        }
    }

    for (int i = 1; i <= n; ++i) {
        for (int j = 1; j <= n; ++j) {
            printf("%d", row[i][j]);
        }
        printf("\n");
    }
    return 0;
}