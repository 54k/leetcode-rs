#include <bits/stdc++.h>
using namespace std;

int main()
{
    int n, m;
    cin >> n >> m;
    vector<vector<int>> g(n, vector<int>(m, 0));
    for (int i = 0; i < n; i++)
    {
        for (int j = 0; j < m; j++)
        {
            cin >> g[i][j];
        }
    }

    vector<vector<int>> dist(n, vector<int>(m, 1e9));
    queue<vector<int>> q;

    for (int i = 0; i < n; i++)
    {
        for (int j = 0; j < m; j++)
        {
            if (g[i][j] == 1)
            {
                dist[i][j] = 0;
                q.push({i, j});
            }
        }
    }

    while (!q.empty())
    {
        auto u = q.front();
        q.pop();
        vector<vector<int>> dir = {{0, 1}, {0, -1}, {1, 0}, {-1, 0}};

        for (auto &d : dir)
        {
            auto nx = u[0] + d[0];
            auto ny = u[1] + d[1];

            if (0 <= nx && nx < n && 0 <= ny && ny < m)
            {
                if (dist[u[0]][u[1]] + 1 < dist[nx][ny])
                {
                    dist[nx][ny] = dist[u[0]][u[1]] + 1;
                    q.push({nx, ny});
                }
            }
        }
    }

    for (int i = 0; i < n; i++)
    {
        for (int j = 0; j < m; j++)
        {
            cout << dist[i][j] << " ";
        }

        cout << "\n";
    }

    return 0;
}