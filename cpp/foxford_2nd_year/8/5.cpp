#include <bits/stdc++.h>
using namespace std;

bool is_prime(int num)
{
    for (int d = 2; d * d <= num; d++)
    {
        if (num % d == 0)
        {
            return false;
        }
    }
    return true;
}

int main()
{
    int n;
    cin >> n;
    if (n == 4)
    {
        cout << "2 2" << endl;
    }
    else
    {
        for (int d = 3; d <= n; d += 2)
        {
            if (is_prime(d) && is_prime(n - d))
            {
                cout << d << " " << (n - d) << endl;
                break;
            }
        }
    }
    return 0;
}