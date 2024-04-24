#include <bits/stdc++.h>
using namespace std;

int main()
{
    int n;
    cin >> n;
    int a[n][n];
    for (int i = 0; i < n; i++)
    {
        for (int j = 0; j < n; j++)
            cin >> a[i][j];
    }

    int ans = 0, i = n - 1, j = 0;
    while (not(i == 0 && j == n - 1))
    {
        ans += a[i][j];
        if (i > 0 && j < n - 1)
        {
            if (a[i - 1][j] > a[i][j + 1])
            {
                i--;
            }
            else
            {
                j++;
            }
        }
        else if (i == 0)
        {
            j++;
        }
        else
        {
            i--;
        }
    }
    ans += a[0][n - 1];
    cout << ans << endl;
    return 0;
}