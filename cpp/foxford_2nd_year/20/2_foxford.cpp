#include <iostream>
using namespace std;
const int SIZE = 1e6 + 228;
int n;
int a[SIZE];
int b[SIZE];
int cnt[1001];
int main()
{
    scanf("%d", &n);
    for (int i = 0; i < n; ++i)
    {
        scanf("%d", &a[i]);
        cnt[a[i]]++;
    }
    for (int i = 1; i <= 1000; ++i)
    {
        cnt[i] += cnt[i - 1];
    }
    for (int i = 0; i < n; ++i)
    {
        b[cnt[a[i] - 1]++] = a[i];
    }
    for (int i = 0; i < n; ++i)
    {
        printf("%d ", b[i]);
    }
    printf("\n");
    return 0;
}