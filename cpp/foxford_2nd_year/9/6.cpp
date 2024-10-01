#include <iostream>

using namespace std;

typedef long long ll;

ll binpow(int x, int n, int p)
{
    if (n == 0)
    {
        return 1;
    }
    if (n == 1)
    {
        return x % p;
    }
    if (n % 2 == 0)
    {
        ll res = binpow(x, n / 2, p);
        return (res * res) % p;
    }
    else
    {
        return (binpow(x, n - 1, p) * x) % p;
    }
}

int main()
{
    ll x, N, P;
    cin >> x >> N >> P;
    cout << binpow(x, N, P) << endl;
    return 0;
}