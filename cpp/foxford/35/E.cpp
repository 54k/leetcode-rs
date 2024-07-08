#include <bits/stdc++.h>
using namespace std;

int main()
{
    const int INF = 0;
    int n;
    cin >> n;
    vector<vector<double>> adj(n, vector<double>(n, INF));
    for (int i = 0; i < n; i++)
    {
        for (int j = 0; j < n; j++)
        {
            cin >> adj[i][j];
        }
    }

    for (int k = 0; k < n; k++)
    {
        for (int i = 0; i < n; i++)
        {
            for (int j = 0; j < n; j++)
            {
                adj[i][j] = max(adj[i][j], adj[i][k] * adj[k][j]);
            }
        }
    }
    bool res = false;
    for (int i = 0; i < n; i++)
    {
        if (adj[i][i] > 1.0)
        {
            res = true;
            break;
        }
    }
    cout << (res ? "YES" : "NO") << endl;
    return 0;
}