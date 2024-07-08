#include <bits/stdc++.h>
using namespace std;

int main()
{
    int n, s;
    cin >> n >> s;
    vector<vector<int>> adj(n, vector<int>(n, 0));
    for (int i = 0; i < n; i++)
        for (int j = 0; j < n; j++)
            cin >> adj[i][j];

    function<int(int)> dfs = [&](int v)
    {
        int ans = 1;
        for (int i = 0; i < n; i++)
        {
            if (v != i && adj[v][i] == 1)
            {
                adj[v][i] = 0;
                adj[i][v] = 0;
                ans += dfs(i);
            }
        }

        return ans;
    };

    cout << dfs(s - 1) << endl;
    return 0;
}