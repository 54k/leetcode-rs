#include <bits/stdc++.h>
using namespace std;

int hanoi(int n, int x, int y)
{
    if (n == 1)
    {
        return 1;
    }
    int ans = 1;
    ans += hanoi(n - 1, x, (6 - x - y));
    ans += hanoi(n - 1, (6 - x - y), y);
    return ans;
}

int main()
{
    int n;
    cin >> n;
    cout << hanoi(n, 1, 2) << endl;
    return 0;
}