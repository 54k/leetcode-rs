#include <bits/stdc++.h>
using namespace std;

int main()
{
    int n, m;
    cin >> n >> m;
    vector<int> color(n, -1);
    vector<vector<int>> adj(n, vector<int>());
    for (int i = 0; i < m; i++)
    {
        int f, t;
        cin >> f >> t;
        adj[f - 1].emplace_back(t - 1);
        adj[t - 1].emplace_back(f - 1);
    }

    function<bool(int, int)> dfs = [&](int v, int c)
    {
        color[v] = c;
        for (auto &u : adj[v])
        {
            if (color[u] == -1)
            {
                if (!dfs(u, 1 - color[v]))
                {
                    return false;
                }
            }
            else if (color[u] == color[v])
            {
                return false;
            }
        }
        return true;
    };

    for (int i = 0; i < n; i++)
    {
        if (color[i] == -1 && !dfs(i, 1))
        {
            cout << "NO" << endl;
            return 0;
        }
    }

    cout << "YES" << endl;

    for (int i = 0; i < n; i++)
    {
        if (color[i] == 0)
        {
            cout << i + 1 << " ";
        }
    }
    cout << endl;
    return 0;
}