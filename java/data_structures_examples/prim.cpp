// https://foxford.ru/lessons/284927/tasks
#include <iostream>
#include <vector>
#include <set>

using namespace std;

const int INF = 1e9;
int main()
{
    int n, m;
    cin >> n >> m;
    vector<vector<pair<int, int>>> w(n);
    for (int i = 0; i < m; ++i)
    {
        int start, end, weight;
        cin >> start >> end >> weight;
        start--;
        end--;
        w[start].push_back(make_pair(end, weight));
        w[end].push_back(make_pair(start, weight));
    }
    long long ans = 0;
    vector<int> dist(n, INF);
    dist[0] = 0;
    vector<bool> used(n, false);
    set<pair<int, int>> min_dist;
    min_dist.insert(make_pair(0, 0));
    while (!min_dist.empty())
    {
        int u = min_dist.begin()->second;
        ans += min_dist.begin()->first;
        used[u] = true;
        min_dist.erase(min_dist.begin());
        for (auto edge : w[u])
        {
            int v = edge.first;
            int wt = edge.second;
            if (!used[v] && dist[v] > wt)
            {
                min_dist.erase(make_pair(dist[v], v));
                dist[v] = wt;
                min_dist.insert(make_pair(dist[v], v));
            }
        }
    }
    cout << ans << endl;
}