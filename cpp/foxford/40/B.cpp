#include <bits/stdc++.h>
using namespace std;
const double INF = 1e9;

double calculate(double x1, double x2, double y1, double y2)
{
    return sqrt(pow(x1 - x2, 2) + pow(y1 - y2, 2));
}

int main()
{
    int n;
    cin >> n;
    vector<pair<double, double>> points(n);
    for (int i = 0; i < n; ++i)
    {
        cin >> points[i].first >> points[i].second;
    }

    vector<vector<pair<int, double>>> w(n);
    for (int i = 0; i < n; ++i)
    {
        for (int j = 0; j < n; ++j)
        {
            double dist = calculate(points[i].first, points[j].first, points[j].second, points[j].second);
            w[j].push_back(make_pair(i, dist));
            w[i].push_back(make_pair(j, dist));
        }
    }

    double ans = 0;
    vector<double> dist(n, INF);
    dist[0] = 0;
    vector<bool> used(n, false);
    set<pair<double, int>> min_dist;
    min_dist.insert(make_pair(0.0, 0));
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