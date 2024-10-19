#include <bits/stdc++.h>
using namespace std;

#define ll long long

int main()
{
    int x, mx = -(1 << 30);
    while (cin >> x)
    {
        if (x % 10 == 7)
        {
            mx = max(mx, x);
        }
    }
    cout << mx << endl;
    return 0;
}