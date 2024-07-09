#include <bits/stdc++.h>
using namespace std;

int main()
{
    int n, x;
    cin >> n >> x;
    x--;
    vector<vector<int>> mat(n, vector<int>(n, 0));
    for (int i = 0; i < n; i++)
    {
        for (int j = 0; j < n; j++)
        {
            cin >> mat[i][j];
        }
    }

    vector<int> dist(n, -1);
    dist[x] = 0;
    queue<int> q;
    q.push(x);

    while (!q.empty())
    {
        int t = q.front();
        q.pop();

        for (int i = 0; i < n; i++)
        {
            if (i != t && mat[t][i] > 0 && dist[i] == -1)
            {
                q.push(i);
                dist[i] = dist[t] + 1;
            }
        }
    }

    for (auto &d : dist)
    {
        cout << d << " ";
    }
    cout << endl;
}