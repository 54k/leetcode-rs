#include <iostream>
#include <vector>
#include <queue>

using namespace std;

bool bfs(int v, const vector<vector<int>> &graph, vector<int> &colors)
{
    queue<int> q;
    q.push(v);
    colors[v] = 1; // Начинаем с окраски вершины в 1

    while (!q.empty())
    {
        int u = q.front();
        q.pop();

        for (int neighbor : graph[u])
        {
            if (colors[neighbor] == -1)
            {
                // Если сосед ещё не окрашен, окрашиваем в противоположный цвет
                colors[neighbor] = 1 - colors[u];
                q.push(neighbor);
            }
            else if (colors[neighbor] == colors[u])
            {
                // Если сосед окрашен в тот же цвет, граф не двудольный
                return false;
            }
        }
    }

    return true;
}

int main()
{
    int n, m;
    cin >> n >> m;

    vector<vector<int>> graph(n);
    for (int i = 0; i < m; ++i)
    {
        int u, v;
        cin >> u >> v;
        u--;
        v--; // Переводим в 0-индексацию
        graph[u].push_back(v);
        graph[v].push_back(u);
    }

    vector<int> colors(n, -1); // Цвета: -1 - не окрашен, 0 - за первый стол, 1 - за второй стол

    bool is_bipartite = true;
    for (int i = 0; i < n; ++i)
    {
        if (colors[i] == -1)
        {
            if (!bfs(i, graph, colors))
            {
                is_bipartite = false;
                break;
            }
        }
    }

    if (is_bipartite)
    {
        cout << "YES" << endl;
        for (int i = 0; i < n; ++i)
        {
            if (colors[i] == 0)
            {
                cout << i + 1 << " ";
            }
        }
        cout << endl;
    }
    else
    {
        cout << "NO" << endl;
    }

    return 0;
}
