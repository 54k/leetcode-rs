#include <stdio.h>

int M = 1000000009;

int mod(int x) {
    return ((x % M) + M) % M;
}

int mod_add(int x, int y) {
    return mod(mod(x) + mod(y));
}

int main() {
    int n, m;
    scanf("%d", &n);
    scanf("%d", &m);

    int arr[n][m];
    for (int i = 0; i < n; ++i)
        for (int j = 0; j < m; ++j) {
            int d;
            scanf("%d", &d);
            arr[i][j] = !d;
        }

    for (int i = 1; i < n; ++i) {
        arr[i][0] = arr[i - 1][0] * arr[i][0];
    }
    for (int i = 1; i < m; ++i) {
        arr[0][i] = arr[0][i - 1] * arr[0][i];
    }

    for (int i = 1; i < n; ++i)
        for (int j = 1; j < m; ++j)
            arr[i][j] = mod_add(arr[i - 1][j], mod_add(arr[i][j - 1], arr[i - 1][j - 1])) * arr[i][j];

    printf("%d", arr[n - 1][m - 1]);
    return 0;
}
