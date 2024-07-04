#include <iostream>
#include <vector>
using namespace std;
const int INF = 1000000000;

int main()
{
    int n, m;
    cin >> n;
    vector<int> price(n);
    for (int i = 0; i < n; ++i)
        cin >> price[i];
    vector<vector<int>> g(3 * n, vector<int>(3 * n, INF));
    for (int i = 0; i < n; ++i)
    {
        g[3 * i][3 * i + 1] = price[i];
        g[3 * i + 1][3 * i + 2] = price[i];
    }
    cin >> m;
    for (int i = 0; i < m; ++i)
    {
        int u, v;
        cin >> u >> v;
        --u;
        --v;
        g[3 * u + 1][3 * v] = 0;
        g[3 * u + 2][3 * v + 1] = 0;
        g[3 * v + 1][3 * u] = 0;
        g[3 * v + 2][3 * u + 1] = 0;
    }
    vector<int> dist(3 * n, INF);
    dist[0] = 0;
    vector<bool> used(3 * n);
    int min_dist = 0;
    int min_vertex = 0;
    while (min_dist < INF)
    {
        used[min_vertex] = true;
        int u = min_vertex;
        for (int v = 0; v < 3 * n; ++v)
            if (dist[u] + g[u][v] < dist[v])
                dist[v] = dist[u] + g[u][v];
        min_dist = INF;
        for (int v = 0; v < 3 * n; ++v)
            if (!used[v] && dist[v] < min_dist)
            {
                min_dist = dist[v];
                min_vertex = v;
            }
    }
    if (dist[3 * (n - 1)] == INF)
        cout << -1 << endl;
    else
        cout << dist[3 * (n - 1)] << endl;
}