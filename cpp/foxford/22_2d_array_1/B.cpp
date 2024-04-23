#include <bits/stdc++.h>
using namespace std;

int main()
{
    int n;
    cin >> n;
    vector<vector<int>> v(n, vector<int>(n));
    for (int i = 0; i < n; i++)
    {
        for (int j = 0; j < i; j++)
        {
            v[i][j] = i - j;
            cout << v[i][j] << " ";
        }
        cout << v[i][i] << " ";
        for (int j = i + 1; j < n; j++)
        {
            v[i][j] = j - i;
            cout << v[i][j] << " ";
        }
        cout << "\n";
    }
    return 0;
}