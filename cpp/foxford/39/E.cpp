#include <iostream>
#include <vector>
#include <set>
using namespace std;
const long long INF = 1e18;
int main()
{
    int n, m;
    cin >> n >> m;
    vector<vector<pair<int, int>>> w(n);
    for (int k = 0; k < m; ++k)
    {
        int i, j, wt;
        cin >> i >> j >> wt;
        w[i - 1].push_back(make_pair(j - 1, wt));
        w[j - 1].push_back(make_pair(i - 1, wt));
    }
    int start, finish;
    cin >> start >> finish;
    --start;
    --finish;
    vector<long long> dist(n, INF);
    dist[start] = 0;
    set<pair<long long, int>> s;
    s.insert(make_pair(0, start));
    while (!s.empty())
    {
        int i = s.begin()->second;
        s.erase(s.begin());
        for (auto edge : w[i])
        {
            int j = edge.first;
            int wt = edge.second;
            if (dist[i] + wt < dist[j])
            {
                s.erase(make_pair(dist[j], j));
                dist[j] = dist[i] + wt;
                s.insert(make_pair(dist[j], j));
            }
        }
    }
    if (dist[finish] == INF)
        cout << -1 << endl;
    else
        cout << dist[finish] << endl;
    return 0;
}