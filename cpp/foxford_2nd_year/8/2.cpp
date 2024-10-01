#include <bits/stdc++.h>
using namespace std;

bool is_prime(int num)
{
    if (num == 1)
        return false;
    for (int d = 2; d * d <= num; d++)
    {
        if (num % d == 0)
            return false;
    }
    return true;
}

int main()
{
    int ans = 0;
    for (int i = 2; i <= 2022; i++)
    {
        int s = 0, num = i;
        while (num)
        {
            s += num % 10;
            num /= 10;
        }

        if (is_prime(s))
        {
            ans++;
        }
    }
    cout << ans << endl;
    return 0;
}