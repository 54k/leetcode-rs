#include <bits/stdc++.h>
using namespace std;

int main()
{
    int n, ans = 0;
    cin >> n;
    while (n-- > 0)
    {
        int x;
        cin >> x;
        if (x == 0)
        {
            ans++;
        }
    }
    cout << ans << endl;
    return 0;
}