#include <bits/stdc++.h>
using namespace std;

const int N = 0;
vector<int> G[N];
int visited[N];
int mt[N];

bool kuhn_dfs(int v, int it)
{
    if (visited[v] == it)
        return false;
    visited[v] = it;

    for (auto &to : G[v])
    {
        if (mt[to] == -1)
        {
            mt[to] = v;
            return true;
        }
    }

    for (auto &to : G[v])
    {
        if (kuhn_dfs(mt[to], it))
        {
            mt[to] = v;
            return true;
        }
    }

    return false;
}

int find_max_mt()
{
    int ans = 0;
    int it = 0;
    for (int v = 0; v < N; v++)
    {
        if (kuhn_dfs(v, it++))
        {
            ans++;
        }
    }
    return ans;
}

int main()
{
    return 0;
}