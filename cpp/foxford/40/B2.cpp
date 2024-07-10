#include <bits/stdc++.h>

using namespace std;

struct Edge
{
    int v;
    int weight;
};

struct Node
{
    int vertex;
    int key;
    bool operator>(const Node &other) const
    {
        return key > other.key;
    }
};

int primMST(const vector<vector<Edge>> &graph)
{
    int n = graph.size();
    vector<bool> inMST(n, false);
    vector<int> key(n, numeric_limits<int>::max());
    priority_queue<Node, vector<Node>, greater<Node>> pq;

    key[0] = 0;
    pq.push({0, 0});

    int mst_weight = 0;

    while (!pq.empty())
    {
        int u = pq.top().vertex;
        pq.pop();

        if (inMST[u])
            continue;

        inMST[u] = true;
        mst_weight += key[u];

        for (const auto &edge : graph[u])
        {
            int v = edge.v;
            int weight = edge.weight;

            if (!inMST[v] && weight < key[v])
            {
                key[v] = weight;
                pq.push({v, key[v]});
            }
        }
    }

    return mst_weight;
}

int main()
{
    int n, m;
    cin >> n >> m;

    vector<vector<Edge>> graph(n);

    for (int i = 0; i < m; ++i)
    {
        int u, v, w;
        cin >> u >> v >> w;
        u--; // Переводим в 0-индексацию
        v--; // Переводим в 0-индексацию
        graph[u].push_back({v, w});
        graph[v].push_back({u, w});
    }

    cout << primMST(graph) << endl;

    return 0;
}
