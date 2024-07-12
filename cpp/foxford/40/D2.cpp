#include <iostream>
#include <vector>
#include <algorithm>
#include <string>

using namespace std;

void init(int n, vector<int> &p, vector<int> &d)
{
    p.resize(n);
    d.resize(n);
    for (int i = 0; i < n; ++i)
    {
        p[i] = i;
    }
}

int get_par(int v, vector<int> &p)
{
    if (p[v] == v)
    {
        return v;
    }
    return p[v] = get_par(p[v], p);
}

void unite(int u, int v, vector<int> &p, vector<int> &d)
{
    u = get_par(u, p);
    v = get_par(v, p);
    if (u != v)
    {
        if (d[u] > d[v])
        {
            swap(u, v);
        }
        p[u] = v;
        if (d[u] == d[v])
        {
            d[v]++;
        }
    }
}

int main()
{
    int n, m, k;
    vector<int> p, d;
    cin >> n >> m >> k;
    init(n, p, d);
    vector<pair<int, pair<int, int>>> query;
    vector<int> ans;
    for (int i = 0; i < m; ++i)
    {
        int a, b;
        cin >> a >> b;
    }
    for (int i = 0; i < k; ++i)
    {
        string tpe;
        int u, v;
        cin >> tpe >> u >> v;
        if (tpe[0] == 'a')
        {
            query.push_back({0, {u - 1, v - 1}});
        }
        else
        {
            query.push_back({1, {u - 1, v - 1}});
        }
    }
    reverse(query.begin(), query.end());
    for (int i = 0; i < k; ++i)
    {
        if (query[i].first)
        {
            unite(query[i].second.first, query[i].second.second, p, d);
        }
        else
        {
            if (get_par(query[i].second.first, p) == get_par(query[i].second.second, p))
            {
                ans.push_back(1);
            }
            else
            {
                ans.push_back(0);
            }
        }
    }
    reverse(ans.begin(), ans.end());
    for (int i : ans)
    {
        cout << (i ? "YES\n" : "NO\n");
    }
    system("pause");
    return 0;
}