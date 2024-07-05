#include <iostream>
#include <vector>

using namespace std;

const int INF = 30000;

int main()
{
    int n, m;
    cin >> n >> m;
    vector<int> dist(n + 1, INF);
    dist[1] = 0;
    vector<vector<int>> edges(m, vector<int>(3));
    for (int i = 0; i < m; ++i)
        cin >> edges[i][0] >> edges[i][1] >> edges[i][2];
    for (int i = 0; i < n - 1; ++i)
        for (auto &edge : edges)
        {
            int start = edge[0];
            int end = edge[1];
            int price = edge[2];
            if (dist[start] != INF && dist[start] + price < dist[end])
                dist[end] = dist[start] + price;
        }
    for (int i = 1; i <= n; ++i)
        cout << dist[i] << " ";
    cout << endl;
}