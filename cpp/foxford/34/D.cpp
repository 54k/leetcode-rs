#include <bits/stdc++.h>

using namespace std;

const int INF = INT_MAX;

struct Edge
{
    int to, weight;
};

vector<int> dijkstra(int n, int start, int end, const vector<vector<Edge>> &graph)
{
    vector<int> dist(n, INF);
    vector<int> prev(n, -1);
    dist[start] = 0;
    priority_queue<pair<int, int>, vector<pair<int, int>>, greater<pair<int, int>>> pq;
    pq.push({0, start});

    while (!pq.empty())
    {
        int d = pq.top().first;
        int u = pq.top().second;
        pq.pop();

        if (d > dist[u])
            continue;

        for (const Edge &edge : graph[u])
        {
            int v = edge.to;
            int weight = edge.weight;
            if (dist[u] + weight < dist[v])
            {
                dist[v] = dist[u] + weight;
                prev[v] = u;
                pq.push({dist[v], v});
            }
        }
    }

    if (dist[end] == INF)
    {
        return {};
    }

    vector<int> path;
    for (int at = end; at != -1; at = prev[at])
    {
        path.push_back(at);
    }
    reverse(path.begin(), path.end());
    return path;
}

int main()
{
    int n, start, end;
    cin >> n >> start >> end;
    start--; // чтобы индексировать с нуля
    end--;

    vector<vector<Edge>> graph(n);

    for (int i = 0; i < n; ++i)
    {
        for (int j = 0; j < n; ++j)
        {
            int weight;
            cin >> weight;
            if (weight != -1 && i != j)
            {
                graph[i].push_back({j, weight});
            }
        }
    }

    vector<int> result = dijkstra(n, start, end, graph);

    if (result.empty())
    {
        cout << -1 << endl;
    }
    else
    {
        for (int vertex : result)
        {
            cout << vertex + 1 << " "; // выводим вершины, начиная с 1
        }
        cout << endl;
    }

    return 0;
}
