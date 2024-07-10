#include <bits/stdc++.h>

using namespace std;

const long long INF = numeric_limits<long long>::max();

struct Edge
{
    int to;
    long long weight;
};

void dijkstra(int n, const vector<vector<Edge>> &graph, int start, vector<long long> &distances)
{
    distances.assign(n + 1, INF);
    distances[start] = 0;
    priority_queue<pair<long long, int>, vector<pair<long long, int>>, greater<pair<long long, int>>> pq;
    pq.push({0, start});

    while (!pq.empty())
    {
        pair<long long, int> dv = pq.top();
        pq.pop();
        auto current_distance = dv.first;
        auto v = dv.second;

        if (current_distance > distances[v])
        {
            continue;
        }

        for (const auto &edge : graph[v])
        {
            int to = edge.to;
            long long weight = edge.weight;

            if (distances[v] + weight < distances[to])
            {
                distances[to] = distances[v] + weight;
                pq.push({distances[to], to});
            }
        }
    }
}

int main()
{
    int n, m;
    cin >> n >> m;

    vector<vector<Edge>> graph(n + 1);

    for (int i = 0; i < m; ++i)
    {
        int u, v;
        long long w;
        cin >> u >> v >> w;
        graph[u].push_back({v, w});
        graph[v].push_back({u, w}); // Поскольку дороги двусторонние
    }

    int start, end;
    cin >> start >> end;

    vector<long long> distances;
    dijkstra(n, graph, start, distances);

    if (distances[end] == INF)
    {
        cout << -1 << endl;
    }
    else
    {
        cout << distances[end] << endl;
    }

    return 0;
}
