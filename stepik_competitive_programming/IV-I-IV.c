#include <stdio.h>
#include <stdlib.h>

int row[501];

int main() {
    int n;
    scanf("%d", &n);
    char *s = malloc(n * sizeof(char));

    for (int i = 1; i <= n; ++i) {
        int c = 0;
        scanf("%s", s);
        int k = 1;
        for (int j = 1; j <= n; ++j) {
            int v = (*(s++) - '0');
            if (v) {
                row[k++] = j;
                ++c;
            }
        }

        printf("%d ", c);
        for (int j = 1; j <= c; ++j) {
            if (row[j]) {
                printf("%d ", row[j]);
            }
        }
        printf("\n");
        s -= n;
    }

    free(s);
    return 0;
}