#include <bits/stdc++.h>
using namespace std;

int INF = 10e9;

int dijkstra(int n, int s, int f, vector<vector<pair<int, int>>> &graph)
{
    vector<int> l(n, INF);
    l[s] = 0;
    vector<bool> used(n);
    while (!used[f])
    {
        int min_len = INF;
        int min_v;
        for (int v = 0; v < n; v++)
        {
            if (!used[v] && l[v] < min_len)
            {
                min_v = v;
                min_len = l[v];
            }
        }
        if (min_len == INF)
        {
            break;
        }
        used[min_v] = true;
        for (const auto &[u, weight] : graph[min_v])
        {
            l[u] = min(l[u], l[min_v] + weight);
        }
    }
    if (!used[f])
    {
        return -1;
    }
    return l[f];
}

int main()
{
    return 0;
}