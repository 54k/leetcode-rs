#include <stdio.h>
#define N 20
int main()
{
    int a[N];
    int i = 0, j = 0, k = 0;
    int ans = 0;
    for (i = 0; i < N; i++)
    {
        if (scanf("%d", &a[i]) == EOF)
        {
            break;
        }
        // printf("read %d\n", a[i]);
        j++;
    }
    // printf("j is %d", j);
    for (i = 1; i < j; i++)
    {
        // printf(" arr[i] % d %d \n", i, a[i]);
        if (a[i - 1] % 7 == 0 || a[i] % 7 == 0)
        {
            ans++;
        }
    }
    printf("%d", ans);
    return 0;
}