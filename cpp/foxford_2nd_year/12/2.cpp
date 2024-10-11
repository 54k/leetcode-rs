#include <bits/stdc++.h>
using namespace std;

int main()
{
    int x = 0, y = 0;

    vector<pair<int, int>> dirs = {{0, 1}, {1, 0}, {0, -1}, {-1, 0}};
    string dir;
    int n;
    while (cin >> dir)
    {
        pair<int, int> d;
        switch (dir[0])
        {
        case 'N':
            d = dirs[0];
            break;
        case 'E':
            d = dirs[1];
            break;
        case 'S':
            d = dirs[2];
            break;
        case 'W':
            d = dirs[3];
            break;
        default:
            break;
        }

        cin >> n;
        while (n-- > 0)
        {
            x += d.first;
            y += d.second;
        }
    }

    cout << x << " " << y << endl;
}