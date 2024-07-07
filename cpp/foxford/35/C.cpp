#include <bits/stdc++.h>
using namespace std;

int main()
{
    const int INF = 100000;
    int n;
    cin >> n;
    vector<vector<int>> g(n, vector<int>(n, INF));

    for (int i = 0; i < n; i++)
    {
        for (int j = 0; j < n; j++)
        {
            cin >> g[i][j];
        }
    }

    vector<int> dist(n, INF);
    vector<int> par(n, -1);
    for (int k = 0; k < n; k++)
    {
        for (int i = 0; i < n; i++)
        {
            for (int j = 0; j < n; j++)
            {
                if (i == j)
                    continue;
                if (dist[i] + g[i][j] < dist[j])
                {
                    dist[j] = dist[i] + g[i][j];
                }
            }
        }
    }

    bool found = false;
    int onCycle = 0;
    for (int i = 0; i < n; i++)
    {
        for (int j = 0; j < n; j++)
        {
            if (i == j)
                continue;
            if (dist[i] + g[i][j] < dist[j])
            {
                dist[j] = dist[i] + g[i][j];
                par[j] = i;
                onCycle = j;
                found = true;
            }
        }
    }

    if (!found)
    {
        cout << "NO" << endl;
    }
    else
    {

        cout << "YES" << endl;
        int cur = onCycle;
        vector<int> cycle;
        vector<int> vis(n, 0);
        while (!vis[cur])
        {
            cycle.push_back(cur);
            vis[cur] = 1;
            cur = par[cur];
        }
        cycle.push_back(cycle.front());
        cout << cycle.size() << endl;
        for (auto &x : cycle)
        {
            cout << x+1 << " ";
        }
    }

    return 0;
}