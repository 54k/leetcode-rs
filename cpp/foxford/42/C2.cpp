#include <bits/stdc++.h>
using namespace std;

int n, m;
int a[100001];
int s[100001][20];
int lg[100001];

void init()
{

    lg[0] = -1;
    for (int i = 1; i <= n; ++i)
    {
        s[i][0] = a[i];
        lg[i] = lg[i / 2] + 1;
    }
    for (int j = 1; (1 << j) <= n; ++j)
    {
        for (int i = 1; i + (1 << j) - 1 <= n; ++i)
            s[i][j] = min(s[i][j - 1], s[i + (1 << (j - 1))][j - 1]);
    }
}

int query(int u, int v)
{
    if (u > v)
        swap(u, v);
    int k = lg[v - u + 1];
    return min(s[u][k], s[v - (1 << k) + 1][k]);
}

int main()
{
    cin >> n >> m;
    cin >> a[1];
    for (int i = 2; i <= n; ++i)
        a[i] = (23 * a[i - 1] + 21563) % 16714589;
    init();
    int u, v;
    cin >> u >> v;
    int ans = query(u, v);
    for (int i = 1; i < m; ++i)
    {
        u = (17 * u + 751 + ans + 2 * i) % n + 1;
        v = (13 * v + 593 + ans + 5 * i) % n + 1;
        ans = query(u, v);
    }
    cout << u << " " << v << " " << ans << endl;
}