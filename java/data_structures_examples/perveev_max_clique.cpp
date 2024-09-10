#include <bits/stdc++.h>
using namespace std;

template <typename T>
bool smax(T &a, const T &b)
{
    if (a < b)
    {
        a = b;
        return true;
    }
    return false;
}

const int N = 10000; // whatever

vector<int> g[N], e[N], dp[N];

void run()
{
    int n;
    int left = n / 2;
    int right = n - left;

    for (int v = 0; v < n; ++v)
    {
        for (auto to : g[v])
        {
            e[v] |= 1 << to;
        }
    }

    // left part
    dp[0] = 0;
    for (int mask = 1; mask < (1 << left); ++mask)
    {
        for (int v = 0; v < left; ++v)
        {
            if (((mask >> v) & 1) == 0)
            {
                continue;
            }
            smax(dp[mask], 1 + dp[mask & e[v]]);
        }
    }

    // если бы не было лефт и райт мы бы написали вот так
    // for (int mask = 0; mask < (1 << n); ++mask)
    // {
    // int ans = mask;
    // for (int v = 0; v < n; ++v)
    // {
    // if (((mask >> v) & 1) == 0)
    // {
    // continue;
    // }
    // ans &= (e[v] | (1 << v));
    // }
    // if (ans == mask)
    // {
    // mask - клика
    // }
    // }

    int maxClique = dp[(1 << left) - 1];
    // right part
    for (int mask = 1; mask < (1 << right); ++mask)
    {
        int ans = mask;
        int neighbours = (1 << left) - 1;
        for (int v = 0; v < right; ++v)
        {
            if (((mask >> v) & 1) == 0)
            {
                continue;
            }
            ans &= (e[v + left] | (1 << (v + left)));
            neighbours &= e[v + left];
        }
        if (ans == mask)
        {
            smax(maxClique, dp[neighbours] + __builtin_popcount(mask));
        }
    }
}

int main()
{
    run();
    return 0;
}