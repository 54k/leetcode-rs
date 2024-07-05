#include <iostream>
#include <vector>
#include <tuple>

using namespace std;

const int INF = 30000;

void bellman_ford(int n, int m, const vector<tuple<int, int, int>>& edges) {
    vector<int> dist(n, INF);
    dist[0] = 0; // стартовая вершина 1, индексация с 0

    for (int i = 0; i < n - 1; ++i) {
        for (const auto& edge : edges) {
            int u, v, w;
            tie(u, v, w) = edge;
            if (dist[u] != INF && dist[u] + w < dist[v]) {
                dist[v] = dist[u] + w;
            }
        }
    }

    for (int d : dist) {
        if (d == INF) {
            cout << 30000 << " ";
        } else {
            cout << d << " ";
        }
    }
    cout << endl;
}

int main() {
    int n, m;
    cin >> n >> m;

    vector<tuple<int, int, int>> edges;
    for (int i = 0; i < m; ++i) {
        int u, v, w;
        cin >> u >> v >> w;
        edges.emplace_back(u - 1, v - 1, w); // приведение к 0-индексации
    }

    bellman_ford(n, m, edges);

    return 0;
}
