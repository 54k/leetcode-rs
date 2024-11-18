#include <bits/stdc++.h>
using namespace std;

int main()
{
    int m, n;
    cin >> m >> n;
    vector<vector<int>> v(m, vector<int>(n));
    for (int i = 0; i < m; i++)
    {
        for (int j = 0; j < n; j++)
        {
            cin >> v[i][j];
        }
    }

    for (int i = 0; i < m; i++)
    {
        for (int j = 0; j < n; j++)
        {
            int sum = 0;
            for (auto &p :
                 vector<pair<int, int>>{{-1, 0}, {1, 0}, {0, 1}, {0, -1}})
            {
                int k = p.first, z = p.second;
                if (k == z)
                    continue;
                int ik = i + k, jz = j + z;
                if (0 <= ik && ik < m && 0 <= jz && jz < n)
                {
                    sum += v[ik][jz];
                }
            }
            cout << sum << " ";
        }
        cout << "\n";
    }

    return 0;
}