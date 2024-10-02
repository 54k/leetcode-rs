#include <bits/stdc++.h>
using namespace std;

int main()
{
    int x, mx;
    vector<int> v;
    while (cin >> x)
    {
        v.emplace_back(x);
        if (x % 5 > 0)
        {
            mx = max(mx, x);
        }
    }
    for (auto &x : v)
    {
        if (x % 5 > 0)
        {
            x = mx;
        }
        cout << x << "\n";
    }
    cout << endl;
    return 0;;
}