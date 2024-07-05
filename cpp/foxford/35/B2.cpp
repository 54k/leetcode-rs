#include <iostream>
#include <vector>
using namespace std;
const int INF = 1e9;
int main()
{
    int n, m, k, s, f;
    cin >> n >> m >> k >> s >> f;
    vector<int> dist(n + 1, INF);
    vector<int> dist_new;
    dist[s] = 0;
    vector<vector<int>> edges(m, vector<int>(3));
    for (int i = 0; i < m; ++i)
        cin >> edges[i][0] >> edges[i][1] >> edges[i][2];
    for (int i = 0; i < k; ++i)
    {
        dist_new = dist;
        for (auto &edge : edges)
        {
            int start = edge[0];
            int end = edge[1];
            int price = edge[2];
            if (dist[start] + price < dist_new[end])
                dist_new[end] = dist[start] + price;
        }
        swap(dist, dist_new);
    }
    if (dist[f] == INF)
        cout << -1 << endl;
    else
        cout << dist[f] << endl;
    return 0;
}