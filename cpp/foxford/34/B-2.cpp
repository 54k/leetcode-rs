#include <iostream>
#include <vector>
#include <queue>
using namespace std;
int sum_digits(int n)
{
    int res = 0;
    while (n > 0)
    {
        res += n % 10;
        n /= 10;
    }
    return res;
}
int main()
{
    int a, b;
    cin >> a >> b;
    vector<int> dist(10000, -1);
    dist[a] = 0;
    queue<int> q;
    q.push(a);
    while (!q.empty())
    {
        int u = q.front();
        q.pop();
        vector<int> next;
        next.push_back(u * 3);
        next.push_back(u - 2);
        next.push_back(u + sum_digits(u));
        for (auto v : next)
            if (v >= 0 && v <= 9999 && dist[v] == -1)
            {
                dist[v] = dist[u] + 1;
                q.push(v);
            }
    }
    cout << dist[b] << endl;
    return 0;
}