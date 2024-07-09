#include <bits/stdc++.h>

using namespace std;

// // Функция для выполнения топологической сортировки
// bool topological_sort(int n, const vector<vector<int>> &adj, vector<int> &result)
// {
//     vector<int> in_degree(n, 0);

//     // Подсчёт входящих рёбер для каждой вершины
//     for (int u = 0; u < n; ++u)
//     {
//         for (int v : adj[u])
//         {
//             in_degree[v]++;
//         }
//     }

//     queue<int> q;
//     // Находим все вершины с нулевой степенью входа
//     for (int i = 0; i < n; ++i)
//     {
//         if (in_degree[i] == 0)
//         {
//             q.push(i);
//         }
//     }

//     while (!q.empty())
//     {
//         int u = q.front();
//         q.pop();
//         result.push_back(u);

//         for (int v : adj[u])
//         {
//             if (--in_degree[v] == 0)
//             {
//                 q.push(v);
//             }
//         }
//     }

//     // Если удалось упорядочить все вершины, то топологическая сортировка возможна
//     return result.size() == n;
// }

bool topological_sort(int v, const vector<vector<int>> &adj, vector<int> &result, vector<int> &vis)
{
    if (vis[v] == 1)
    {
        return false;
    }
    vis[v] = 1;
    for (auto &u : adj[v])
    {
        if (vis[u] != 2 && !topological_sort(u, adj, result, vis))
        {
            return false;
        }
    }
    vis[v] = 2;
    result.push_back(v);
    return true;
}

int main()
{
    int n, m;
    cin >> n >> m;

    vector<vector<int>> adj(n, vector<int>());
    for (int i = 0; i < m; ++i)
    {
        int a, b;
        cin >> a >> b;
        --a;
        --b; // Переход к 0-индексации
        adj[a].push_back(b);
    }

    vector<int> visited(n, 0);
    vector<int> result;

    bool is_successful = true;
    for (int i = 0; i < n; i++)
    {
        if (visited[i] == 0 && !topological_sort(i, adj, result, visited))
        {
            is_successful = false;
            break;
        }
    }

    if (is_successful)
    {
        cout << "Yes" << endl;
        reverse(result.begin(), result.end());
        for (int &u : result)
        {
            cout << u + 1 << " "; // Переход обратно к 1-индексации
        }
        cout << endl;
    }
    else
    {
        cout << "No" << endl;
    }

    return 0;
}
