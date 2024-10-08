#include <iostream>
#include <algorithm>
#include <vector>
using namespace std;
int main()
{
    int n, m, i, ans = 1e9;
    cin >> n >> m;
    vector<int> a(n), b(m);
    for (i = 0; i < n; ++i)
        cin >> a[i];
    for (i = 0; i < m; ++i)
        cin >> b[i];
    for (int x : a)
    {
        auto it = upper_bound(b.begin(), b.end(), x);
        if (it != b.end())
            ans = min(ans, abs(x - *it));
        if (it != b.begin())
            ans = min(ans, abs(x - *(it - 1)));
    }
    cout << ans << endl;
    return 0;
}