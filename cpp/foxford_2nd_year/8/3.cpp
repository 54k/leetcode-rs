#include <bits/stdc++.h>
using namespace std;

bool is_good(int num)
{
    for (int d = 2; d * d <= num; d++)
    {
        while (num % d == 0)
        {
            if (d != 2 && d != 5 && d != 7)
            {
                return false;
            }
            num /= d;
        }
    }
    if (num > 1 && num != 2 && num != 5 && num != 7)
    {
        // cout << num << " is not a valid number" << endl;
        return false;
    }
    return true;
}

int main()
{
    int prev = 2;
    int ans = 0;
    for (int i = 1; i <= 100; i++)
    {
        while (!is_good(prev))
        {
            prev++;
        }
        ans = prev;
        // cout << "prev: " << ans << endl;
        prev++;
    }
    cout << ans << endl;
    return 0;
}