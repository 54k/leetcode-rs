#include <bits/stdc++.h>
using namespace std;

const int INF = 10e9;
int n = 0;

void dijkstra(vector<vector<pair<int, int>>> graph)
{
    int s;
    cin >> s;
    vector<int> l(n, INF);
    l[s] = 0;
    set<pair<int, int>> st;
    st.insert({0, s});

    while (!st.empty())
    {
        int min_v = st.begin()->second;
        st.erase(st.begin());
        for (const auto &[u, weight] : graph[min_v])
        {
            if (l[min_v] + weight < l[u])
            {
                st.erase({l[u], u});
                l[u] = l[min_v] + weight;
                st.insert({l[u], u});
            }
        }
    }
    for (int i = 0; i < n; ++i)
    {
        cout << l[i] << ' ';
    }
    cout << '\n';
}

int main()
{
    return 0;
}