#include <stdio.h>
int main()
{
    int n, m, i, j;
    scanf("%d %d", &n, &m);
    int a[n][m], b[n][m];
    for (i = 0; i < n; i++)
    {
        for (j = 0; j < m; j++)
        {
            scanf("%d", &a[i][j]);
        }
    }
    for (i = 0; i < n; i++)
    {
        for (j = 0; j < m; j++)
        {
            b[i][j] = 0;
            if (i > 0)
                b[i][j] += a[i - 1][j];
            if (j > 0)
                b[i][j] += a[i][j - 1];
            if (i < n - 1)
                b[i][j] += a[i + 1][j];
            if (j < m - 1)
                b[i][j] += a[i][j + 1];
        }
    }
    for (i = 0; i < n; i++)
    {
        for (j = 0; j < m; j++)
        {
            printf("%d", b[i][j]);
            if (j < m - 1)
                printf(" ");
        }
        printf("\n");
    }
    return 0;
}