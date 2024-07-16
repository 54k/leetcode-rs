#include <bits/stdc++.h>

using namespace std;

int main()
{
    int n, m, k;
    cin >> n >> m >> k;

    vector<vector<long long>> arr(n + 1, vector<long long>(m + 1));
    for (int i = 0; i < n; i++)
    {
        for (int j = 0; j < m; j++)
        {
            cin >> arr[i + 1][j + 1];
            arr[i + 1][j + 1] += arr[i + 1][j] + arr[i][j + 1] - arr[i][j];
        }
    }

    while (k-- > 0)
    {
        int x1, y1, x2, y2;
        cin >> x1 >> y1 >> x2 >> y2;
        long long res = arr[x2][y2] - arr[x1 - 1][y2] - arr[x2][y1 - 1] + arr[x1 - 1][y1 - 1];
        cout << res << "\n";
    }

    cout << endl;
    return 0;
}