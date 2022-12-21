#include <stdio.h>

#define max(x, y) (x > y ? x : y)

int main(void) {
    int n;
    scanf("%d", &n);
    int arr[n + 2][n + 2];
    for (int i = 0; i <= n; ++i) {
        arr[i][0] = -101;
        arr[i][i + 1] = -101;
    }
    for (int i = 1; i <= n; ++i) {
        for (int j = 1; j <= i; ++j) {
            scanf("%d", &arr[i][j]);
        }
    }
    for (int i = 2; i <= n; ++i) {
        for (int j = 1; j <= i; ++j) {
            int u = max(arr[i - 1][j], arr[i - 1][j - 1]);
            arr[i][j] += u;
        }
    }

    int ans = -101;
    for (int i = 1; i <= n; ++i) {
        ans = max(ans, arr[n][i]);
    }
    printf("%d", ans);
    return 0;
}