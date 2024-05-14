#include <bits/stdc++.h>
using namespace std;

int main()
{
    long long a, b, c, d;
    cin >> a >> b >> c >> d;
    if (a == 0)
    {
        cout << "1 " << c + 1;
    }
    else if (b == 0)
    {

        cout << "1 " << d + 1;
    }
    else if (c == 0)
    {
        cout << a + 1 << " 1";
    }
    else if (d == 0)
    {
        cout << b + 1 << " 1";
    }
    else
    {
        int shirt1 = a + 1, socks1 = c + 1;
        if (shirt1 + socks1 > b + 1 + d + 1)
        {
            shirt1 = b + 1;
            socks1 = d + 1;
        }
        int shirt2 = max(a, b) + 1, socks2 = 1;
        if (max(a, b) > max(c, d))
        {
            shirt2 = 1;
            socks2 = max(c, d) + 1;
        }
        if (shirt1 + socks1 < shirt2 + socks2)
        {
            cout << shirt1 << ' ' << socks1;
        }
        else
        {
            cout << shirt2 << ' ' << socks2;
        }
    }
}