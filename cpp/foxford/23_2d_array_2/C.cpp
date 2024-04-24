#include <bits/stdc++.h>
using namespace std;

int main()
{
    int n;
    cin >> n;
    vector<vector<int>> a(n, vector<int>(n, 0));

    for (int i = 0; i < a.size(); ++i)
        for (int j = 0; j < a[i].size(); ++j)
            cin >> a[i][j];

    for (int i = 0; i < (n + 1) / 2; i++)
    {
        for (int j = 0; j < n / 2; j++) {
            int temp = a[n - 1 - j][i];       
            a[n - 1 - j][i] = a[n - 1 - i][n - j - 1];
            a[n - 1 - i][n - j - 1] = a[j][n-1-i];
            a[j][n-1-i] = a[i][j];
            a[i][j] = temp;
        }
    }

    for (int i = 0; i < a.size(); ++i)
    {
        for (int j = 0; j < a[i].size(); ++j)
        {
            cout << a[i][j] << " ";
        }
        cout << endl;
    }

    return 0;
}