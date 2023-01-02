#include <stdio.h>
#include <stdlib.h>

int row[501][501];

int main() {
    int n;
    scanf("%d", &n);
    char *s = malloc(n * sizeof(char));

    for (int i = 1; i <= n; ++i) {
        int c = 0;
        scanf("%s", s);
        for (int j = 1; j <= n; ++j) {
            row[i][j] = (*(s++) - '0');
            if (row[i][j]) {
                ++c;
            }
        }
        row[0][0] += c;
        s -= n;
    }

    free(s);

    printf("%d\n", row[0][0] / 2);
    for (int i = 1; i <= n; ++i) {
        for (int j = 1; j <= n; ++j) {
            if (row[i][j] && i < j) {
                printf("%d %d\n", i, j);
            }
        }
    }

    return 0;
}