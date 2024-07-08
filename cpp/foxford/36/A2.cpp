#include <iostream>
#include <vector>

using namespace std;

void dfs(int v, const vector<vector<int>> &graph, vector<bool> &visited, int &count)
{
    visited[v] = true;
    count++;
    for (int i = 0; i < graph.size(); ++i)
    {
        if (graph[v][i] == 1 && !visited[i])
        {
            dfs(i, graph, visited, count);
        }
    }
}

int main()
{
    int n, s;
    cin >> n >> s;
    s--; // Переводим вершину в 0-индексацию

    vector<vector<int>> graph(n, vector<int>(n));
    for (int i = 0; i < n; ++i)
    {
        for (int j = 0; j < n; ++j)
        {
            cin >> graph[i][j];
        }
    }

    vector<bool> visited(n, false);
    int count = 0;
    dfs(s, graph, visited, count);

    cout << count << endl;

    return 0;
}
