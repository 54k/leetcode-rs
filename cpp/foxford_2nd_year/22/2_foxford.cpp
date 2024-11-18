#include <stdio.h>
int main()
{
    int n;
    scanf("%d", &n);
    int a[n][n];
    for (int i = 0; i < n; ++i)
        for (int j = 0; j < n; ++j)
            if (i > j)
                a[i][j] = i - j;
            else
                a[i][j] = j - i;
    for (int i = 0; i < n; ++i)
    {
        for (int j = 0; j < n; ++j)
            printf("%d ", a[i][j]);
        printf("\n");
    }
}