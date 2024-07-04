#include <iostream>
#include <vector>
#include <queue>
#include <unordered_map>
#include <unordered_set>

using namespace std;

vector<int> bfs_shortest_path(const unordered_map<int, vector<int>> &graph, int start, int end)
{
    queue<pair<int, vector<int>>> q;
    unordered_set<int> visited;
    q.push({start, {start}});
    visited.insert(start);

    while (!q.empty())
    {
        auto pair = q.front();
        auto current = pair.first;
        auto path = pair.second;
        q.pop();

        if (current == end)
        {
            return path;
        }

        for (int neighbor : graph.at(current))
        {
            if (visited.find(neighbor) == visited.end())
            {
                visited.insert(neighbor);
                vector<int> new_path = path;
                new_path.push_back(neighbor);
                q.push({neighbor, new_path});
            }
        }
    }
    return {};
}

int main()
{
    int n, m;
    cin >> n >> m;
    int start, end;
    cin >> start >> end;

    unordered_map<int, vector<int>> graph;
    for (int i = 0; i < m; ++i)
    {
        int u, v;
        cin >> u >> v;
        graph[u].push_back(v);
        graph[v].push_back(u);
    }

    vector<int> result = bfs_shortest_path(graph, start, end);

    if (result.empty())
    {
        cout << -1 << endl;
    }
    else
    {
        cout << result.size() - 1 << endl;
        for (int node : result)
        {
            cout << node << " ";
        }
        cout << endl;
    }

    return 0;
}
