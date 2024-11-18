#include <stdio.h>
int main()
{
    int n, m, i, j, ans = 0, _max = 0;
    scanf("%d %d", &n, &m);
    int a[n][m];
    for (i = 0; i < n; i++)
    {
        for (j = 0; j < m; j++)
        {
            scanf("%d", &a[i][j]);
            if (a[i][j] > _max)
            {
                _max = a[i][j];
            }
        }
    }
    for (i = 0; i < n; i++)
    {
        for (j = 0; j < m; j++)
        {
            if (a[i][j] * 2 < _max)
                ans++;
        }
    }
    printf("%d\n", ans);
    return 0;
}