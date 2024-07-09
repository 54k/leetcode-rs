#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

struct Edge
{
    int u, v, w;
};

vector<vector<int>> adj;
vector<int> tin, low;
vector<bool> visited;
int timer;
vector<int> bridges;

void dfs(int v, int p = -1)
{
    visited[v] = true;
    tin[v] = low[v] = timer++;
    for (int to : adj[v])
    {
        if (to == p)
            continue;
        if (visited[to])
        {
            low[v] = min(low[v], tin[to]);
        }
        else
        {
            dfs(to, v);
            low[v] = min(low[v], low[to]);
            if (low[to] > tin[v])
            {
                bridges.push_back(to);
            }
        }
    }
}

void find_bridges(int n)
{
    timer = 0;
    tin.assign(n, -1);
    low.assign(n, -1);
    visited.assign(n, false);
    for (int i = 0; i < n; ++i)
    {
        if (!visited[i])
        {
            dfs(i);
        }
    }
}

int main()
{
    int n, m;
    cin >> n >> m;

    adj.resize(n);
    vector<Edge> edges(m);
    for (int i = 0; i < m; ++i)
    {
        int u, v, w;
        cin >> u >> v >> w;
        u--;
        v--;
        w--;
        adj[u].push_back(v);
        adj[v].push_back(w);
        adj[w].push_back(u);
        edges[i] = {u, v, w};
    }

    find_bridges(n);

    vector<int> critical_pads;
    for (int i = 0; i < m; ++i)
    {
        int u = edges[i].u;
        int v = edges[i].v;
        int w = edges[i].w;
        if (find(bridges.begin(), bridges.end(), u) != bridges.end() ||
            find(bridges.begin(), bridges.end(), v) != bridges.end() ||
            find(bridges.begin(), bridges.end(), w) != bridges.end())
        {
            critical_pads.push_back(i + 1);
        }
    }

    cout << critical_pads.size() << endl;
    for (int pad : critical_pads)
    {
        cout << pad << " ";
    }
    cout << endl;

    return 0;
}
