#include <bits/stdc++.h>
using namespace std;

int main()
{
    int m, n, k;
    cin >> m >> n >> k;
    vector<vector<char>> v(m, vector<char>(n, '0'));
    while (k-- > 0)
    {
        int r, c;
        cin >> r >> c;

        v[--r][--c] = '*';

        for (int i = -1; i < 2; i++)
        {
            for (int j = -1; j < 2; j++)
            {
                if (i == 0 && j == 0)
                {
                    continue;
                }
                int nr = r + i, nc = c + j;
                if (0 <= nr && nr < m && 0 <= nc && nc < n && v[nr][nc] != '*')
                {
                    v[nr][nc] = (v[nr][nc] - '0' + 1) + '0';
                }
            }
        }
    }

    for (int i = 0; i < m; i++)
    {
        for (int j = 0; j < n; j++)
        {
            cout << v[i][j] << " ";
        }
        cout << "\n";
    }
    return 0;
}