#include <stdio.h>
#define min(x, y) (x < y ? x : y)

int main() {
    int n,m;
    scanf("%d", &n);
    scanf("%d", &m);

    int arr[n][m];
    for (int i = 0; i < n; ++i)
        for (int j = 0; j < m; ++j)
            scanf("%d", &arr[i][j]);

    for (int i = 1; i < n; ++i)
        arr[i][0] = arr[i - 1][0] + arr[i][0];
    for (int j = 1; j < m; ++j)
        arr[0][j] = arr[0][j - 1] + arr[0][j];

    for (int i = 1; i < n; ++i)
        for (int j = 1; j < m; ++j)
            arr[i][j] = arr[i][j] + min(min(arr[i - 1][j], arr[i][j - 1]), arr[i - 1][j - 1]);

    printf("%d", arr[n - 1][m - 1]);
    return 0;
}
