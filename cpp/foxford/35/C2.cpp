#include <bits/stdc++.h>
using namespace std;

const int INF = 100000;
const int UNDEFINED = -1;

void find_negative_cycle(int n, const vector<vector<int>> &graph)
{
    vector<int> dist(n, 0);           // начальное расстояние до всех вершин 0
    vector<int> parent(n, UNDEFINED); // для восстановления пути
    int x = UNDEFINED;                // вершина, участвующая в цикле

    // алгоритм Форда-Беллмана
    for (int i = 0; i < n; ++i)
    {
        x = UNDEFINED;
        for (int u = 0; u < n; ++u)
        {
            for (int v = 0; v < n; ++v)
            {
                if (graph[u][v] != INF)
                {
                    if (dist[v] > dist[u] + graph[u][v])
                    {
                        dist[v] = dist[u] + graph[u][v];
                        parent[v] = u;
                        x = v;
                    }
                }
            }
        }
    }

    if (x == UNDEFINED)
    {
        cout << "NO" << endl;
    }
    else
    {
        cout << "YES" << endl;
        // Восстановление цикла
        for (int i = 0; i < n; ++i)
        {
            x = parent[x];
        }

        vector<int> cycle;
        for (int v = x;; v = parent[v])
        {
            cycle.push_back(v);
            if (v == x && cycle.size() > 1)
            {
                break;
            }
        }

        reverse(cycle.begin(), cycle.end());

        cout << cycle.size() << endl;
        for (int v : cycle)
        {
            cout << v + 1 << " "; // Переход к 1-индексации
        }
        cout << endl;
    }
}

int main()
{
    int n;
    cin >> n;

    vector<vector<int>> graph(n, vector<int>(n));
    for (int i = 0; i < n; ++i)
    {
        for (int j = 0; j < n; ++j)
        {
            cin >> graph[i][j];
        }
    }

    find_negative_cycle(n, graph);

    return 0;
}
