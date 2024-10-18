#include <bits/stdc++.h>
using namespace std;
int main()
{
    int n;
    cin >> n;
    vector<int> v(n);
    for (auto &x : v)
    {
        cin >> x;
    }
    int ans = 0, mx = 0;
    for (int i = v.size() - 1; i >= 0; i--)
    {
        if (v[i] > mx)
        {
            ans++;
            mx = v[i];
        }
    }
    cout << ans << endl;
    return 0;
}