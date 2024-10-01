#include <iostream>

using namespace std;

typedef long long ll;

ll gcd(ll a, ll b)
{
    if (a == 0)
    {
        return b;
    }
    return gcd(b % a, a);
}

int main()
{
    ll a, b, c, d;
    cin >> a >> b >> c >> d;
    ll x = abs(c - a), y = abs(d - b);
    ll g = gcd(x, y);
    cout << g * (x / g + y / g - 1) << endl;
    return 0;
}